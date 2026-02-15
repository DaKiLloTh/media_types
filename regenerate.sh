#!/bin/bash
set -e

echo "Running generator..."
# Examples automatically have access to dev-dependencies
cargo run --example generate_mime_types

echo "Building library..."
cargo build --lib

echo "Running tests..."
cargo test --lib

echo "Done!"





