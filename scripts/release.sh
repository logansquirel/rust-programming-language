#!/bin/bash
# compile (release) + link
cargo build --quiet --release --workspace --all-targets --all-features
