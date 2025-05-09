# 📊 PagCrypto Dashboard Substreams

This project uses [The Graph Substreams](https://thegraph.com) to index transactions on the Solana blockchain that contain the memo `paycrypto:{hash}`, representing operations of the **PagCrypto** protocol.

## What does he do?

- Scans all Solana blocks for transactions with PagCrypto's standard memo.
- Extracts:
  - `signature`
  - `slot`
  - `memo`
  - `fee`
  - `success` (indicates whether the transaction failed or not)
- Exposes data for dashboards, analytics or consumption by AI/alerts.

## How to run

### 1. Requirements

- `Rust` + `cargo`
- `substreams` CLI  
  Install with:
  ```bash
  curl -s https://get.substreams.dev | bash
  ```

### 2. Clone and prepare

```bash
git clone https://github.com/pagcrypto/pagcrypto-dashboard-substreams.git
cd pagcrypto-dashboard-substreams
```

### 3. Generate Protobuf types

```bash
substreams protogen
```

### 4. Compile the project

```bash
cargo build --release
```

### 5. Generate the `.spkg` package

```bash
substreams package
```

### 6. Run locally

```bash
substreams run map_pagcrypto_tx \
  --start-block 0 \
  --stop-block +100 \
  --output json
```

## Usage

```bash
substreams build
substreams auth
substreams gui       			  # Get streaming!
```

Optionally, you can publish your Substreams to the [Substreams Registry](https://substreams.dev).

```bash
substreams registry login         # Login to substreams.dev
substreams registry publish       # Publish your Substreams to substreams.dev
```

## Project Structure

```
.
├── Cargo.toml
├── proto/
│   └── pagcrypto.proto
├── src/
│   └── lib.rs
├── substreams.yaml
├── README.md
```

## Deployment

1. Publish the generated `.spkg` to [https://substreams.dev](https://substreams.dev).
2. Use a sink like:
    - `substreams-sink-postgres`
    - `substreams-sink-sqlite`
    - `substreams-sink-grpc` for custom backend

## Contribution

Pull requests are welcome. For more information, contact the [PagCrypto](https://pagcrypto.finance) team.

## 🧾 Licença

MIT
