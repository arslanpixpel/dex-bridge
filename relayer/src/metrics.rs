use prometheus::{
    core::{AtomicU64, GenericGauge},
    IntCounter, IntGauge, Registry, TextEncoder,
};

#[derive(Clone)]
pub struct Metrics {
    pub(crate) merkle_tree_size: IntGauge,
    pub warnings_total: IntCounter,
    pub(crate) errors_total: IntCounter,
    pub(crate) num_deposits: IntCounter,
    pub(crate) num_completed_deposits: IntCounter,
    pub(crate) num_withdrawals: IntCounter,
    pub(crate) num_completed_withdrawals: IntCounter,
    pub(crate) concordium_height: IntGauge,
    pub(crate) ethereum_height: IntGauge,
    pub(crate) sent_concordium_transactions: IntCounter,
    pub(crate) sent_ethereum_transactions: IntCounter,
    pub(crate) time_last_merkle_root: IntGauge,
    pub concordium_balance: GenericGauge<AtomicU64>,
    pub ethereum_balance: GenericGauge<AtomicU64>,
}

impl Metrics {
    pub fn new() -> anyhow::Result<(Registry, Self)> {
        let registry = Registry::new();

        let merkle_tree_size = IntGauge::new(
            "merkle_tree_size",
            "Current size of the Merkle tree for withdrawal approvals.",
        )?;
        registry.register(Box::new(merkle_tree_size.clone()))?;

        let warnings_total = IntCounter::new(
            "warnings_counter",
            "Number of warnings emitted since start.",
        )?;
        registry.register(Box::new(warnings_total.clone()))?;

        let errors_total =
            IntCounter::new("errors_total", "Number of errors emitted since start.")?;
        registry.register(Box::new(errors_total.clone()))?;

        let num_deposits =
            IntCounter::new("num_deposits", "Number deposits detected since start.")?;
        registry.register(Box::new(num_deposits.clone()))?;

        let num_completed_deposits = IntCounter::new(
            "num_completed_deposits",
            "Number deposits completed on Concordium since start.",
        )?;
        registry.register(Box::new(num_completed_deposits.clone()))?;

        let num_withdrawals = IntCounter::new(
            "num_withdrawals",
            "Number of started withdrawals detected since start.",
        )?;
        registry.register(Box::new(num_withdrawals.clone()))?;

        let num_completed_withdrawals = IntCounter::new(
            "num_completed_withdrawals",
            "Number of withdrawals completed since start.",
        )?;
        registry.register(Box::new(num_completed_withdrawals.clone()))?;

        let concordium_height = IntGauge::new(
            "concordium_height",
            "Largest processed height for Concordium.",
        )?;
        registry.register(Box::new(concordium_height.clone()))?;

        let ethereum_height =
            IntGauge::new("ethereum_height", "Largest processed height for Ethereum.")?;
        registry.register(Box::new(ethereum_height.clone()))?;

        let sent_concordium_transactions = IntCounter::new(
            "sent_concordium_transactions",
            "Number of transactions sent to Concordium since start.",
        )?;
        registry.register(Box::new(sent_concordium_transactions.clone()))?;

        let sent_ethereum_transactions = IntCounter::new(
            "sent_ethereum_transactions",
            "Number of transactions sent to Ethereum since start.",
        )?;
        registry.register(Box::new(sent_ethereum_transactions.clone()))?;

        let time_last_merkle_root = IntGauge::new(
            "timestamp_last_merkle_root",
            "Unix timestamp in seconds of the last time a Merkle root was set.",
        )?;
        registry.register(Box::new(time_last_merkle_root.clone()))?;

        let concordium_balance = GenericGauge::new(
            "concordium_account_balance",
            "Balance, in microCCD, of the sender account for Concordium.",
        )?;
        registry.register(Box::new(concordium_balance.clone()))?;

        let ethereum_balance = GenericGauge::new(
            "ethereum_account_balance",
            "Balance, in microEther, of the sender account for Ethereum.",
        )?;
        registry.register(Box::new(ethereum_balance.clone()))?;

        Ok((registry, Self {
            merkle_tree_size,
            warnings_total,
            errors_total,
            num_deposits,
            num_withdrawals,
            num_completed_withdrawals,
            concordium_height,
            ethereum_height,
            sent_concordium_transactions,
            sent_ethereum_transactions,
            time_last_merkle_root,
            num_completed_deposits,
            concordium_balance,
            ethereum_balance,
        }))
    }
}

async fn text_metrics(
    axum::extract::State(registry): axum::extract::State<Registry>,
) -> Result<String, axum::response::ErrorResponse> {
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    Ok(encoder
        .encode_to_string(&metric_families)
        .map_err(|_| "Unable to encode metrics.")?)
}

/// Start the server. The task only terminates if the server terminates, i.e.,
/// if it crashes.
pub async fn start_prometheus_server(
    addr: std::net::SocketAddr,
    registry: Registry,
) -> anyhow::Result<()> {
    // build our application with a single route
    let app = axum::Router::new()
        .route("/metrics", axum::routing::get(text_metrics))
        .with_state(registry)
        .layer(tower_http::timeout::TimeoutLayer::new(
            std::time::Duration::from_millis(1000),
        ))
        .layer(tower_http::limit::RequestBodyLimitLayer::new(0)); // no bodies, we only have GET requests.
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
