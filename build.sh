#!/bin/bash
set -e

export RUSTUP_DIST_SERVER="https://mirrors.ustc.edu.cn/rust-static"
export RUSTUP_UPDATE_ROOT="https://mirrors.ustc.edu.cn/rust-static/rustup"

mkdir -p ~/.cargo
cat > ~/.cargo/config.toml << 'EOF'
[source.crates-io]
replace-with = "ustc"
[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"
[net]
retry = 5
EOF

rustup target add wasm32-unknown-unknown

echo "=== BUILDING ==="
rm -f Cargo.lock
cargo build --target wasm32-unknown-unknown --release

echo "=== BUILD COMPLETE ==="
ls -lh target/wasm32-unknown-unknown/release/solana_transfers.wasm
