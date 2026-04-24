#!/bin/bash
cargo clippy --fix --allow-dirty -- -A unused
cargo fmt --all