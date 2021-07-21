#!/bin/sh

cargo fmt
cargo test --workspace
cargo readme --output README.md
