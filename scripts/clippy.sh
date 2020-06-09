#!/bin/bash
# compile (debug) + lint checks
cargo clippy --quiet --workspace --all-targets --all-features
