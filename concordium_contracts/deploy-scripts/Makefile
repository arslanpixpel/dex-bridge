all: cis2-bridgeable bridge-manager collect-schemas deploy-scripts

cis2-bridgeable:
	cd ../cis2-bridgeable; \
	cargo concordium build --schema-out="./scripts/schema.bin"

bridge-manager: bridge-manager-test
	cd ../bridge-manager; \
	cargo concordium build --schema-out="./scripts/schema.bin"

cis2-bridgeable-test:
	cd ../cis2-bridgeable; \
	RUSTFLAGS="-Awarnings" cargo test;

bridge-manager-test:
	cd ../bridge-manager; \
	RUSTFLAGS="-Awarnings" cargo test;

test: cis2-bridgeable-test bridge-manager-test

collect-schemas: bridge-manager cis2-bridgeable
	mkdir -p data; \
	cp ../cis2-bridgeable/scripts/schema.bin data/cis2_bridgeable.bin; \
	cp ../bridge-manager/scripts/schema.bin data/bridge_manager.bin;
	cp ../bridge-manager/target/concordium/wasm32-unknown-unknown/release/bridge_manager.wasm.v1 data/bridge_manager.wasm.v1
	cp ../cis2-bridgeable/target/concordium/wasm32-unknown-unknown/release/cis2_bridgeable.wasm.v1 data/cis2_bridgeable.wasm.v1

deploy-scripts:
	cargo build