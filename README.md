# LP Staking Contracts
This repository contains the smart contract for the LP staking and single staking offerings.

## Development

### Environment Setup

- Rust v1.53.1+
- `wasm32-unknown-unknown` target
- Docker

1. Install `rustup` via https://rustup.rs/

2. Run the following:

```sh
rustup default stable
rustup target add wasm32-unknown-unknown
```

3. Make sure [Docker](https://www.docker.com/) is installed

### Unit / Integration Tests

Each contract contains Rust unit tests embedded within the contract source directories. You can run:

```sh
cargo unit-test
```

### Compiling

After making sure tests pass, you can compile each contract with the following:

```sh
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/cw1_subkeys.wasm .
ls -l cw1_subkeys.wasm
sha256sum cw1_subkeys.wasm
```

#### Production

For production builds, run the following:

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.11.5
```

This performs several optimizations which can significantly reduce the final size of the contract binaries, which will be available inside the `artifacts/` directory.

Contracts are based on the Staking Contracts of Valkyrie Protocol

Contract Verification
lp_staking.wasm
Checksum: 9d8153b2dcc4aa49fda9f32f36eb4a9294000bde49112c3958494cfb74fef4c5

https://lcd.terra.dev/wasm/codes/1859

{"height":"5890139","result":{
  "code_id": "1859",
  "code_hash": "nYFTstzEqkn9qfMvNutKkpQAC95JESw5WElM+3T+9MU=",
  "creator": "terra16k0mnlqd36a4v3t3m62057wkxjxy46s7ppjj4m"
}}

Base64 to Hex of nYFTstzEqkn9qfMvNutKkpQAC95JESw5WElM+3T+9MU=

9d8153b2dcc4aa49fda9f32f36eb4a9294000bde49112c3958494cfb74fef4c5
