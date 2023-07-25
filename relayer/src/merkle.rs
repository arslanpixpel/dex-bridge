use anyhow::Context;
use concordium_rust_sdk::{
    cis2,
    types::{hashes::TransactionHash, ContractAddress},
};
use ethabi::{
    ethereum_types::{H160, H256, U256},
    RawLog, Token,
};
use ethers::{
    prelude::{
        types::transaction::eip2718::TypedTransaction, Middleware, Signer, SignerMiddleware,
    },
    utils::rlp::Rlp,
};
use rs_merkle::{Hasher, MerkleProof, MerkleTree};
use std::{collections::BTreeMap, sync::Arc};

use crate::{
    concordium_contracts::WithdrawEvent,
    db::{self, DatabaseOperation, MerkleUpdate, PendingEthereumTransactions},
    root_chain_manager::BridgeManager,
    state_sender,
};

pub fn make_proof<A: PartialEq + Eq, E>(
    leaves: impl IntoIterator<Item = Result<(A, [u8; 32]), E>>,
    elem: A,
) -> Result<Option<MerkleProof<Keccak256Algorithm>>, E> {
    let mut tree = MerkleTree::<Keccak256Algorithm>::new();
    let mut index = Vec::new();
    for (i, ok_or) in leaves.into_iter().enumerate() {
        let (v, leaf) = ok_or?;
        tree.insert(leaf);
        if v == elem {
            index.push(i)
        }
    }
    if !index.is_empty() {
        tree.commit();
        Ok(Some(tree.proof(&index)))
    } else {
        Ok(None)
    }
}

pub struct MerkleData {
    /// The child token address that is being withdrawn.
    pub child_token: ContractAddress,
    /// The amount of the token that is being withdrawn.
    pub amount: U256,
    /// The target address of the withdrawal. The Ethereum wallet.
    pub user_wallet: H160,
    /// The transaction hash
    pub transaction_hash: TransactionHash,
    /// Event id on Concordium.
    pub transaction_event_id: u64,
    /// Id of the token that is being withdrawn.
    pub token_id: u64,
}

impl MerkleData {
    pub fn encode(&self) -> Vec<u8> {
        let ccd_index = Token::Uint(U256::from(self.child_token.index));
        let ccd_sub_index = Token::Uint(U256::from(self.child_token.subindex));
        let amount = Token::Uint(self.amount);
        let withdraw_to_wallet = Token::Address(self.user_wallet);
        let transaction_hash = Token::FixedBytes(self.transaction_hash.as_ref().to_vec());
        let transaction_event_id = Token::Uint(U256::from(self.transaction_event_id));
        let token_id = Token::Uint(U256::from(self.token_id));

        ethabi::encode(&vec![
            ccd_index,
            ccd_sub_index,
            amount,
            withdraw_to_wallet,
            transaction_hash,
            transaction_event_id,
            token_id,
        ])
    }
}

#[derive(Clone)]
pub struct Keccak256Algorithm {}

impl rs_merkle::Hasher for Keccak256Algorithm {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        use sha3::Digest;
        sha3::Keccak256::digest(data).into()
    }

    // The OpenZeppelin contract computes the hash of inner nodes by ordering them
    // lexicographically first. So we must override the default implementation.
    fn concat_and_hash(left: &Self::Hash, right: Option<&Self::Hash>) -> Self::Hash {
        use sha3::Digest;
        let mut hasher = sha3::Keccak256::new();
        match right {
            Some(right_node) => {
                if left <= right_node {
                    hasher.update(left);
                    hasher.update(right_node);
                } else {
                    hasher.update(right_node);
                    hasher.update(left);
                }
                hasher.finalize().into()
            }
            None => *left,
        }
    }

    fn hash_size() -> usize {
        std::mem::size_of::<Self::Hash>()
    }
}

pub fn convert_from_token_amount(a: &cis2::TokenAmount) -> U256 {
    let le_bytes = a.0.to_bytes_le();
    U256::from_little_endian(&le_bytes)
}

pub fn build_merkle_tree(
    events: &[(TransactionHash, WithdrawEvent)],
) -> anyhow::Result<MerkleTree<Keccak256Algorithm>> {
    let mut tree = MerkleTree::new();
    for (transaction_hash, we) in events {
        let data = MerkleData {
            child_token: we.contract,
            amount: convert_from_token_amount(&we.amount),
            user_wallet: we.eth_address.into(),
            transaction_hash: *transaction_hash,
            transaction_event_id: we.event_index,
            token_id: u64::from_le_bytes(
                Vec::from(we.token_id.clone())
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("Invalid token id."))?,
            ),
        };
        let hash = Keccak256Algorithm::hash(&data.encode());
        tree.insert(hash);
    }
    tree.commit();
    Ok(tree)
}

pub struct MerkleSetterClient<M: ethers::providers::Middleware, S> {
    /// The client used for setting merkle roots.
    pub root_manager: BridgeManager<M>,
    pub signer_middleware: SignerMiddleware<Arc<M>, S>,
    pub signer: S,
    /// Maximum gas price allowed. If the current gas price is above this
    /// the sending will be skipped for this iteration.
    pub max_gas_price: U256,
    /// Maximum gas for sending the set merkle root transaction.
    pub max_gas: U256,
    /// Next nonce used for sending transactions. This is updated **after** a
    /// pending transaction is confirmed.
    pub next_nonce: U256,
    /// Minimum number of seconds between merkle root updates.
    pub update_interval: std::time::Duration,
    /// Interval when we escalate the transaction price.
    pub escalate_interval: std::time::Duration,
    /// Interval when we escalate the transaction price.
    pub warn_duration: std::time::Duration,
    /// List of event indices and the hashes
    pub current_leaves: Arc<std::sync::Mutex<BTreeMap<u64, [u8; 32]>>>,
    /// The high water mark. The last event that was set in the merkle root.
    /// This is used to skip sending updates when there are no new
    /// withdrawals to be approved.
    pub max_marked_event_index: Option<u64>,
}

pub fn make_event_leaf_hash(
    transaction_hash: TransactionHash,
    we: &WithdrawEvent,
) -> anyhow::Result<[u8; 32]> {
    let data = MerkleData {
        child_token: we.contract,
        amount: convert_from_token_amount(&we.amount),
        user_wallet: we.eth_address.into(),
        transaction_hash,
        transaction_event_id: we.event_index,
        token_id: u64::from_le_bytes(
            Vec::from(we.token_id.clone())
                .try_into()
                .map_err(|_| anyhow::anyhow!("Invalid token id."))?,
        ),
    };
    Ok(Keccak256Algorithm::hash(&data.encode()))
}

impl<M: ethers::providers::Middleware, S: Signer + Clone> MerkleSetterClient<M, S> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        root_manager: BridgeManager<M>,
        signer: S,
        max_gas_price: U256,
        max_gas: U256,
        next_nonce: U256,
        pending_merkle_set: &Option<db::PendingEthereumTransactions>,
        update_interval: std::time::Duration,
        pending_withdrawals: Vec<(TransactionHash, WithdrawEvent)>,
        max_marked_event_index: Option<u64>,
        escalate_interval: std::time::Duration,
        warn_duration: std::time::Duration,
    ) -> anyhow::Result<Self> {
        let next_nonce = if let Some(PendingEthereumTransactions { txs, .. }) = pending_merkle_set {
            let Some((tx_hash, data)) = txs.last() else {
                anyhow::bail!("Invariant violation. There are claimed pending transactions, but we cannot find them.");
            };
            let (tx, _) = ethers::types::transaction::eip2718::TypedTransaction::decode_signed(
                &Rlp::new(data),
            )?;
            log::debug!(
                "There is a pending Ethereum transaction with hash {:#x}. Using it's nonce as the \
                 next nonce.",
                tx_hash,
            );
            anyhow::ensure!(
                tx.from() == Some(&signer.address()),
                "Pending transaction is incorrectly signed."
            );
            *tx.nonce().context("Nonce must have been set.")?
        } else {
            next_nonce
        };
        let msc = Self {
            signer_middleware: SignerMiddleware::new(root_manager.client(), signer.clone()),
            root_manager,
            signer,
            max_gas_price,
            max_gas,
            next_nonce,
            update_interval,
            current_leaves: Arc::new(std::sync::Mutex::new(BTreeMap::new())),
            max_marked_event_index,
            escalate_interval,
            warn_duration,
        };
        for (tx_hash, we) in pending_withdrawals {
            let event_index = we.event_index;
            let merkle_event_hash = make_event_leaf_hash(tx_hash, &we)?;
            add_withdraw_event(&msc.current_leaves, event_index, merkle_event_hash)?;
        }
        Ok(msc)
    }
}

fn add_withdraw_event(
    leaves: &Arc<std::sync::Mutex<BTreeMap<u64, [u8; 32]>>>,
    event_index: u64,
    hash: [u8; 32],
) -> anyhow::Result<(Option<[u8; 32]>, usize)> {
    let mut lock = leaves
        .lock()
        .map_err(|_| anyhow::anyhow!("Unable to acquire lock."))?;
    let r = lock.insert(event_index, hash);
    Ok((r, lock.len()))
}

fn remove_withdraw_event(
    leaves: &Arc<std::sync::Mutex<BTreeMap<u64, [u8; 32]>>>,
    event_index: u64,
) -> anyhow::Result<(Option<[u8; 32]>, usize)> {
    let mut lock = leaves
        .lock()
        .map_err(|_| anyhow::anyhow!("Unable to acquire lock."))?;
    let r = lock.remove(&event_index);
    Ok((r, lock.len()))
}
pub enum SetMerkleRootResult {
    SetTransaction {
        tx_hash: H256,
        raw_tx: ethers::prelude::Bytes,
        tx: TypedTransaction,
        root: [u8; 32],
        ids: Arc<[u64]>,
    },
    GasTooHigh {
        max_gas_price: U256,
        current_gas_price: U256,
    },
    NoPendingWithdrawals,
}

#[derive(Debug, thiserror::Error)]
enum SetMerkleRootError<M: Middleware, S: Signer> {
    #[error("Error acquiring the lock: {0}")]
    LockError(anyhow::Error),
    #[error("Error querying Ethereum client: {0}")]
    Network(M::Error),
    #[error("Error signing transaction: {0}")]
    Signer(S::Error),
}

impl<M: Middleware, S: Signer> MerkleSetterClient<M, S>
where
    M::Error: 'static,
    S::Error: 'static,
{
    async fn set_merkle_root(&self) -> Result<SetMerkleRootResult, SetMerkleRootError<M, S>> {
        let mut tree = MerkleTree::<Keccak256Algorithm>::new();
        let ids = {
            let leaves = self.current_leaves.lock().map_err(|_| {
                SetMerkleRootError::LockError(anyhow::anyhow!(
                    "Unable to set merkle root, unable to acquire lock."
                ))
            })?;
            if leaves.last_key_value().map(|x| *x.0) <= self.max_marked_event_index {
                // Nothing to do.
                return Ok(SetMerkleRootResult::NoPendingWithdrawals);
            }
            leaves
                .iter()
                .map(|(&id, hash)| {
                    tree.insert(*hash);
                    id
                })
                .collect::<Arc<[_]>>()
        }; // drop lock.
        tree.commit();
        if let Some(new_root) = tree.root() {
            let client = self.root_manager.client();
            let current_gas_price = client
                .get_gas_price()
                .await
                .map_err(SetMerkleRootError::Network)?;
            log::debug!("Current gas price is {}.", current_gas_price);
            if current_gas_price <= self.max_gas_price {
                let call = self.root_manager.set_merkle_root(new_root);
                let tx = call.tx;
                let tx = ethers::types::transaction::eip2718::TypedTransaction::Legacy(
                    ethers::types::TransactionRequest {
                        from: tx.from().cloned(),
                        to: tx.to().cloned(),
                        gas: Some(self.max_gas),
                        gas_price: Some(current_gas_price),
                        value: tx.value().cloned(),
                        data: tx.data().cloned(),
                        nonce: Some(self.next_nonce),
                        chain_id: Some(self.signer.chain_id().into()),
                    },
                );
                // TODO: redesign here
                // legacy
                //     .set_chain_id(self.signer.chain_id())
                //     .set_nonce(self.next_nonce)
                //     .set_gas_price(current_gas_price)
                //     .set_gas(self.max_gas);
                let signature = self
                    .signer
                    .sign_transaction(&tx)
                    .await
                    .map_err(SetMerkleRootError::Signer)?;
                let tx_hash = tx.hash(&signature);
                let raw_tx = tx.rlp_signed(&signature);
                Ok(SetMerkleRootResult::SetTransaction {
                    tx_hash,
                    raw_tx,
                    tx,
                    root: new_root,
                    ids,
                })
            } else {
                Ok(SetMerkleRootResult::GasTooHigh {
                    max_gas_price: self.max_gas_price,
                    current_gas_price,
                })
            }
        } else {
            Ok(SetMerkleRootResult::NoPendingWithdrawals)
        }
    }
}

/// A task that will send Merkle root updates.
///
/// Upon start it will check if it needs to resubmit any pending transactions.
///
/// After that a background task [`ethereum_tx_sender`] is started that sends
/// updates to the Ethereum chain. See its documentation for details.
///
/// This worker instead monitors the provided `receiver` channel for new Merkle
/// tree updates to update its in-memory state.
pub async fn send_merkle_root_updates<M: Middleware + 'static, S: Signer + 'static>(
    metrics: crate::metrics::Metrics,
    client: MerkleSetterClient<M, S>,
    pending_merkle_set: Option<PendingEthereumTransactions>,
    mut receiver: tokio::sync::mpsc::Receiver<MerkleUpdate>,
    db_sender: tokio::sync::mpsc::Sender<DatabaseOperation>,
    num_confirmations: u64,
    mut stop: tokio::sync::watch::Receiver<()>,
) -> anyhow::Result<()>
where
    M::Error: 'static,
    S::Error: 'static,
{
    let mut pending = None;
    // Check if we need to resubmit the pending transaction.
    if let Some(PendingEthereumTransactions { txs, root, idxs }) = pending_merkle_set {
        let ethereum_client = client.root_manager.client();
        let mut _txs = vec![];
        for (mut tx_hash, raw_tx) in txs.into_iter().rev() {
            let status = ethereum_client
                .get_transaction(tx_hash)
                .await
                .context("Unable to get transaction status.")?;
            if status.is_none() {
                log::info!(
                    "Transaction with hash {tx_hash:#x} is in the database, but not known to the \
                     Ethereum chain. Submitting it."
                );
                let _pending_tx = ethereum_client
                    .send_raw_transaction(raw_tx.clone())
                    .await
                    .context("Unable to send raw transaction.")?;
                metrics.sent_ethereum_transactions.inc();
            }
            _txs.push((tx_hash, raw_tx))
        }
        pending = Some(EthereumPendingTransactions {
            root,
            ids: idxs,
            pending_txs: _txs,
        });
    }
    let leaves = client.current_leaves.clone();
    let sender_handle = tokio::spawn(ethereum_tx_sender(
        metrics.clone(),
        client,
        db_sender,
        pending,
        num_confirmations,
        stop.clone(),
    ));
    metrics
        .merkle_tree_size
        .set(leaves.lock().map_or(0, |x| x.len()) as i64);
    while let Some(mu) = tokio::select! {
        // Make sure to process all events that are in the queue before shutting down.
        // Thus prioritize getting things from the channel.
        // This only works in combination with the fact that we shut down senders
        // upon receving a kill signal, so the receiver will be drained eventually.
        biased;

        v = receiver.recv() => v,
        _ = stop.changed() => None,
    } {
        match mu {
            MerkleUpdate::NewWithdraws { withdraws } => {
                metrics.num_withdrawals.inc_by(withdraws.len() as u64);
                for (event_index, merkle_hash) in withdraws {
                    log::debug!("New withdraw event with index {event_index}.");
                    let (r, new_size) = add_withdraw_event(&leaves, event_index, merkle_hash)?;
                    metrics.merkle_tree_size.set(new_size as i64);
                    if r.is_some() {
                        metrics.warnings_total.inc();
                        log::warn!(
                            "Duplicate event index {event_index} added to the withdraw events."
                        );
                    }
                }
            }
            MerkleUpdate::WithdrawalCompleted {
                receiver: _,
                original_event_index,
            } => {
                metrics.num_completed_withdrawals.inc();
                let (r, new_size) = remove_withdraw_event(&leaves, original_event_index)?;
                metrics.merkle_tree_size.set(new_size as i64);
                if r.is_none() {
                    metrics.errors_total.inc();
                    log::error!(
                        "An event {original_event_index} marked as withdrawn, but was not known."
                    );
                }
            }
        }
    }
    sender_handle.await??;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum EthereumSenderError<M: Middleware> {
    #[error("A network error occurred: {0}")]
    Retryable(<M as Middleware>::Error),
    #[error("An internal parsing error occurred. Aborting: {0}.")]
    InternalABI(#[from] ethabi::Error),
    #[error("An internal error occurred. Aborting: {0}.")]
    Internal(anyhow::Error),
}

impl<S: Signer, M: Middleware> From<SetMerkleRootError<M, S>> for EthereumSenderError<M>
where
    S::Error: 'static,
{
    fn from(value: SetMerkleRootError<M, S>) -> Self {
        match value {
            SetMerkleRootError::LockError(e) => Self::Internal(e),
            SetMerkleRootError::Network(e) => Self::Retryable(e),
            // Signing is an offline process the way we do it. So if it fails some configuration is
            // wrong.
            SetMerkleRootError::Signer(e) => Self::Internal(e.into()),
        }
    }
}

struct EthereumPendingTransactions {
    /// The Merkle root to be set by all the pending transactions.
    root: [u8; 32],
    /// The event_indices that are marked approved by the transactions.
    ids: Arc<[u64]>,
    /// The list of transactions hashes and transactions.
    /// This list is never empty and is ordered by increasing gas price.
    pending_txs: Vec<(H256, ethers::prelude::Bytes)>,
}

async fn ethereum_tx_sender<M: Middleware, S: Signer>(
    metrics: crate::metrics::Metrics,
    mut client: MerkleSetterClient<M, S>,
    db_sender: tokio::sync::mpsc::Sender<DatabaseOperation>,
    mut pending: Option<EthereumPendingTransactions>,
    num_confirmations: u64,
    mut stop: tokio::sync::watch::Receiver<()>,
) -> anyhow::Result<()>
where
    M::Error: 'static,
    S::Error: 'static,
{
    while let Err(e) = ethereum_tx_sender_worker(
        &metrics,
        &mut client,
        &db_sender,
        &mut pending,
        num_confirmations,
        &mut stop,
    )
    .await
    {
        match e {
            EthereumSenderError::Retryable(e) => {
                metrics.warnings_total.inc();
                log::warn!(
                    "An error occurred when trying to send transactions to Ethereum or query the \
                     sent transaciton. Will attempt again in 10s: {e}"
                );
                // Just wait 10s and retry.
                let delay = std::time::Duration::from_secs(10);
                tokio::time::sleep(delay).await;
            }
            EthereumSenderError::InternalABI(e) => {
                metrics.errors_total.inc();
                log::error!(
                    "Unable to parse responses from Ethereum. This indicates a configuration \
                     error: {e:#}"
                );
                return Err(e.into());
            }
            EthereumSenderError::Internal(e) => {
                metrics.errors_total.inc();
                log::error!(
                    "An unrecoverable error occurred when sending transactions to Ethereum: {e:#}."
                );
                return Err(e);
            }
        }
    }
    Ok(())
}

/// The task that sends updates to the Ethereum chain.
/// - Check if there are any non-approved withdrawals, and if so makes a merkle
///   proof and submits it to the Ethereum chain. Before transaction submission
///   the transaction is stored in the database, and we mark the withdrawals
///   that are to be approved in the database as "tentatively approved".
/// - After that it waits until the transaction is confirmed on the chain. When
///   this is done, i.e., the transaction is confirmed, it uses the provided
///   `db_sender` channel to notify the database to mark the
///
/// The response is `Ok(())` if the service was asked to stop, otherwise it is
/// one of the errors if some part of the job was interrupted.
/// The client and `pending` are always left in a consistent state, so that a
/// retry can be made.
async fn ethereum_tx_sender_worker<M: Middleware, S: Signer>(
    metrics: &crate::metrics::Metrics,
    client: &mut MerkleSetterClient<M, S>,
    db_sender: &tokio::sync::mpsc::Sender<DatabaseOperation>,
    pending: &mut Option<EthereumPendingTransactions>,
    num_confirmations: u64,
    stop: &mut tokio::sync::watch::Receiver<()>,
) -> Result<(), EthereumSenderError<M>>
where
    M::Error: 'static,
    S::Error: 'static,
{
    let mut send_interval = tokio::time::interval_at(
        tokio::time::Instant::now() + client.update_interval,
        client.update_interval,
    );
    send_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    if db_sender
        .send(db::DatabaseOperation::SetNextMerkleUpdateTime {
            next_time: chrono::Utc::now()
                + chrono::Duration::from_std(send_interval.period())
                    .map_err(|e| EthereumSenderError::Internal(e.into()))?,
        })
        .await
        .is_err()
    {
        {
            log::debug!("The database has been shut down. Stopping the transaction sender.");
            return Ok(());
        }
    }
    'outer: loop {
        // Handle followup for any pending transaction first.
        let pending_result =
            wait_pending_ethereum_tx(metrics, client, db_sender, pending, num_confirmations, stop)
                .await?;
        let stop = match pending_result {
            WaitPendingResult::Stop => {
                // if told to stop then propagate.
                break 'outer;
            }
            WaitPendingResult::Ok => {
                // wait for next scheduled send if nothing is pending.
                tokio::select! {
                    _ = stop.changed() => true,
                    _ = send_interval.tick() => false,
                }
            }
            WaitPendingResult::Escalate => {
                // don't wait, immediately send an escalation transaction.
                false
            }
        };
        if stop {
            break 'outer;
        }
        // Now check if we have to send a new one
        let stop_loop = send_ethereum_tx(metrics, client, db_sender, pending).await?;
        if stop_loop {
            break 'outer;
        }
        // Record in the database for next time we are going to attempt an update.
        if db_sender
            .send(db::DatabaseOperation::SetNextMerkleUpdateTime {
                next_time: chrono::Utc::now()
                    + chrono::Duration::from_std(send_interval.period())
                        .map_err(|e| EthereumSenderError::Internal(e.into()))?,
            })
            .await
            .is_err()
        {
            {
                log::debug!("The database has been shut down. Stopping the transaction sender.");
                return Ok(());
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum WaitPendingResult {
    /// The service should stop.
    Stop,
    /// The job is completed, proceed normally.
    Ok,
    /// Timeout for escalation reached. Send a new transaction.
    Escalate,
}

/// Wait until the transaction is no longer pending.
/// Return whether the process should be stopped.
async fn wait_pending_ethereum_tx<M: Middleware, S: Signer>(
    metrics: &crate::metrics::Metrics,
    client: &mut MerkleSetterClient<M, S>,
    db_sender: &tokio::sync::mpsc::Sender<DatabaseOperation>,
    pending: &mut Option<EthereumPendingTransactions>,
    num_confirmations: u64,
    stop: &mut tokio::sync::watch::Receiver<()>,
) -> Result<WaitPendingResult, EthereumSenderError<M>>
where
    M::Error: 'static,
    S::Error: 'static,
{
    let mut check_interval = tokio::time::interval(std::time::Duration::from_secs(10));
    check_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    let start = tokio::time::Instant::now();
    'outer: while let Some(EthereumPendingTransactions {
        root,
        ids,
        pending_txs,
    }) = pending
    {
        let stop = tokio::select! {
            _ = stop.changed() => true,
            _ = check_interval.tick() => false,
        };
        if stop {
            return Ok(WaitPendingResult::Stop);
        }
        for (i, (pending_hash, _)) in pending_txs.iter().enumerate() {
            let elapsed = start.elapsed();
            if elapsed > client.escalate_interval {
                return Ok(WaitPendingResult::Escalate);
            } else if elapsed > client.warn_duration {
                metrics.warnings_total.inc();
                log::warn!(
                    "More than {}s elapsed waiting for {pending_hash:#x} to be confirmed.",
                    elapsed.as_secs()
                );
            }
            let result = client
                .root_manager
                .client()
                .get_transaction_receipt(*pending_hash)
                .await
                .map_err(EthereumSenderError::Retryable)?;
            let Some(receipt) = result else {
                log::debug!("Ethereum transaction {pending_hash:#x} has no receipt.");
                continue;
            };
            let Some(bn) = receipt.block_number else {
                return Err(EthereumSenderError::Internal(anyhow::anyhow!(
                    "A submitted transaction is confirmed, but without a block hash. This \
                     indicates a configuration error."
                )));
            };
            let current_block = client
                .root_manager
                .client()
                .get_block_number()
                .await
                .map_err(EthereumSenderError::Retryable)?;
            if bn.saturating_add(num_confirmations.into()) <= current_block {
                let mut found = false;
                for log in receipt.logs {
                    use ethers::contract::EthEvent;
                    let sig = state_sender::MerkleRootFilter::signature();
                    if log.topics.first().map_or(false, |s| s == &sig) {
                        let emitted_root = state_sender::MerkleRootFilter::decode_log(&RawLog {
                            topics: log.topics,
                            data: log.data.0.into(),
                        })?;
                        if emitted_root.root != *root {
                            return Err(EthereumSenderError::Internal(anyhow::anyhow!(
                                "Transaction {pending_hash:#x} emitted an incorrect Merkle root."
                            )));
                        }
                        found = true;
                    }
                }
                log::info!("Withdrawal transaction confirmed in block number: {bn}.");
                if !found {
                    metrics.errors_total.inc();
                    log::error!(
                        "A transaction with hash {pending_hash:#x} did not set a Merkle root. \
                         This means it failed."
                    )
                };
                let (response, receiver) = tokio::sync::oneshot::channel();
                let last_id = ids.last().copied();
                if db_sender
                    .send(DatabaseOperation::MarkSetMerkleCompleted {
                        root: *root,
                        ids: ids.clone(),
                        response,
                        success: found,
                        tx_hash: *pending_hash,
                        // Collect hashes of transactions that have not been confirmed to mark them
                        // as failed.
                        failed_hashes: pending_txs
                            .iter()
                            .enumerate()
                            .flat_map(|(j, (hash, _))| if i != j { Some(*hash) } else { None })
                            .collect(),
                    })
                    .await
                    .is_err()
                {
                    log::debug!(
                        "The database has been shut down. Stopping the transaction sender."
                    );
                    return Ok(WaitPendingResult::Stop);
                }
                metrics
                    .time_last_merkle_root
                    .set(chrono::Utc::now().timestamp());
                // Wait until the database operation completes.
                if receiver.await.is_err() {
                    metrics.warnings_total.inc();
                    log::warn!("The database has been shut down. Stopping the transaction sender.");
                    return Ok(WaitPendingResult::Stop);
                }
                if found {
                    log::info!(
                        "New merkle root set to {} and marked in the database.",
                        TransactionHash::from(*root)
                    );
                    // Assuming that the order is preserved by the channel.
                    // Mark the high watermark of processed ids.
                    client.max_marked_event_index = last_id;
                }
                // Transaction is confirmed, update the nonce for the next iteration
                // of sending.
                client.next_nonce += 1.into();
                // we have completed this nonce/root setting. Terminate normally.
                break 'outer;
            } else {
                log::debug!(
                    "Ethereum transaction {pending_hash:#x} is in block {bn}, but not yet \
                     confirmed."
                );
            }
        }
    }
    *pending = None;
    Ok(WaitPendingResult::Ok)
}

/// Send a transaction and store it in the `pending` value.
/// Return whether the process should be stopped.
async fn send_ethereum_tx<M: Middleware, S: Signer>(
    metrics: &crate::metrics::Metrics,
    client: &mut MerkleSetterClient<M, S>,
    db_sender: &tokio::sync::mpsc::Sender<DatabaseOperation>,
    pending: &mut Option<EthereumPendingTransactions>,
) -> Result<bool, EthereumSenderError<M>>
where
    M::Error: 'static,
    S::Error: 'static,
{
    // only send new transaction if there is no pending transaction. If there is a
    // pending transaction that means we need to escalate.
    let (tx_hash, raw_tx, ids, root) = if let Some(EthereumPendingTransactions {
        root,
        ids,
        pending_txs,
    }) = pending
    {
        let Some((_, tx)) = pending_txs.last() else
        {
            return Ok(false);
        };
        let (mut tx, _) =
            ethers::types::transaction::eip2718::TypedTransaction::decode_signed(&Rlp::new(tx))
                .map_err(|e| {
                    EthereumSenderError::Internal(anyhow::anyhow!(
                        "Error decoding pending transaction {e}"
                    ))
                })?;
        // Make sure the transaction was not tampered with.
        if tx.from() != Some(&client.signer.address()) {
            return Err(EthereumSenderError::Internal(anyhow::anyhow!(
                "The pending transaction is not correctly signed."
            )));
        }
        let Some(existing_gas_price) = tx.gas_price() else {
                return Err(EthereumSenderError::Internal(anyhow::anyhow!(
                    "Pending transaction with an unset gas price. That is a bug."
                )));
            };
        // Increase the gas price by 5%.
        let current_gas_price = client
            .root_manager
            .client()
            .get_gas_price()
            .await
            .map_err(EthereumSenderError::Retryable)?;
        let new_gas_price = std::cmp::max(
            existing_gas_price + existing_gas_price / 20,
            current_gas_price,
        );
        if new_gas_price > client.max_gas_price {
            metrics.warnings_total.inc();
            log::warn!(
                "Escalating would lead to transaction price that is too high {new_gas_price} > \
                 {}. Waiting for next iteration.",
                client.max_gas_price,
            );
            return Ok(false);
        } else {
            tx.set_gas_price(new_gas_price);
            let signature = client
                .signer
                .sign_transaction(&tx)
                .await
                .map_err(|e| EthereumSenderError::Internal(e.into()))?;
            let tx_hash = tx.hash(&signature);
            let raw_tx = tx.rlp_signed(&signature);
            log::debug!("Sending escalation SetMerkleRoot transaction with hash {tx_hash:#x}.");
            pending_txs.push((tx_hash, raw_tx.clone()));
            (tx_hash, raw_tx, ids.clone(), *root)
        }
    } else {
        match client.set_merkle_root().await? {
            SetMerkleRootResult::SetTransaction {
                tx_hash,
                raw_tx,
                tx: _,
                root,
                ids,
            } => {
                log::debug!(
                    "New merkle root to be set to {} using transaction {tx_hash:#x}.",
                    TransactionHash::from(root)
                );
                *pending = Some(EthereumPendingTransactions {
                    root,
                    ids: ids.clone(),
                    pending_txs: vec![(tx_hash, raw_tx.clone())],
                });
                (tx_hash, raw_tx, ids, root)
            }
            SetMerkleRootResult::GasTooHigh {
                max_gas_price,
                current_gas_price,
            } => {
                metrics.warnings_total.inc();
                log::warn!(
                    "Ethereum transaction price is too high {current_gas_price} > \
                     {max_gas_price}. Waiting for next iteration."
                );
                return Ok(false);
            }
            SetMerkleRootResult::NoPendingWithdrawals => {
                log::debug!("No pending withdrawals. Doing nothing.");
                return Ok(false);
            }
        }
    };
    // Now actually send the transaction
    let (response, receiver) = tokio::sync::oneshot::channel();
    if db_sender
        .send(DatabaseOperation::StoreEthereumTransaction {
            tx_hash,
            tx: raw_tx.clone(),
            response,
            ids,
            root,
        })
        .await
        .is_err()
    {
        log::info!("The database has been shut down. Stopping the transaction sender.");
        return Ok(true);
    }
    match receiver.await {
        Ok(x) => x,
        Err(_) => {
            metrics.warnings_total.inc();
            log::warn!("The database has been shut down. Stopping the transaction sender.");
            return Ok(true);
        }
    };
    let ethereum_client = client.root_manager.client();
    log::debug!(
        "Sending SetMerkleRoot transaction with hash {tx_hash:#x} to set Merkle root to {}.",
        TransactionHash::from(root)
    );
    let _pending_tx = ethereum_client
        .send_raw_transaction(raw_tx)
        .await
        .map_err(EthereumSenderError::Retryable)?;
    metrics.sent_ethereum_transactions.inc();
    Ok(false)
}
