#!/bin/bash

# Stop the script if any command fails
set -e

# Build the project in release mode
echo "Building netmon in release mode..."
cargo build --release

# Check if the build succeeded
if [ ! -f ./target/release/netmon ]; then
  echo "Build failed: netmon binary not found."
  exit 1
fi

# Copy the binary to /usr/local/bin using install (which is safer than cp)
echo "Installing netmon..."
sudo install -m 755 ./target/release/netmon /usr/local/bin/

# Verify installation
echo "Installed successfully:"
/usr/local/bin/netmon version

