#!/bin/bash

set -ex

cd deploy-scripts

git submodule init && git submodule update

cd deps/concordium-rust-sdk

git submodule init && git submodule update

cd concordium-base

git submodule init && git submodule update

cd ../../..

cargo build