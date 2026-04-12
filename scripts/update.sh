#!/bin/bash
rustup self update
rustup update
cargo clean
cargo update --verbose
#cargo test