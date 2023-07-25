#!/bin/bash

set -ex

cd deploy-scripts

cargo_concordium_version="2.5.0"

rustup target add wasm32-unknown-unknown
wget "https://distribution.concordium.software/tools/linux/cargo-concordium_$cargo_concordium_version"
cargo_binary=$(which cargo)
cargo_dir=$(dirname $cargo_binary)
mv "cargo-concordium_$cargo_concordium_version" $cargo_dir/cargo-concordium
chmod +x $cargo_dir/cargo-concordium

make