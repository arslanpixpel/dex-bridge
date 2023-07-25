# Relayer

The relayer is the component of the bridge that monitors both the Ethereum and
Concordium chains and acts on events on both of them by submitting transactions
on the other chain.

The relayer collects events and auxiliary data in a PostgreSQL database. The
high-level design of the relayer is as follows.

The relayer consists of a number of tasks that work concurrently.
- A task for looking at events on the Ethereum chain, using the provided API
  URL, e.g., Infura.
- A task for looking at events on Concordium, using a connection to the node
  that is provided.
- A task for sending transactions to the Concordium chain.
- A task for sending transactions to the Etheruem chain (this is called the
  MerkleClient in the code).
- A database task that has exclusive access to the database. It communicates
  with other tasks via MPSC channels. This is used to ensure atomicity of state
  updates, and to make it simpler to do reconnects, since they have to be done
  in a single place only. All processing is done per block and checkpoints are
  stored in the database when blocks are processed.

The overall flow is as follows
- when relevant events are observed on the chain they are written to the
  database.
- if any actions are needed as a result of these events these actions are
  written to the database in the same database transaction as the events
  themselves.
- after the database transaction is committed the relevant transactions are sent
  to either Concordium or Ethereum chains.
- when transactions are sent to Concordium we don't do anything afterwards and
  we rely on the monitoring task to observe when the transaction is finalized
  and its effects.
- when a transaction is send to Ethereum we follow it since we need to update
  the database when it is finalized with auxiliary data needed by the API server
  to produce Merkle proofs.

There is retry logic for events that are deemed transient, and the service will
try for some time to re-establish the connections to Ethereum APIs, Concordium
node, and the database. In case any of the tasks terminate then all are killed
and the service will stop. A task ending prematurely indicates a bug, or failure
to reconnect for some time.

## Configuration options

The service can be configured using either command-line options or environment
variables.

The following configuration options are available

### General configuration
- Configure logging level. Supported values are `off, error, info, debug, trace`.

      --log-level <LOG_LEVEL>
          Maximum log level. [env: ETHCCD_RELAYER_LOG_LEVEL=] [default: info]

- Database connection string. This must point to a PostgreSQL database

      --db <DB_CONFIG>
          Database connection string. [env: ETHCCD_RELAYER_DB_STRING=] [default: "host=localhost dbname=relayer user=postgres password=password port=5432"]

- Address where the prometheus exporter should listen. If not set the prometheus
  server is not started.

      --prometheus-server <PROMETHEUS_SERVER>
          Listen address:port for the Prometheus server. [env: ETHCCD_RELAYER_PROMETHEUS_SERVER=]

### Ethereum specific options

- Address of the `StateSender` contract (or proxy) which is monitored for all Ethereum events.

      --state-sender-address <STATE_SENDER>
          Address of the StateSender proxy instance on Ethereum. [env: ETHCCD_RELAYER_STATE_SENDER_PROXY=]

- Address of the `RootChainManager` contract which is sent Merkle root updates.

      --root-chain-manager-address <ROOT_CHAIN_MANAGER>
          Address of the RootChainManager proxy instance on Ethereum. [env: ETHCCD_RELAYER_ROOT_CHAIN_MANAGER_PROXY=]

- Block number where the `StateSender` instance was created. This is used only
      during initial startup as a starting point for monitoring the chain

      --state-sender-creation-height <STATE_SENDER_CREATION_BLOCK_NUMBER>
          Block number when the state sender instance was created. This is used as a starting point for monitoring the Ethereum chain. [env: ETHCCD_RELAYER_STATE_SENDER_CREATION_BLOCK_NUMBER=]

- URL of the Ethereum JSON-RPC API, e.g., https://goerli.infura.io/v3/$API_KEY

      --ethereum-api <ethereum-api>
          JSON-RPC interface of an Ethereum node. Only HTTPS is supported as transport. [env: ETHCCD_RELAYER_ETHEREUM_API=]

- Maximum allowed gas price. If the price is higher than that then Merkle root updates are not going to be sent.

      --max-gas-price <MAX_GAS_PRICE>
          Maximum gas price allowed for Ethereum transactions. If the current gas price is higher then the Merkle updates will be skipped. [env: ETHCCD_RELAYER_MAX_GAS_PRICE=] [default: 1000000000]

- Maximum allowed gas cost of Merkle root updates. The default is reasonable here since the cost of this transaction is fixed.

      --max-gas <MAX_GAS>
          Maximum gas allowed for setting the Merkle root on Ethereum. [env: ETHCCD_RELAYER_MAX_GAS=] [default: 100000]

- How often to send Merkle root updates to Ethereum (in seconds).

      --merkle-update-interval <MERKLE_UPDATE_INTERVAL>
          How often to approve new withdrawals on Ethereum. [env: ETHCCD_RELAYER_MERKLE_UPDATE_INTERVAL=] [default: 600]

- The private key used for signing Merkle root updates. This option conflicts
  with `--eth-key-secret-name` option.

      --eth-private-key <eth-private-key>
          Private key used to sign Merkle update tranasctions on Ethereum. The address derived from this key must have the MERKLE_UPDATER role. [env: ETHCCD_RELAYER_ETH_PRIVATE_KEY=]

- The Amazon Secret Manager secret to retrieve the private key used for signing
  Merkle root updates. The access to the secret manager should be configured
  via the host.

      --eth-key-secret-name <eth-key-secret-name>
          Secret name of the key stored in Amazon secret manager. [env: ETHCCD_RELAYER_ETH_PRIVATE_KEY_SECRET_NAME=]

- The chain id corresponding to the network. This is used when sending transactions.
      --chain-id <CHAIN_ID>
          Chain ID. Goerli is 5, mainnet is 1. [env: ETHCCD_RELAYER_CHAIN_ID=]

- The number of confirmations to require before considering Ethereum
  transactions finalized.

      --num-confirmations <NUM_CONFIRMATIONS>
          Number of confirmations required on Ethereum before considering the transaction as final. [env: ETHCCD_RELAYER_NUM_CONFIRMATIONS=] [default: 10]

- Timeout for individual requests to the Ethereum API.

      --ethereum-request-timeout <ETHEREUM_REQUEST_TIMEOUT>
          Timeout for requests to the Ethereum node. [env: ETHCCD_RELAYER_ETHEREUM_REQUEST_TIMEOUT=] [default: 10]

- When sending transactions

      --escalation-interval <ESCALATION_INTERVAL>
          Interval (in seconds) on when to escalate the price of the transaction. [env: ETHCCD_RELAYER_MERKLE_ESCALATION_INTERVAL=] [default: 120]

- Start warning that a transaction has not been confirmed after this amount of time (in seconds).

      --warn-duration <WARN_DURATION>
          When to start warning that the transaction has not yet been confirmed. [env: ETHCCD_RELAYER_MERKLE_WARN_DURATION=] [default: 120]

- Minimum balance of the Ethereum account. If this is breached then the service
  will stop. The account must then be topped up and service restarted.

      --eth-min-balance <eth-min-balance>
          Minimum balance of the Ethereum account. In microEther [env: ETHCCD_RELAYER_MIN_ETHEREUM_BALANCE=]

### Concordium specific options

- Link to the Concordium V2 GRPC API.

      --concordium-api <concordium-api>
          GRPC V2 interface of the Concordium node. [env: ETHCCD_RELAYER_CONCORDIUM_API=] [default: http://localhost:20000]

- Maximum number of parallel queries to do when querying the Concordium node.
  This is only relevant if the relayer is started a lot after the Contracts
  are deployed, during initial catchup. It should otherwise be 1.

      --concordium-max-parallel <MAX_PARALLEL>
          Maximum number of parallel queries of the Concordium node. This is only useful in initial catchup if the relayer is started a long time after the bridge contracts are in operation. [env: ETHCCD_RELAYER_MAX_PARALLEL_QUERIES_CONCORDIUM=] [default: 1]

- Maximum allowed time for a the last finalized block to be behind before failing and attempting to reconnect to the Concordium API.

      --concordium-max-behind <MAX_BEHIND>
          Maximum number of seconds the Concordium node's last finalized block can be behind before we log warnings. [env: ETHCCD_RELAYER_CONCORDIUM_MAX_BEHIND=] [default: 240]

- Request timeout for each individual Concordium API request.

      --request-timeout <REQUEST_TIMEOUT>
          Timeout for requests to the Concordium node. [env: ETHCCD_RELAYER_CONCORDIUM_REQUEST_TIMEOUT=] [default: 10]

- Address of the `BridgeManager` instance on Concordium. This is used both to
  monitor for new events and to send deposit transactions

      --bridge-manager-address <BRIDGE_MANAGER>
          Address of the BridgeManger contract instance on Concordium. [env: ETHCCD_RELAYER_BRIDGE_MANAGER=]

- Maximum NRG allowed for execution of deposits and token map transactions on
  Concordium. This should generally not be changed from the default.

      --max-energy <MAX_ENERGY>
          Maximum energy to allow for transactions on Concordium. [env: ETHCCD_RELAYER_CONCORDIUM_MAX_ENERGY=] [default: 100000]

- Minimum allowed balance of CCD on the Concordium sender account. If the
  balance goes below this then the service will stop.

      --ccd-min-balance <ccd-min-balance>
          Minimum balance of the Concordium account. In microCCD [env: ETHCCD_RELAYER_MIN_CONCORDIUM_BALANCE=]

- The path to the Concordium wallet, in the format that is exported from the
  browser extension wallet. This option conflicts with `concordium-wallet-secret-name`

      --concordium-wallet-file <concordium-wallet-file>
          File with the Concordium wallet in the browser extension wallet export format. [env: ETHCCD_RELAYER_CONCORDIUM_WALLET_FILE=]

- The Amazon Secret Manager secret to retrieve the Concordium wallet.
  The access to the secret manager should be configured via the host.

      --concordium-wallet-secret-name <concordium-wallet-secret-name>
          File with the Concordium wallet in the browser extension wallet export format. [env: ETHCCD_RELAYER_CONCORDIUM_WALLET_SECRET_NAME=]

## Logging levels.

The service logs events of interest on `error`, `warn`, `info`, and `debug` levels.

### Error

Only critical errors are logged on this level and most of them lead to service
shutdown. There are some errors logged on this level which should be manually
investigated since they are most likely due to misconfiguration.

### Warn

This level is used to log situations where the service has experienced
intermittent failure, such as loss of connectivity to, e.g., the database. The
service should recover from those situations, but an abnormal number of warnings
should be investigated to identify the root cause.

### Info, Debug.

These are for information purposes only and do not have to be monitored closely.

## Building the binaries

The project is a pure Rust project, and can be built by running

```shell
cargo build --release
```

The minimum supported Rust version is 1.67. There are issues with older version,
some with dependencies, and a compiler bug in 1.65 which causes compilation failure.

This produces a single binary `target/release/ccdeth_relayer`.

**Make sure that you have checked and initialized submodules before the build**
e.g., using
```
git submodule update --init --recursive
```

## Docker image

A docker image containing the relayer and API server can be built using the
provided [`Dockerfile`](./scripts/build.Dockerfile) as follows **from the root
of the repository**. Make sure to do a full repository checkout first using

```
git submodule update --init --recursive
```

Then run

```
docker build \
    --build-arg build_image=rust:1.67-buster\
    --build-arg base_image=debian:buster\
    -f relayer/scripts/build.Dockerfile\
    -t ccdeth_relayer:latest .
```

The image has two binaries, `ccdeth_relayer` and `api_server` installed at `/usr/local/bin/`.

## Metrics

The relayer service exposes a Prometheus exporter if configured with
`ETHCCD_RELAYER_PROMETHEUS_SERVER` or `--prometheus-server` flag. The value
should be `IP:PORT` to listen on. The metrics can be collected on
`IP:PORT/metrics` endpoint. These can be used for monitoring the service for any
irregularity.

The following metrics are exposed
- `concordium_account_balance` - Balance, in microCCD, of the sender account for
  Concordium. This should be monitored so that it does not become too low. If
  this value goes below `--ccd-min-balance` the service will shut down.
- `concordium_height` - Largest processed height for Concordium. This indicates
  progress. If this lingers then likely the service has trouble querying new
  blocks from the Concordium node, or the Concordium node is behind.
- `errors_total` - Number of errors emitted since start of the service. Errors
  are only emitted on irregularities and in normal operation there should not be
  any errors. The logs should be consulted if there are errors.
- `warnings_total` - Number of warnings emitted since start of the service.
  Warnings are non-critical in the sense that the service is expected to recover
  from them. However a large spike in warnings indicates a problem and should be
  investigated.
- `ethereum_account_balance` - Balance, in microEther, of the sender account for
  Ethereum. If this goes below `--eth-min-balance` then the service will stop.
- `ethereum_height` - Largest processed height for Ethereum. This indicates
  progress. If this lingers then likely the service has trouble querying new
  blocks from Etheruem API.
- `merkle_tree_size` - Current size of the Merkle tree for withdrawal approvals.
- `num_completed_deposits` - Number deposits completed on Concordium since start.
- `num_completed_withdrawals` - Number of withdrawals completed since start.
- `num_deposits` -  Number deposits detected since start. This should be close
  to number of completed deposits, but at any point in time there can be a
  slight discrepancy. A large discrepancy indicates and issue.
- `num_withdrawals` - Number of started withdrawals detected since start.
  This will differ from `num_completed_withdrawals` since withdrawals are
  batched and only happen every update interval.
- `sent_concordium_transactions` - Number of transactions sent to Concordium since start.
- `sent_ethereum_transactions` Number of transactions sent to Ethereum since start.
- `timestamp_last_merkle_root` Unix timestamp in seconds of the last time a Merkle root was set.


## Generation of clients for Ethereum contracts.

The relayer needs to interact with the root chain manager on Ethereum, and
listen for events emitted by the state sender contract. The `ethers.rs` library
can generate these clients from the ABI of those contracts (specifically
`src/erc20.rs`, `src/root_chain_manager.rs` and `src/state_sender.rs` are
generated). The build script for the relayer can be used to generate these
clients if the build is run using `generate-client` feature, i.e.,

```
cargo build --feature=generate-client
```

This should only be necessary if the ABI of those contracts changes.

# API server

The package contains another binary, the api-server which exposes data from the
database that the relayer writes to. The API is used by the bridge frontend to
keep track of transactions, and to get Merkle proofs.

The following configuration options are available

- Maximum logging level, options are `off`, `error`, `warn`, `info`, `debug`, `trace`.

      --log-level <LOG_LEVEL>
          Maximum log level. [env: ETHCCD_API_LOG_LEVEL=] [default: info]

- Database connection string.

      --db <DB_CONFIG>
          Database connection string. [env: ETHCCD_API_DB_STRING=] [default: "host=localhost dbname=relayer user=postgres password=password port=5432"]

- Address where the server will listen on for its API.

      --listen-address <LISTEN_ADDRESS>
          Listen address for the server. [env: ETHCCD_API_LISTEN_ADDRESS=] [default: 0.0.0.0:8080]

- Optional address of the Prometheus server. The server exposes one endpoint
  `/metrics` which contains information about the accessed endpoints and timings
  of requests.

      --prometheus-address <PROMETHEUS_ADDRESS>
          Listen address for the server. [env: ETHCCD_API_PROMETHEUS_ADDRESS=] [default: 0.0.0.0:9090]

- The maximum number of database connections to be kept at the connection pool.
      --max-pool-size <MAX_POOL_SIZE>
          Maximum size of a database connection pool. [env: ETHCCD_API_MAX_DB_CONNECTION_POOL_SIZE=] [default: 16]

- Maximum request handling duration, in milliseconds.

- Log request and response headers. This is quite verbose, so should only be
      used when diagnosing a problem.

      --log-headers
          Whether to log headers for requests and responses. [env: ETHCCD_API_LOG_HEADERS=]

- The API server can serve static files from a given directory. This is intended
  to serve token metadata. The files are served under `/assets/`
 
      --assets-dir <ASSETS_DIR>
          Serve files from the supplied directory under /assets. [env: ETHCCD_API_SERVE_ASSETS=]

# Notes for operation of the relayer

The relayer is built to be able to recover from most outages, such as the node
being disconnected, the Ethereum API being temporarily unavailable, and the
database being temporarily unavailable. However there are some scenarios which
could make it stop relaying deposits and withdrawals.

## Use of the Concordium account

The relayer assumes that it has the sole ownership of the account it uses to
send Concordium transactions. Incoming transfers to the account are fine, but no
other entity should be sending transactions from it. If it does the relayer will
fail. If such operations are needed then the following steps should be followed
- stop the relayer
- the necessary transactions should be sent and **waited until they are
  finalized**
- the relayer is restarted

## Incorrect configuration of the Concordium account.

If the configuration of the relayer is incorrect, in particular if the maximum
allowed energy is insufficient for sending state updates to Concordium chain
then the transactions will fail and will not be retried. There is currently no
automatic recovery from such a situation. See the section below on
coarse-grained recovery.

## Price fluctuations on the Ethereum chain

The relayer is configured with `MAX_GAS_PRICE` which states the maximum gas
price allowed for the Ethereum transaction (setting the Merkle root). The
relayer will try to get the current price from the configured API when it needs
to send the transaction. If the transaction is not committed in time (configured
via `ESCALATION_INTERVAL`) then the relayer will increase the gas price by 5%
and send it again. This process continues until either the transaction is
successful, or the maximum gas price is hit. At that point the relayer will wait
and just check on the existing transactions it has sent. This can potentially
lead to infinite waiting if the gas price does not drop. One possible recovery
in such a situation is to restart the relayer with increased maximum gas price.
Another option is to wait until the price drops and restart the relayer.

## Coarse grained recovery

The state of the relayer is stored in a Postgres database. This includes
checkpoints, sent transactions, their status, and events emitted from the
contracts. The database is used by the api server to support the frontend.

The relayer is designed so that it is **safe** to delete the entire database
restart the server. However it is not free. Some transactions might be sent to
the chains that will not have an effect (e.g., a set merkle root transaction
that only approves withdrawals that have already been approved). The cost should
be minimal however.

Note that during this recovery the api server will have stale information, and
so the frontend will not be fully operational.

In light of this the recommended setup for operation is to do a daily database
backup. If there are irrecoverable issues then a backup from the last day, or
previous if detection of issues took longer than a day, should be restored and
the service restarted. Catching up for one day is going to take a few minutes
only.

## Security assumptions

The following are critical for security of the relayer

- the connection to the Concordium node. The relayer by nature has to trust the
  data that is coming from the node. So the node itself is trusted, and the link
  between the node and the relayer as well. Consequently, if the relayer is
  served incorrect data it will act on it. The relayer supports TLS when
  connecting to the node if the two are connected via public internet. This
  features should be used.
- the data from the Ethereum API provider is trusted. No secret data is
  transmitted on this connection, however the relayer acts on the information it
  receives on this connection. This means it can be compromised if this
  connection is compromised. For this reason the relayer only supports HTTPS.
- the keys for approving withdrawals for the Ethereum chain. Compromise of these
  keys allows for draining of the bridge, i.e., taking ownership of all the
  tokens locked on the Ethereum side of the bridge.
- the keys for the account on Concordium that is allowed to issue deposits. If
  keys of this account are compromised then the bridge can be drained, but also
  arbitrary amounts of bridged tokens can be minted.


In particular the Postgres database is not critical for security in the sense
that manipulating the database cannot be used to drain the bridge. However
manipulating the database can affect the relayer, by making it skip events, and
it can affect the api server by making it serve incorrect transaction histories,
incorrect Merkle proofs, etc. This will lead to failed transactions on the
respective chains.
