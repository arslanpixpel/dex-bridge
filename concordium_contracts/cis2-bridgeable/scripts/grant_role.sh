#!/bin/bash

concordium-client contract invoke 4591 \
	--entrypoint viewTokenOwners \
	--energy=3000 \
	--schema=schema.bin \
	--grpc-ip node.testnet.concordium.com