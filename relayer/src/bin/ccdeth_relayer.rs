use anyhow::Context;
use ccdeth_relayer::{
    concordium_contracts::{self, BridgeManagerClient},
    db::{self, Database},
    ethereum,
    merkle::{self, MerkleSetterClient},
    state_sender::StateSender,
};
use clap::Parser;
use concordium::{
    id::types::AccountAddress,
    smart_contracts::common::Amount,
    types::{AbsoluteBlockHeight, ContractAddress, WalletAccount},
    v2::{self, BlockIdentifier},
};
use concordium_rust_sdk as concordium;
use ethabi::ethereum_types::U256;
use ethers::prelude::{
    Http, HttpRateLimitRetryPolicy, LocalWallet, Middleware, Provider, RetryClient, Signer,
};
use futures::StreamExt;
use std::{path::PathBuf, sync::Arc};
use tonic::transport::ClientTlsConfig;

#[derive(Parser, Debug)]
struct EthereumConfig {
    #[clap(
        long = "state-sender-address",
        help = "Address of the StateSender proxy instance on Ethereum.",
        env = "ETHCCD_RELAYER_STATE_SENDER_PROXY"
    )]
    state_sender: ethers::core::types::Address,
    #[clap(
        long = "root-chain-manager-address",
        help = "Address of the RootChainManager proxy instance on Ethereum.",
        env = "ETHCCD_RELAYER_ROOT_CHAIN_MANAGER_PROXY"
    )]
    root_chain_manager: ethers::core::types::Address,
    #[clap(
        long = "state-sender-creation-height",
        help = "Block number when the state sender instance was created. This is used as a \
                starting point for monitoring the Ethereum chain.",
        env = "ETHCCD_RELAYER_STATE_SENDER_CREATION_BLOCK_NUMBER"
    )]
    state_sender_creation_block_number: u64,
    #[clap(
        long = "ethereum-api",
        name = "ethereum-api",
        help = "JSON-RPC interface of an Ethereum node. Only HTTPS is supported as transport.",
        env = "ETHCCD_RELAYER_ETHEREUM_API"
    )]
    api: url::Url,
    // Maximum gas price.
    #[clap(
        long,
        env = "ETHCCD_RELAYER_MAX_GAS_PRICE",
        help = "Maximum gas price allowed for Ethereum transactions. If the current gas price is higher then the Merkle updates will be skipped.",
        value_parser = U256::from_dec_str,
        default_value = "1000000000",
    )]
    max_gas_price: U256,
    // Maximum gas for setting merkle roots.
    #[clap(long,
           help = "Maximum gas allowed for setting the Merkle root on Ethereum.",
           env = "ETHCCD_RELAYER_MAX_GAS",
           value_parser = U256::from_dec_str,
           default_value = "100000")]
    max_gas: U256,
    // Maximum gas for setting merkle roots.
    #[clap(
        long,
        help = "How often to approve new withdrawals on Ethereum.",
        env = "ETHCCD_RELAYER_MERKLE_UPDATE_INTERVAL",
        default_value = "600"
    )]
    merkle_update_interval: u64,
    /// Chain ID for the Ethereum network.
    #[clap(
        long,
        help = "Chain ID. Goerli is 5, mainnet is 1.",
        env = "ETHCCD_RELAYER_CHAIN_ID"
    )]
    chain_id: u64,
    /// Number of confirmations required on Ethereum before considering
    /// the transaction as "final".
    #[clap(
        long,
        help = "Number of confirmations required on Ethereum before considering the transaction \
                as final.",
        env = "ETHCCD_RELAYER_NUM_CONFIRMATIONS",
        default_value = "10"
    )]
    num_confirmations: u64,
    /// Request timeout for Ethereum node requests.
    #[clap(
        long,
        help = "Timeout for requests to the Ethereum node.",
        env = "ETHCCD_RELAYER_ETHEREUM_REQUEST_TIMEOUT",
        default_value = "10"
    )]
    ethereum_request_timeout: u64,
    #[clap(
        long,
        help = "Interval (in seconds) on when to escalate the price of the transaction.",
        env = "ETHCCD_RELAYER_MERKLE_ESCALATION_INTERVAL",
        default_value = "120"
    )]
    escalation_interval: u64,
    #[clap(
        long,
        help = "When to start warning that the transaction has not yet been confirmed.",
        env = "ETHCCD_RELAYER_MERKLE_WARN_DURATION",
        default_value = "120"
    )]
    warn_duration: u64,
    #[clap(
        long = "eth-min-balance",
        name = "eth-min-balance",
        help = "Minimum balance of the Ethereum account. In microEther",
        env = "ETHCCD_RELAYER_MIN_ETHEREUM_BALANCE"
    )]
    min_balance: u128,
}

impl EthereumConfig {
    fn log(&self) {
        // Do not log the API since that can be sensitive.
        let EthereumConfig {
            state_sender,
            root_chain_manager,
            state_sender_creation_block_number,
            api: _,
            max_gas_price,
            max_gas,
            merkle_update_interval,
            chain_id,
            num_confirmations,
            ethereum_request_timeout,
            escalation_interval,
            warn_duration,
            min_balance,
        } = self;
        log::info!("Using {state_sender:#x} as the state sender address.");
        log::info!("Using {root_chain_manager:#x} as the root chain manager address.");
        log::info!(
            "Using {state_sender_creation_block_number} as the starting height on Ethereum."
        );
        log::info!("Using {max_gas_price} as the maximum gas price.");
        log::info!("Using {max_gas} as the maximum allowed gas for transactions.");
        log::info!("Using {merkle_update_interval}s as the update interval for Merkle roots.");
        log::info!("Using {chain_id} as the chain id.");
        log::info!("Requiring {num_confirmations} confirmations for transactions on Ethereum.");
        log::info!("Using {ethereum_request_timeout}s as the request timeout for Ethereum API.");
        log::info!("Will escalate price every {escalation_interval}s.");
        log::info!("Will warn after transaction is not confirmed after {warn_duration}s.");
        log::info!("Requiring {min_balance} microETH on the sender account balance.");
    }
}

#[derive(Debug, Parser)]
struct ConcordiumConfig {
    #[clap(
        long = "concordium-api",
        name = "concordium-api",
        help = "GRPC V2 interface of the Concordium node.",
        env = "ETHCCD_RELAYER_CONCORDIUM_API",
        default_value = "http://localhost:20000"
    )]
    api: v2::Endpoint,
    #[clap(
        long = "concordium-max-parallel",
        help = "Maximum number of parallel queries of the Concordium node. This is only useful in \
                initial catchup if the relayer is started a long time after the bridge contracts \
                are in operation.",
        env = "ETHCCD_RELAYER_MAX_PARALLEL_QUERIES_CONCORDIUM",
        default_value = "1"
    )]
    max_parallel: u32,
    // Maximum number of seconds a concordium node can be behind before it is deemed "behind".
    #[clap(
        long = "concordium-max-behind",
        help = "Maximum number of seconds the Concordium node's last finalized block can be \
                behind before we log warnings.",
        env = "ETHCCD_RELAYER_CONCORDIUM_MAX_BEHIND",
        default_value = "240"
    )]
    max_behind: u32,
    /// Request timeout for Concordium node requests.
    #[clap(
        long,
        help = "Timeout for requests to the Concordium node.",
        env = "ETHCCD_RELAYER_CONCORDIUM_REQUEST_TIMEOUT",
        default_value = "10"
    )]
    request_timeout: u64,
    #[clap(
        long = "bridge-manager-address",
        help = "Address of the BridgeManger contract instance on Concordium.",
        env = "ETHCCD_RELAYER_BRIDGE_MANAGER"
    )]
    bridge_manager: ContractAddress,
    #[clap(
        long = "max-energy",
        help = "Maximum energy to allow for transactions on Concordium.",
        default_value = "100000",
        env = "ETHCCD_RELAYER_CONCORDIUM_MAX_ENERGY"
    )]
    max_energy: concordium::types::Energy,
    #[clap(
        long = "ccd-min-balance",
        name = "ccd-min-balance",
        help = "Minimum balance of the Concordium account. In microCCD",
        env = "ETHCCD_RELAYER_MIN_CONCORDIUM_BALANCE"
    )]
    min_balance: u64,
}

impl ConcordiumConfig {
    fn log(&self) {
        let ConcordiumConfig {
            api,
            max_parallel,
            max_behind,
            request_timeout,
            bridge_manager,
            max_energy,
            min_balance,
        } = self;
        log::info!("Using Concordium node at {}", api.uri());
        log::info!("Allowing up to {max_parallel} parallel queries of the Concordium node.");
        log::info!("Allowing the Concordium node to be at most {max_behind}s behind present.");
        log::info!("Using {request_timeout}s as the request timeout for Concordium.");
        log::info!("Using {bridge_manager} as bridge manager.");
        log::info!("Allowing up to {max_energy}NRG for Concordium tranasactions.");
        log::info!("Requiring  {min_balance} microCCD on the Concordium sender account.");
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Relayer {
    #[clap(
        long = "log-level",
        default_value = "info",
        help = "Maximum log level.",
        env = "ETHCCD_RELAYER_LOG_LEVEL"
    )]
    log_level: log::LevelFilter,
    #[clap(flatten)]
    ethereum_config: EthereumConfig,
    #[clap(flatten)]
    concordium_config: ConcordiumConfig,
    #[clap(
        long = "concordium-wallet-file",
        name = "concordium-wallet-file",
        help = "File with the Concordium wallet in the browser extension wallet export format.",
        env = "ETHCCD_RELAYER_CONCORDIUM_WALLET_FILE",
        conflicts_with = "concordium-wallet-secret-name"
    )]
    concordium_wallet: Option<PathBuf>,
    #[clap(
        long = "concordium-wallet-secret-name",
        name = "concordium-wallet-secret-name",
        help = "File with the Concordium wallet in the browser extension wallet export format.",
        env = "ETHCCD_RELAYER_CONCORDIUM_WALLET_SECRET_NAME",
        conflicts_with = "concordium-wallet-file"
    )]
    concordium_wallet_secret_name: Option<String>,
    #[clap(
        long = "eth-private-key",
        name = "eth-private-key",
        help = "Private key used to sign Merkle update tranasctions on Ethereum. The address \
                derived from this key must have the MERKLE_UPDATER role.",
        env = "ETHCCD_RELAYER_ETH_PRIVATE_KEY"
    )]
    eth_private_key: Option<LocalWallet>,
    #[clap(
        long = "eth-key-secret-name",
        name = "eth-key-secret-name",
        help = "Secret name of the key stored in Amazon secret manager.",
        env = "ETHCCD_RELAYER_ETH_PRIVATE_KEY_SECRET_NAME",
        conflicts_with = "eth-private-key"
    )]
    eth_private_key_secret_name: Option<String>,
    #[clap(
        long = "db",
        default_value = "host=localhost dbname=relayer user=postgres password=password port=5432",
        help = "Database connection string.",
        env = "ETHCCD_RELAYER_DB_STRING"
    )]
    db_config: tokio_postgres::Config,
    #[clap(
        long = "prometheus-server",
        help = "Listen address:port for the Prometheus server.",
        env = "ETHCCD_RELAYER_PROMETHEUS_SERVER"
    )]
    prometheus_server: Option<std::net::SocketAddr>,
}

async fn find_start_ethereum_config<M: Middleware>(
    client: M,
    last_processed: Option<u64>,
    creation_height: u64,
    num_confirmations: u64,
) -> anyhow::Result<(u64, u64)>
where
    M::Error: 'static,
{
    let last_finalized: u64 = client
        .get_block_number()
        .await?
        .as_u64()
        .saturating_sub(num_confirmations);
    if let Some(last_processed) = last_processed {
        Ok((
            last_processed + 1,
            std::cmp::max(last_processed + 1, last_finalized),
        ))
    } else {
        Ok((
            creation_height,
            std::cmp::max(last_finalized, creation_height),
        ))
    }
}

async fn find_concordium_start_height(
    mut client: v2::Client,
    last_processed: Option<AbsoluteBlockHeight>,
    manager_address: ContractAddress,
) -> anyhow::Result<AbsoluteBlockHeight> {
    if let Some(h) = last_processed {
        Ok(h.next())
    } else {
        let (height, _, _) = client.find_instance_creation(.., manager_address).await?;
        Ok(height)
    }
}

async fn query_concordium_balance(
    metrics: ccdeth_relayer::metrics::Metrics,
    mut client: v2::Client,
    address: AccountAddress,
    min_balance: Amount,
) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    loop {
        interval.tick().await;
        match client
            .get_account_info(&address.into(), BlockIdentifier::LastFinal)
            .await
        {
            Ok(ai) => {
                let balance = ai.response.account_amount;
                metrics.concordium_balance.set(balance.micro_ccd);
                if ai.response.account_amount < min_balance {
                    anyhow::bail!(
                        "Concordium account balance {balance} is below minimum required \
                         {min_balance}."
                    );
                }
            }
            Err(e) => {
                metrics.warnings_total.inc();
                log::warn!("Unable to query Concordium account balance: {e:#}")
            }
        }
    }
}

async fn query_ethereum_balance<M: Middleware>(
    metrics: ccdeth_relayer::metrics::Metrics,
    client: M,
    address: ethers::prelude::Address,
    min_balance: U256,
) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    loop {
        interval.tick().await;
        match client.get_balance(address, None).await {
            Ok(balance) => {
                metrics
                    .ethereum_balance
                    .set((balance / 1_000_000_000_000u64).low_u64());
                if balance < min_balance {
                    anyhow::bail!(
                        "Ethereum account balance {balance} is below minimum required \
                         {min_balance}."
                    );
                }
            }
            Err(e) => {
                metrics.warnings_total.inc();
                log::warn!("Unable to query Ethereum account balance: {e:#}")
            }
        }
    }
}

/// Like `tokio::spawn` but the provided future is modified so that
/// once it terminates it sends a message on the provided channel.
/// This is sent regardless of how the future terminates, as long as it
/// terminates normally (i.e., does not panic).
fn spawn_cancel<T>(
    died_sender: tokio::sync::broadcast::Sender<()>,
    future: T,
) -> tokio::task::JoinHandle<T::Output>
where
    T: futures::Future + Send + 'static,
    T::Output: Send + 'static,
{
    tokio::spawn(async move {
        let res = future.await;
        // We ignore errors here since this always happens at the end of a task.
        // Since we keep one receiver alive until the end of the `main` function
        // the error should not happen anyhow.
        let _ = died_sender.send(());
        res
    })
}

#[tokio::main(worker_threads = 4)]
async fn main() -> anyhow::Result<()> {
    let app: Relayer = Relayer::parse();

    let mut log_builder = env_logger::Builder::from_env("ETHCCD_RELAYER_LOG");
    // only log the current module (main).
    log_builder.filter_module(module_path!(), app.log_level);
    log_builder.init();

    log::info!("Using {} as the maximum log level.", app.log_level);
    app.ethereum_config.log();
    app.concordium_config.log();

    let concordium_wallet = match (
        app.concordium_wallet.as_ref(),
        app.concordium_wallet_secret_name.as_ref(),
    ) {
        (Some(_), Some(_)) => {
            anyhow::bail!(
                "Both file and secret name provided as the key location for Concordium. Choose \
                 one."
            )
        }
        (Some(w), None) => WalletAccount::from_json_file(w)
            .context("Unable to read Concordium wallet from the provided file.")?,
        (None, Some(sn)) => ccdeth_relayer::aws_secret_manager::get_concordium_keys_aws(sn)
            .await
            .context("Unable to get Concordium wallet from Amazon secret manager.")?,
        (None, None) => {
            anyhow::bail!("Concordium keys were not provided.")
        }
    };
    let concordium_sender_address = concordium_wallet.address;
    log::info!(
        "Using {} as the sender of Concordium transactions.",
        concordium_sender_address
    );

    let inner_ethereum_client = {
        let network_client = reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(
                app.ethereum_config.ethereum_request_timeout,
            ))
            .connect_timeout(std::time::Duration::from_secs(10))
            .https_only(true)
            .build()
            .context("Unable to construct network client to access Ethereum API.")?;
        Http::new_with_client(app.ethereum_config.api, network_client)
    };
    let ethereum_client = RetryClient::new(
        inner_ethereum_client,
        Box::<HttpRateLimitRetryPolicy>::default(),
        5,
        3000,
    );

    let provider = Provider::new(ethereum_client);
    let ethereum_client = Arc::new(provider);

    // Transactions will be signed with the private key below and will be broadcast
    // via the eth_sendRawTransaction API)
    let wallet: LocalWallet = match (
        app.eth_private_key,
        app.eth_private_key_secret_name.as_ref(),
    ) {
        (Some(_), Some(_)) => {
            anyhow::bail!(
                "Both file and secret name provided as the key location for Ethereum. Choose one."
            )
        }
        (Some(w), None) => w.with_chain_id(app.ethereum_config.chain_id),
        (None, Some(sn)) => ccdeth_relayer::aws_secret_manager::get_ethereum_keys_aws(sn)
            .await
            .context("Unable to get Ethereum wallet from Amazon secret manager.")?
            .with_chain_id(app.ethereum_config.chain_id),
        (None, None) => {
            anyhow::bail!("Ethereum keys were not provided.")
        }
    };

    let ethereum_sender = wallet.address();
    log::info!("Using {ethereum_sender:#x} as the Ethereum wallet.");

    let balance = ethereum_client
        .get_balance(ethereum_sender, None)
        .await
        .context("Unable to get initial balance of the Ethereum sender")?;
    log::info!(
        "Balance of the Ethereum sender account is {} microETH.",
        balance / 1_000_000_000_000u64
    );
    let ethereum_nonce = ethereum_client
        .get_transaction_count(ethereum_sender, None)
        .await
        .context("Unable to get nonce for the Ethereum account")?;
    log::info!("Nonce of the Ethereum sender account is {ethereum_nonce}.");

    // Set up signal handlers before doing anything non-trivial so we have some sort
    // of graceful shut down during initial database lookups and pending
    // transaction sends.
    log::info!("Setting up signal handlers.");
    let (stop_sender, mut stop_receiver) = tokio::sync::watch::channel(());
    let (died_sender, died_receiver) = tokio::sync::broadcast::channel(10);
    let shutdown_handler_handle = spawn_cancel(
        died_sender.clone(),
        set_shutdown(stop_sender, died_receiver),
    );

    let (registry, metrics) = ccdeth_relayer::metrics::Metrics::new()
        .context("Unable to construct new metrics object.")?;
    if let Some(prometheus_server) = app.prometheus_server {
        log::info!("Starting prometheus server at {prometheus_server}.");
        spawn_cancel(
            died_sender.clone(),
            ccdeth_relayer::metrics::start_prometheus_server(prometheus_server, registry),
        );
    }

    let state_sender_contract =
        StateSender::new(app.ethereum_config.state_sender, ethereum_client.clone());

    let root_chain_manager_contract = ccdeth_relayer::root_chain_manager::BridgeManager::new(
        app.ethereum_config.root_chain_manager,
        Arc::new(ethereum_client.clone()),
    );

    let mut concordium_client = {
        // Use TLS if the URI scheme is HTTPS.
        // This uses whatever system certificates have been installed as trusted roots.
        let endpoint = if app
            .concordium_config
            .api
            .uri()
            .scheme()
            .map_or(false, |x| x == &http::uri::Scheme::HTTPS)
        {
            app.concordium_config
                .api
                .tls_config(ClientTlsConfig::new())
                .context("Unable to construct TLS configuration for the Concordium API.")?
        } else {
            app.concordium_config.api
        };
        let ep = endpoint
            .timeout(std::time::Duration::from_secs(
                app.concordium_config.request_timeout,
            ))
            .connect_timeout(std::time::Duration::from_secs(10));
        v2::Client::new(ep)
            .await
            .context("Unable to connect Concordium node.")?
    };
    {
        let lfb = concordium_client
            .get_consensus_info()
            .await
            .context("Unable to get Consensus info.")?
            .last_finalized_block;
        let bi = concordium_client
            .get_block_info(lfb)
            .await
            .context("Unable to get block information about last finalized block")?
            .response;
        if chrono::Utc::now().signed_duration_since(bi.block_slot_time)
            > chrono::Duration::seconds(app.concordium_config.max_behind.into())
        {
            anyhow::bail!(
                "Unable to start. The last finalized time of the Concordium node is more than {}s \
                 in the past.",
                app.concordium_config.max_behind,
            );
        }
    }

    let (last_ethereum, last_concordium, db) = Database::new(&app.db_config)
        .await
        .context("Unable to connect to the database.")?;
    let start_nonce = db
        .submit_missing_txs(concordium_client.clone())
        .await
        .context("Unable to submit missing transactions.")?;

    let bridge_manager_client = BridgeManagerClient::new(
        concordium_client.clone(),
        concordium_wallet.address,
        app.concordium_config.bridge_manager,
    );

    let bridge_manager = concordium_contracts::BridgeManager::new(
        bridge_manager_client.clone(),
        concordium_wallet,
        start_nonce,
        app.concordium_config.max_energy,
    )
    .await
    .context("Unable to connect to Concordium API.")?;

    let (start_number, upper_number) = find_start_ethereum_config(
        ethereum_client.clone(),
        last_ethereum,
        app.ethereum_config.state_sender_creation_block_number,
        app.ethereum_config.num_confirmations,
    )
    .await
    .context("Unable to find starting point for Ethereum monitoring")?;
    log::info!(
        "Found starting point on Ethereum chain at start = {start_number}, end = {upper_number})"
    );
    let concordium_start_height = find_concordium_start_height(
        concordium_client.clone(),
        last_concordium,
        app.concordium_config.bridge_manager,
    )
    .await
    .context("Unable to find starting point for Concordium monitoring.")?;

    log::info!("Starting at {concordium_start_height} on the Concordium chain.");

    let (max_marked_event_index, leaves) = db
        .pending_withdrawals(bridge_manager_client.clone())
        .await
        .context("Unable to get pending withdrawals.")?;

    // To spawn

    let (db_sender, db_receiver) = tokio::sync::mpsc::channel(50);
    let (ccd_transaction_sender, ccd_transaction_receiver) = tokio::sync::mpsc::channel(50);
    let (merkle_setter_sender, merkle_setter_receiver) = tokio::sync::mpsc::channel(50);

    let pending_merkle_set = db
        .pending_ethereum_tx()
        .await
        .context("Unable to get pending Merkle tree.")?;

    // Now we set up the main service, after we have established a baseline.
    // The different tasks communicate using channels established above.
    // The shutdown plan is as follows.
    // - Tasks which only query are just aborted.
    // - Tasks which send transactions or write to the database are given an
    //   opportunity to shut down gracefully by sending a signal on the
    //   stop_sender/stop_receiver channel. The same broadcast channel is shared by
    //   all tasks, and the only sender is the signal handler.
    let tx_sender_handle = spawn_cancel(
        died_sender.clone(),
        concordium_contracts::concordium_tx_sender(
            metrics.clone(),
            concordium_client.clone(),
            ccd_transaction_receiver,
            stop_receiver.clone(),
        ),
    );
    let db_task_handle = spawn_cancel(
        died_sender.clone(),
        db::handle_database(
            metrics.clone(),
            app.db_config,
            db,
            db_receiver,
            bridge_manager,
            ccd_transaction_sender,
            merkle_setter_sender,
            stop_receiver.clone(),
        ),
    );
    let merkle_updater_handle = {
        let merkle_client = MerkleSetterClient::new(
            root_chain_manager_contract,
            wallet,
            app.ethereum_config.max_gas_price,
            app.ethereum_config.max_gas,
            ethereum_nonce,
            &pending_merkle_set,
            std::time::Duration::from_secs(app.ethereum_config.merkle_update_interval),
            leaves,
            max_marked_event_index,
            std::time::Duration::from_secs(app.ethereum_config.escalation_interval),
            std::time::Duration::from_secs(app.ethereum_config.warn_duration),
        )
        .context("Unable to construct the client for setting Merkle roots.")?;

        spawn_cancel(
            died_sender.clone(),
            merkle::send_merkle_root_updates(
                metrics.clone(),
                merkle_client,
                pending_merkle_set,
                merkle_setter_receiver,
                db_sender.clone(),
                app.ethereum_config.num_confirmations,
                stop_receiver.clone(),
            ),
        )
    };

    // The remaining tasks only watch so they are aborted on signal received.
    let watch_concordium_handle = spawn_cancel(
        died_sender.clone(),
        concordium_contracts::listen_concordium(
            metrics.clone(),
            bridge_manager_client.clone(),
            db_sender.clone(),
            concordium_start_height,
            app.concordium_config.max_parallel,
            app.concordium_config.max_behind,
        ),
    );
    let watch_ethereum_handle = spawn_cancel(
        died_sender.clone(),
        ethereum::watch_eth_blocks(
            metrics.clone(),
            state_sender_contract,
            db_sender.clone(),
            start_number,
            upper_number,
            app.ethereum_config.num_confirmations,
        ),
    );

    let balance_query_handle = spawn_cancel(
        died_sender.clone(),
        query_concordium_balance(
            metrics.clone(),
            concordium_client,
            concordium_sender_address,
            Amount::from_micro_ccd(app.concordium_config.min_balance),
        ),
    );

    let ethereum_balance_query_handle = spawn_cancel(
        died_sender.clone(),
        query_ethereum_balance(
            metrics.clone(),
            ethereum_client,
            ethereum_sender,
            U256::from(app.ethereum_config.min_balance) * 1_000_000_000_000u64,
        ),
    );

    // Wait for signal to be received.
    if let Err(e) = stop_receiver.changed().await {
        log::error!("The signal handler unexpectedly died with {e}. Shutting off the service.");
    }

    // Stop watcher tasks.
    watch_concordium_handle.abort();
    watch_ethereum_handle.abort();
    balance_query_handle.abort();
    ethereum_balance_query_handle.abort();
    // And wait for all of them to terminate.
    let shutdown = [
        await_and_report("merkle updater", merkle_updater_handle),
        await_and_report("watch Ethereum", watch_ethereum_handle),
        await_and_report("watch Concordium", watch_concordium_handle),
        await_and_report("database handler", db_task_handle),
        await_and_report("concordium transaction sender", tx_sender_handle),
    ];
    shutdown
        .into_iter()
        .collect::<futures::stream::FuturesUnordered<_>>()
        .collect::<()>()
        .await;
    await_and_report("shutdown handler", shutdown_handler_handle).await;
    drop(died_sender); // keep the sender alive until here explicitly so that we don't have spurious
                       // errors when the last task is dying.
    Ok(())
}

/// Await the task to terminate. When terminated, if it raised an error,
/// report it together with the `descr` of the task.
async fn await_and_report<E: std::fmt::Display>(
    descr: &str,
    handle: tokio::task::JoinHandle<Result<(), E>>,
) {
    match handle.await {
        Ok(Ok(())) => {
            log::info!("Task {descr} terminated.");
        }
        Ok(Err(e)) => {
            log::error!("Task {descr} unexpectedly stopped due to {e:#}.");
        }
        Err(e) => {
            if e.is_panic() {
                log::error!("Task panicked.");
            } else if e.is_cancelled() {
                log::info!("Task {descr} was cancelled.");
            } else {
                log::error!("Task {descr} unexpectedly closed.");
            }
        }
    }
}

/// Construct a future for shutdown signals (for unix: SIGINT and SIGTERM) (for
/// windows: ctrl c and ctrl break). The signal handler is set when the future
/// is polled and until then the default signal handler.
async fn set_shutdown(
    stop_sender: tokio::sync::watch::Sender<()>,
    mut task_died: tokio::sync::broadcast::Receiver<()>,
) -> anyhow::Result<()> {
    #[cfg(unix)]
    {
        use tokio::signal::unix as unix_signal;
        let mut terminate_stream = unix_signal::signal(unix_signal::SignalKind::terminate())?;
        let mut interrupt_stream = unix_signal::signal(unix_signal::SignalKind::interrupt())?;
        let terminate = Box::pin(terminate_stream.recv());
        let interrupt = Box::pin(interrupt_stream.recv());
        let task_died = Box::pin(task_died.recv());
        futures::future::select(task_died, futures::future::select(terminate, interrupt)).await;
        if stop_sender.send(()).is_err() {
            log::error!("Unable to send stop signal.");
        }
    }
    #[cfg(windows)]
    {
        use tokio::signal::windows as windows_signal;
        let mut ctrl_break_stream = windows_signal::ctrl_break()?;
        let mut ctrl_c_stream = windows_signal::ctrl_c()?;
        let ctrl_break = Box::pin(ctrl_break_stream.recv());
        let ctrl_c = Box::pin(ctrl_c_stream.recv());
        let task_died = Box::pin(task_died.recv());
        futures::future::select(task_died, futures::future::select(ctrl_break, ctrl_c)).await;
        if stop_sender.send(()).is_err() {
            log::error!("Unable to send stop signal.");
        }
    }
    Ok(())
}
