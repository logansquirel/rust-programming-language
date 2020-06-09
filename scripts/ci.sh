#!/bin/bash
# same script than continuous integration
# compile (release) + link + unit test + integration test + benchmarks + examples + doc test + lint checks + format checks
cargo build --verbose --release --workspace --all-targets --all-features
cargo test --verbose --release --workspace --all-targets --all-features
cargo clippy --verbose --workspace --all-targets --all-features -- -D warnings
cargo fmt --verbose --all -- --check --files-with-diff
