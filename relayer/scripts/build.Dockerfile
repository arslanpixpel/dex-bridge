ARG build_image
ARG base_image
FROM ${build_image} AS build

WORKDIR /build
COPY relayer relayer
COPY deps/concordium-rust-sdk deps/concordium-rust-sdk
RUN cargo build --locked --manifest-path relayer/Cargo.toml --release

FROM ${base_image}
RUN apt-get update && \
    apt-get -y install \
      ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=build /build/relayer/target/release/ccdeth_relayer /usr/local/bin/
COPY --from=build /build/relayer/target/release/api_server /usr/local/bin/

