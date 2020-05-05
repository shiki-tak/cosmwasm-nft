#!/bin/bash
rm -rf contract.wasm hash.txt

cargo check

cargo schema

docker run --rm -v $(pwd):/code --mount type=volume,source=$(basename $(pwd))_cache,target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry confio/cosmwasm-opt:0.7.3

wasmcli tx wasm store contract.wasm --from validator --gas 42000000 -y
