#!/bin/bash
set -e
echo "Copying binaries and shared objects..."
cp ./build/* /usr/local/lib
cp ./target/release/flux /usr/local/bin
ldconfig /usr/local/lib
echo "Installed successfully!"