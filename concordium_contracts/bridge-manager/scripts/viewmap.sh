#!/bin/bash

concordium-client contract invoke 4602 \
	--entrypoint viewTokenMappings \
	--energy=3000 \
	--schema=schema.bin \
	--grpc-ip node.testnet.concordium.com
	# --sender=default \
	# --grpc-port 20000 \