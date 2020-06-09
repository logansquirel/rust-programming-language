#!/bin/bash
# compile (debug) + link + unit test + integration test + benchmarks + examples + doc test
cargo test --quiet --workspace --all-targets --all-features
