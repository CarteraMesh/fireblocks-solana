#!/bin/bash
name=$(basename "$0")
cargo run -p solana-cli -- --config "${1:?}" create-nonce-account --verbose  ./scripts/"${name%.sh}".json 0.01
