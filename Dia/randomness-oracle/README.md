# randomness-oracle

## Overview
On-chain oracle for serving random values from [drand](https://drand.love/) service. Should be feeded by off-chain relay - `randomness-oracle-feeder`.

## Setup
1. Install Rust.
2. Add nightly toolchain: `rustup toolchain add nightly`.
3. Add wasm target: `rustup target add wasm32-unknown-unknown --toolchain nightly`.
4. Build oracle in release mode: `cargo +nightly build --release`.
5. Oracle binaries located in: `target/wasm32-unknown-unknown/release/`.