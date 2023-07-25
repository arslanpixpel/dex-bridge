# Leveraging the pre-built Docker images with 
# cargo-chef and the Rust toolchain
# FROM lukemathwalker/cargo-chef:latest-rust-1.69 AS chef
# WORKDIR app

# FROM chef AS planner
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.69 as builder 
# COPY --from=planner /app/recipe.json recipe.json
WORKDIR app

# RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y
RUN update-ca-certificates

# Build dependencies - this is the caching Docker layer!
# RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin ccdeth_relayer

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS runtime

ARG ETHCCD_API_DB_STRING
ARG STATE_SENDER_ADDRESS
ARG ROOT_CHAIN_MANAGER_ADDRESS
ARG CHAIN_ID
ARG STATE_SENDER_CREATION_HEIGHT
ARG ETHEREUM_API
ARG ETH_MIN_BALANCE
ARG BRIDGE_MANAGER_ADDRESS
ARG CCD_MIN_BALANCE
ARG CONCORDIUM_API
ARG CONCORDIUM_WALLET
ARG ETH_PRIVATE_KEY

RUN apt-get update && \
    apt-get -y install \
      ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR app
COPY --from=builder /app/target/release/ccdeth_relayer /usr/local/bin/ccdeth_relayer
CMD ["/usr/local/bin/ccdeth_relayer"]