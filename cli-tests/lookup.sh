#!/bin/bash
cargo run -p solana-cli -- --config "${1:?}" --verbose address-lookup-table create
