//! This module deal with interaction with the bridge manager contract
//! on Concordium. It deals with parsing events emitted by the contract,
//! and sending updates to it.
use crate::db;
use anyhow::Context;
use concordium_rust_sdk::{
    cis2::{self, TokenId},
    common::types::{Amount, TransactionTime},
    endpoints::QueryError,
    id::types::AccountAddress,
    smart_contracts::common as contracts_common,
    types::{
        hashes::TransactionHash,
        queries::BlockInfo,
        smart_contracts::{
            ContractContext, InvokeContractResult, OwnedParameter, OwnedReceiveName,
        },
        transactions::{self, BlockItem, EncodedPayload, UpdateContractPayload},
        AbsoluteBlockHeight, Address, BlockItemSummary, ContractAddress, Energy, Nonce,
        RejectReason, WalletAccount,
    },
    v2::{self, BlockIdentifier},
};
use futures::{StreamExt, TryStreamExt};
use std::sync::Arc;

/// Type of Ethereum addresses.
type EthAddress = [u8; 20];

#[derive(contracts_common::Serialize, Debug)]
/// Mint new token in response to a deposit on Ethereum.
pub struct DepositOperation {
    /// Id of the event on Ethereum that emitted the deposit.
    pub id: u64,
    /// The address to which the deposit should be made.
    pub user: Address,
    /// Address of the root token to be mapped.
    pub root: EthAddress,
    /// Amount to be minted.
    pub amount: cis2::TokenAmount,
    /// Token ID of the Concordium token to be minted.
    pub token_id: TokenId,
}

#[derive(contracts_common::Serialize, Debug)]
pub struct TokenMapOperation {
    /// Id of the operation emitted by Ethereum StateSender.
    pub id: u64,
    /// Address of the origin token on Ethereum.
    pub root: EthAddress,
    /// Address of the mapped token on Concordium.
    pub child: ContractAddress,
}
#[derive(contracts_common::Serialize, Debug)]
/// State updates supported by the Bridge Manager contract.
pub enum StateUpdate {
    /// Deposit an amount of a mapped token.
    Deposit(DepositOperation),
    /// Add a new token mapping.
    TokenMap(TokenMapOperation),
}

#[derive(Debug, Clone)]
/// A wrapper around [`BridgeManagerClient`] that adds ability to send
/// transactions.
/// This structure maintains secret keys and the nonce.
///
/// The nonce is created when the client is created, and it is updated by
/// [`make_state_update_tx`](BridgeManager::make_state_update_tx).
/// This means that the [`BridgeManager`] assumes exclusive access to the
/// account.
pub struct BridgeManager {
    pub client: BridgeManagerClient,
    sender: std::sync::Arc<WalletAccount>,
    /// Maximum NRG allowed for state updates on Concordium.
    pub max_energy: Energy,
    /// Next nonce to be used for sending the transaction.
    next_nonce: Nonce,
}

/// Maximum allowed dry run energy.
const ALLOWED_DRY_RUN_NRG: Energy = Energy { energy: 1_000_000 };

// TODO: See how to keep this more easily in sync with the contracts.
const DUPLICATE_OPERATION: i32 = -10;

#[derive(Debug, Clone)]
/// Return value from dry-running a transaction.
pub enum DryRunReturn {
    /// Dry run succeeded with the given amount of used energy.
    Success {
        used_energy: Energy,
        /// Payload to be used for sending a smart contract update.
        payload: UpdateContractPayload,
    },
    /// The operation was already executed against the Concordium contract.
    DuplicateOperation,
    /// Another reason for failure.
    OtherError { reason: RejectReason },
}

impl BridgeManager {
    /// Construct a new [`Self`].
    ///
    /// If the `start_nonce` is not supplied it will be queried
    /// using [`get_next_account_sequence_number`](v2::Client::
    /// get_next_account_sequence_number). In such a case, if there are
    /// non-finalized transactions the invocation will fail.
    pub async fn new(
        mut client: BridgeManagerClient,
        sender: WalletAccount,
        start_nonce: Option<Nonce>,
        max_energy: Energy,
    ) -> anyhow::Result<Self> {
        let next_nonce = {
            if let Some(nonce) = start_nonce {
                nonce
            } else {
                let nonce = client
                    .client
                    .get_next_account_sequence_number(&sender.address)
                    .await?;
                // TODO: We could wait here to be sure instead of failing.
                anyhow::ensure!(nonce.all_final, "There are non-finalized transactions.");
                nonce.nonce
            }
        };
        Ok(Self {
            client,
            sender: Arc::new(sender),
            next_nonce,
            max_energy,
        })
    }

    /// Make the payload corresponding to the desired [`StateUpdate`].
    fn make_payload(&self, update: &StateUpdate) -> UpdateContractPayload {
        UpdateContractPayload {
            amount: Amount::from_micro_ccd(0),
            address: self.client.contract,
            receive_name: OwnedReceiveName::new_unchecked(
                "bridge-manager.receiveStateUpdate".into(),
            ),
            message: OwnedParameter::new_unchecked(contracts_common::to_bytes(update)),
        }
    }

    /// Check whether a given operation id has already been executed.
    pub async fn check_operation_used(
        &mut self,
        id: u64,
        bi: impl v2::IntoBlockIdentifier,
    ) -> anyhow::Result<bool> {
        let ctx = ContractContext {
            invoker: Some(self.sender.address.into()),
            contract: self.client.contract,
            amount: Amount::from_micro_ccd(0),
            method: OwnedReceiveName::new_unchecked("bridge-manager.checkOperationUsed".into()),
            parameter: OwnedParameter::new_unchecked(id.to_le_bytes().into()),
            energy: 10_000.into(),
        };
        let result = self.client.client.invoke_instance(bi, &ctx).await?;
        match result.response {
            InvokeContractResult::Success { return_value, .. } => {
                let rv = return_value.context("Unexpected response.")?.value;
                let Some(&first) = rv.first() else {
                    anyhow::bail!("Invocation returned no data.")
                };
                Ok(first == 1)
            }
            InvokeContractResult::Failure { .. } => {
                anyhow::bail!("Invocation failed.")
            }
        }
    }

    /// Dry run a state update transaction in the provided block.
    pub async fn dry_run_state_update(
        &mut self,
        update: &StateUpdate,
        bi: impl v2::IntoBlockIdentifier,
    ) -> anyhow::Result<DryRunReturn> {
        let payload = self.make_payload(update);
        log::info!("{:#?}", update);
        let ctx = ContractContext::new_from_payload(
            self.sender.address,
            ALLOWED_DRY_RUN_NRG,
            payload.clone(),
        );
        let result = self.client.client.invoke_instance(bi, &ctx).await?;
        match result.response {
            InvokeContractResult::Success { used_energy, .. } => Ok(DryRunReturn::Success {
                used_energy,
                payload,
            }),
            InvokeContractResult::Failure { reason, .. } => {
                if let RejectReason::RejectedReceive { reject_reason, .. } = reason {
                    if reject_reason == DUPLICATE_OPERATION {
                        Ok(DryRunReturn::DuplicateOperation)
                    } else {
                        Ok(DryRunReturn::OtherError { reason })
                    }
                } else {
                    Ok(DryRunReturn::OtherError { reason })
                }
            }
        }
    }

    /// Construct a update transaction that can be sent to execute the provided
    /// [`StateUpdate`]. The expiry time of the transaction is set for 1d.
    ///
    /// This function will dry run the transaction first. If the tranasction is
    /// rejected because the action has already been processed on the chain then
    /// `Ok(None)` is returned. If the transaction fails for some other reason
    /// then it is retried, and if it is not successful after 6 attempts
    /// an error is returned.
    ///
    /// Dry run is attempted in the last finalized block.
    ///
    /// This **does not** send the transaction, but does dry run the update.
    pub async fn make_state_update_tx(
        &mut self,
        update: &StateUpdate,
    ) -> anyhow::Result<Option<BlockItem<EncodedPayload>>> {
        let mut iter_num = 0;
        let (mut execution_energy, payload) = loop {
            match self
                .dry_run_state_update(update, BlockIdentifier::LastFinal)
                .await
            {
                Ok(v) => match v {
                    DryRunReturn::Success {
                        used_energy,
                        payload,
                    } => break (used_energy, payload),
                    DryRunReturn::DuplicateOperation => {
                        return Ok(None);
                    }
                    DryRunReturn::OtherError { reason } => {
                        log::error!(
                            "Unexpected response from dry running state update. This is a \
                             configuration error: {reason:#?}"
                        );
                        anyhow::bail!(
                            "Unexpected response from dry running state update. This is a \
                             configuration error: {reason:#?}"
                        );
                    }
                },
                Err(e) => {
                    log::warn!("Unable to dry run state update due to: {e:#}");
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(1000 << iter_num)).await;
            iter_num += 1;
            anyhow::ensure!(
                iter_num <= 6,
                "Too many retries trying to run state update."
            );
        };
        // Add an extra 1000 NRG to prevent race conditions in case the cost changes
        // slightly due to withdrawals.
        execution_energy = execution_energy.energy.saturating_add(1000).into();
        anyhow::ensure!(
            execution_energy <= self.max_energy,
            "Estimated energy exceeds maximum allowed"
        );
        // Set 1d expiry.
        let expiry: TransactionTime =
            TransactionTime::from_seconds((chrono::Utc::now().timestamp() + 24 * 60 * 60) as u64);
        let nonce = self.next_nonce;
        // increase the nonce.
        self.next_nonce.next_mut();

        let tx = transactions::send::update_contract(
            &*self.sender,
            self.sender.address,
            nonce,
            expiry,
            payload,
            execution_energy,
        );
        Ok(Some(tx.into()))
    }
}

#[derive(contracts_common::Serialize, PartialEq, Eq, Debug, Copy, Clone)]
/// Roles that can be set for the `BridgeManager` contracg
pub enum Roles {
    /// Admin, can set other roles, including admins.
    Admin,
    /// Mapper can add and remove token mappings.
    Mapper,
    /// `StateSyncher` can send deposits. This role is played by the relayer.
    StateSyncer,
}

#[derive(Debug, PartialEq, Eq, contracts_common::Serialize)]
/// A new token was mapped.
pub struct TokenMapEvent {
    /// Id of the operation emitted by Ethereum. Used to deduplicate them.
    pub id: u64,
    /// Address of the original token on Ethereum.
    pub root: EthAddress,
    /// Address of the mapped token on Concordium.
    pub child: ContractAddress,
}

#[derive(Debug, PartialEq, Eq, contracts_common::Serialize)]
pub struct DepositEvent {
    /// Id of the operation emitted by Ethereum. Used to deduplicate them.
    pub id: u64,
    /// Address of the child token that is to be minted.
    pub contract: ContractAddress,
    /// Amount to be minted.
    pub amount: cis2::TokenAmount,
    /// Id of the token on Concordium.
    pub token_id: TokenId,
}

#[derive(Debug, PartialEq, Eq, contracts_common::Serialize)]
pub struct WithdrawEvent {
    /// Index of the event emitted by the BridgeManager.
    pub event_index: u64,
    /// Address of the child token that is to be withdrawn.
    pub contract: ContractAddress,
    /// Amount to be withdrawn.
    pub amount: cis2::TokenAmount,
    /// Address that originated the withdrawal.
    pub ccd_address: Address,
    /// The recepient of the withdrawal on Ethereum.
    pub eth_address: EthAddress,
    /// Id of the token on Concordium.
    pub token_id: TokenId,
}

#[derive(Debug, PartialEq, Eq, contracts_common::Serialize)]
/// An event emitted when a role has been granted.
pub struct GrantRoleEvent {
    /// Address that has been granted the role.
    address: Address,
    /// The role that has been granted.
    role: Roles,
}

#[derive(Debug, PartialEq, Eq, contracts_common::Serialize)]
/// An event emitted when a role has been revoked/removed.
pub struct RevokeRoleEvent {
    /// Address that has been revoked the role
    address: Address,
    /// The role that has been revoked.
    role: Roles,
}

#[derive(Debug, PartialEq, Eq, contracts_common::Serialize)]
/// All possible events emitted by the bridge.
pub enum BridgeEvent {
    TokenMap(TokenMapEvent),
    Deposit(DepositEvent),
    Withdraw(WithdrawEvent),
    GrantRole(GrantRoleEvent),
    RevokeRole(RevokeRoleEvent),
}

impl BridgeEvent {
    /// Extract an event index if possible. Event index
    /// is only emitted by [`WithdrawEvent`].
    pub fn event_index(&self) -> Option<u64> {
        match self {
            BridgeEvent::TokenMap(_) => None,
            BridgeEvent::Deposit(_) => None,
            BridgeEvent::Withdraw(we) => Some(we.event_index),
            BridgeEvent::GrantRole(_) => None,
            BridgeEvent::RevokeRole(_) => None,
        }
    }
}

#[derive(Clone, Debug)]
/// A client for querying and looking at events of the bridge manager contract.
pub struct BridgeManagerClient {
    pub client: v2::Client,
    pub sender_account: AccountAddress,
    contract: ContractAddress,
}

impl BridgeManagerClient {
    pub fn new(
        client: v2::Client,
        sender_account: AccountAddress,
        contract: ContractAddress,
    ) -> Self {
        Self {
            client,
            sender_account,
            contract,
        }
    }

    /// Get all the bridge manager event logs.
    pub fn extract_events(
        &mut self,
        summary: &BlockItemSummary,
    ) -> anyhow::Result<Vec<BridgeEvent>> {
        if let Some(logs) = summary.contract_update_logs() {
            let mut out = Vec::new();
            for (ca, section_logs) in logs {
                // Only process logs from the bridge manager contract. Ignore the rest.
                if ca == self.contract {
                    for log in section_logs {
                        // Parsing should never fail. If it does that indicates a configuration
                        // error.
                        let event = contracts_common::from_bytes(log.as_ref())?;
                        out.push(event)
                    }
                }
            }
            Ok(out)
        } else {
            Ok(Vec::new())
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NodeError {
    /// No finalization in some time.
    #[error("Timeout.")]
    Timeout,
    /// Query error.
    #[error("Error querying the node {0}.")]
    QueryError(#[from] v2::QueryError),
    /// Internal error. This is a configuration issue.
    #[error("Internal error: {0}.")]
    Internal(anyhow::Error),
}

pub async fn listen_concordium(
    metrics: crate::metrics::Metrics,
    // The client used to query the chain.
    mut bridge_manager: BridgeManagerClient,
    // A channel used to insert into the database.
    sender: tokio::sync::mpsc::Sender<db::DatabaseOperation>,
    // Height at which to start querying.
    mut height: AbsoluteBlockHeight, // start height
    // Maximum number of parallel queries to make. This speeds up initial catchup.
    max_parallel: u32,
    // Maximum number of seconds to wait for a new finalized block.
    max_behind: u32,
) -> anyhow::Result<()> {
    let mut retry_attempt = 0;
    let mut last_height = height;
    loop {
        let res = listen_concordium_worker(
            &metrics,
            &mut bridge_manager,
            &sender,
            &mut height,
            max_parallel,
            max_behind,
        )
        .await;

        // If the last query did something clear the retry counter.
        if height > last_height {
            last_height = height;
            retry_attempt = 0;
        }
        match res {
            Ok(()) => {
                log::info!("Terminated listening for new Concordium events.");
                return Ok(());
            }
            Err(e) => match e {
                NodeError::Timeout => {
                    retry_attempt += 1;
                    if retry_attempt > 6 {
                        log::error!("Too many failures attempting to reconnect. Aborting.");
                        anyhow::bail!("Too many failures attempting to reconnect. Aborting.");
                    }
                    let delay = std::time::Duration::from_secs(5 << retry_attempt);
                    log::warn!(
                        "Querying the node timed out. Will attempt again in {} seconds..",
                        delay.as_secs()
                    );
                    tokio::time::sleep(delay).await;
                }
                NodeError::QueryError(e) => {
                    retry_attempt += 1;
                    if retry_attempt > 6 {
                        log::error!("Too many failures attempting to reconnect. Aborting.");
                        anyhow::bail!("Too many failures attempting to reconnect. Aborting.");
                    }
                    let delay = std::time::Duration::from_secs(5 << retry_attempt);
                    log::warn!(
                        "Querying the node failed due to {:#}. Will attempt again in {} seconds.",
                        e,
                        delay.as_secs()
                    );
                    tokio::time::sleep(delay).await;
                }
                NodeError::Internal(e) => {
                    log::error!("Internal configuration error: {e}. Terminating the query task.");
                    return Err(e);
                }
            },
        };
    }
}

/// Return Err if querying the node failed.
/// Return Ok(()) if the channel to the database was closed.
async fn listen_concordium_worker(
    metrics: &crate::metrics::Metrics,
    // The client used to query the chain.
    bridge_manager: &mut BridgeManagerClient,
    // A channel used to insert into the database.
    sender: &tokio::sync::mpsc::Sender<db::DatabaseOperation>,
    // Height at which to start querying.
    height: &mut AbsoluteBlockHeight, // start height
    // Maximum number of parallel queries to make. This speeds up initial catchup.
    max_parallel: u32,
    // Maximum number of seconds to wait for a new finalized block.
    max_behind: u32,
) -> Result<(), NodeError> {
    let mut finalized_blocks = bridge_manager
        .client
        .get_finalized_blocks_from(*height)
        .await?;
    let timeout = std::time::Duration::from_secs(max_behind.into());
    loop {
        let (error, chunk) = finalized_blocks
            .next_chunk_timeout(max_parallel as usize, timeout)
            .await
            .map_err(|_| NodeError::Timeout)?;
        let mut futures = futures::stream::FuturesOrdered::new();
        for fb in chunk {
            let mut node = bridge_manager.client.clone();
            // A future to query the block at the given hash.
            let poller = async move {
                let binfo = node.get_block_info(fb.block_hash).await?;
                let events = if binfo.response.transaction_count == 0 {
                    Vec::new()
                } else {
                    node.get_block_transaction_events(fb.block_hash)
                        .await?
                        .response
                        .try_collect()
                        .await?
                };
                Ok::<(BlockInfo, Vec<BlockItemSummary>), QueryError>((binfo.response, events))
            };
            futures.push_back(poller);
        }

        while let Some(result) = futures.next().await {
            let (block, summaries) = result?;
            log::debug!(
                "Processing Concordium block {} at height {}",
                block.block_hash,
                block.block_height
            );
            let mut transaction_events = Vec::new();
            for summary in summaries {
                let events = bridge_manager
                    .extract_events(&summary)
                    .map_err(NodeError::Internal)?;
                if !events.is_empty() {
                    transaction_events.push((summary.hash, events));
                }
                // Also check for any other transactions from the sender account.
                // So we can mark transactions we have sent as failed.
                if summary.is_rejected_account_transaction().is_some() {
                    if let Some(acc) = summary.sender_account() {
                        if acc.is_alias(&bridge_manager.sender_account) {
                            log::warn!(
                                "Discovered a failed transaction {} sent by Concordium relayer \
                                 account.",
                                summary.hash
                            );
                            if sender
                                .send(db::DatabaseOperation::MarkConcordiumTransaction {
                                    tx_hash: summary.hash,
                                    state: db::TransactionStatus::Failed,
                                })
                                .await
                                .is_err()
                            {
                                log::info!("The channel to the database writer has been closed.");
                                return Ok(());
                            }
                        }
                    }
                }
            }
            metrics
                .concordium_height
                .set(block.block_height.height as i64);
            if sender
                .send(db::DatabaseOperation::ConcordiumEvents {
                    block,
                    transaction_events,
                })
                .await
                .is_err()
            {
                log::info!("The channel to the database writer has been closed.");
                return Ok(());
            }
            *height = height.next();
        }
        if error {
            // we have processed the blocks we can, but further queries on the same stream
            // will fail since the stream signalled an error.
            return Err(NodeError::QueryError(v2::QueryError::RPCError(
                v2::Status::unavailable("No more blocks are available to process.").into(),
            )));
        }
    }
}

/// A worker that sends transactions to the Concordium node.
///
/// The transactions in the channel should be in increasing order of nonces,
/// otherwise sending will fail.
pub async fn concordium_tx_sender(
    metrics: crate::metrics::Metrics,
    mut client: v2::Client,
    mut receiver: tokio::sync::mpsc::Receiver<BlockItem<EncodedPayload>>,
    // Flag to signal stopping the task gracefully.
    mut stop: tokio::sync::watch::Receiver<()>,
) -> anyhow::Result<()> {
    // Process the response.
    // Return an error if submitting this transaction failed and this cannot be
    // recovered.
    //
    // Otherwise either return Ok(true) if retry should be attempted, or Ok(false).
    // if submission succeeded.
    let process_response = |hash, response: v2::RPCResult<TransactionHash>| match response {
        Ok(hash) => {
            log::info!("Transaction {hash} sent to the Concordium node.");
            metrics.sent_concordium_transactions.inc();
            Ok(false)
        }
        Err(e) => {
            if e.is_duplicate() {
                metrics.warnings_total.inc();
                log::warn!("Transaction {hash} already exists at the node.");
                Ok(false)
            } else if e.is_invalid_argument() {
                metrics.errors_total.inc();
                log::error!(
                    "Transaction {hash} is not valid for the current state of the node: {e:#}. \
                     Aborting."
                );
                anyhow::bail!(
                    "Transaction {hash} is not valid for the current state of the node: {e:#}. \
                     Aborting.",
                )
            } else {
                metrics.warnings_total.inc();
                log::warn!("Sending transaction to Concordium failed due to {e:#}. Will retry.");
                Ok(true)
            }
        }
    };

    'outer: while let Some(bi) = tokio::select! {
        // Make sure to process all events that are in the queue before shutting down.
        // Thus prioritize getting things from the channel.
        // This only works in combination with the fact that we shut down senders
        // upon receving a kill signal, so the receiver will be drained eventually.
        biased;
            x = receiver.recv() => x,
            _ = stop.changed() => None,
    } {
        let hash = bi.hash();
        let retry = process_response(hash, client.send_block_item(&bi).await)?;
        if retry {
            // Retry at most 5 times, waiting at most 32 * 5 = 160s
            let mut success = false;
            for i in 0..6 {
                // if the stop sender has been dropped we should terminate since that
                // should be the last thing that happens in the program.
                if stop.has_changed().unwrap_or(true) {
                    break 'outer;
                }
                let delay = std::time::Duration::from_secs(5 << i);
                metrics.warnings_total.inc();
                log::warn!(
                    "Waiting for {} seconds before resubmitting {hash}.",
                    delay.as_secs()
                );
                tokio::time::sleep(delay).await;
                let retry = process_response(hash, client.send_block_item(&bi).await)?;
                if !retry {
                    success = true;
                    break;
                }
            }
            anyhow::ensure!(success, "Unable to reconnect in 6 attempts.");
        }
    }
    log::info!("Concordium transaction sender terminated.");
    Ok(())
}
