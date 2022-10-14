# randomness-oracle-feeder

## Overview

Off-chain relay for `randomness-oracle`. This relay takes values from [drand](https://drand.love/) service and feed on-chain oracle.

## Setup

1. Create `.env` file, fill with the following contents:

```
ENDPOINT_URL = "RPC NODE URL"
ORACLE_ADDRESS = "RANDOMNESS ORACLE ADDRESS(with 0x prefix)"
ORACLE_META_WASM_PATH = "PATH TO RANDOMNESS ORACLE METADATA WASM"
KEYRING_MNEMONIC = "MNEMONIC SEED PHRASE"
```

1. Open terminal and write: `yarn`.
2. For starting in dev mode write: `yarn start:dev`.
3. For production, use: `yarn build`.
4. Then: `yarn start`.
