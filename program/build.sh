#!/bin/bash

# Build script for E9th Token Program

echo "Building E9th Token Program..."

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo is not installed. Please install Rust and Cargo first."
    exit 1
fi

# Build the program
echo "Compiling program..."
cargo build-bpf

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "Program binary: target/deploy/e9th_token_program.so"
else
    echo "❌ Build failed!"
    exit 1
fi
