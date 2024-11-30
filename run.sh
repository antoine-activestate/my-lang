#!/usr/bin/bash

# Exit if any command fails
set -e

cargo fmt
cargo test
cargo build
./target/debug/my-lang
