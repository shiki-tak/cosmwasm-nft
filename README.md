# COSMWASM-NFT
[![CircleCI](https://circleci.com/gh/shiki-tak/cosmwasm-nft.svg?style=svg)](https://circleci.com/gh/shiki-tak/cosmwasm-nft)
[![codecov](https://codecov.io/gh/shiki-tak/cosmwasm-nft/branch/master/graph/badge.svg)](https://codecov.io/gh/shiki-tak/cosmwasm-nft)

## Compile
```
❯ docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.6
```
