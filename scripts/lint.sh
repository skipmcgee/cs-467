#!/bin/bash
cargo clippy --fix --allow-dirty --all-targets --all-features -- -D warnings -A clippy::too_many_arguments #-W clippy::pedantic
cargo fmt --all
#cargo test