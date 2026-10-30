# Solana Substreams: Native SOL Transfer Extraction

Substreams module written in Rust to extract native SOL transfers from the Solana blockchain.

## Project Structure
```text
├── Cargo.toml            # Rust dependencies & cdylib configuration
├── substreams.yaml       # Substreams manifest configuration
├── build.rs              # Protobuf build configuration
├── proto/
│   └── transfers.proto   # Protobuf definition (Transfer & Transfers)
├── src/
│   ├── lib.rs            # Core extraction logic (top-level + inner CPI)
│   └── pb/
│       └── mod.rs        # Generated protobuf module bridge
├── output/               # Destination for extracted JSONL results
└── README.md
```

## Prerequisites

1. **Rust & WASM target**:
   ```powershell
   winget install Rustlang.Rustup
   rustup target add wasm32-unknown-unknown
   ```

2. **Substreams CLI**:
   Download the latest release binary from [StreamingFast Substreams Releases](https://github.com/streamingfast/substreams/releases) and place `substreams.exe` in your system `PATH`.

3. **Protobuf Compiler**:
   ```powershell
   winget install protobuf
   ```

## Authentication

Set your StreamingFast or Pinax API token:
```powershell
$env:SUBSTREAMS_API_TOKEN="<your_api_token>"
```
Or run:
```powershell
substreams auth
```

## Build Instructions

1. Generate code and compile WASM:
   ```powershell
   substreams build
   ```
2. Package into `.spkg`:
   ```powershell
   substreams pack substreams.yaml
   ```

## Execution Instructions

Stream transfers from block **385870151** to **385870156** inclusive (stop-block is exclusive in Substreams CLI, so stop block is `385870157`):

```powershell
substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_transfers -s 385870151 -t 385870157 -o jsonl > output/transfers.jsonl
```

Or using Pinax:
```powershell
substreams run -e solana.substreams.pinax.network:443 substreams.yaml map_transfers -s 385870151 -t 385870157 -o jsonl > output/transfers.jsonl
```

