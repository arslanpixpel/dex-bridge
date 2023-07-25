#!/bin/bash

concordium-client contract update 4602 \
	--entrypoint receiveStateUpdate \
	--energy=3000 \
	--sender=default \
	--parameter-json map.json \
	--schema=schema.bin \
	--grpc-ip node.testnet.concordium.com
	# --grpc-port 20000 \