# Leveraging the pre-built Docker images with 
# cargo-chef and the Rust toolchain
FROM lukemathwalker/cargo-chef:latest-rust-1.69 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin api_server

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS runtime

ARG ETHCCD_RELAYER_LOG_LEVEL trace

ARG DB

RUN apt-get update && \
    apt-get -y install \
      ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR app
COPY --from=builder /app/target/release/api_server /usr/local/bin
CMD ["/usr/local/bin/api_server"]