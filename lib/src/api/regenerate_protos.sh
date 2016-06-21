#!/bin/sh
# Note: This assumes that rust-protobuf has been correctly installed.
# For details follow the instructions here: https://github.com/stepancheg/rust-protobuf
# Note that we require proto3, so using homebrew: `brew install --devel protobuf`

# Remove everything generated except for the mod file, which we need to stick around.
find ./gen ! -name 'mod.rs' -type f -exec rm -f {} +

# Re-generate the proto bindings.
protoc --proto_path ./proto/ --rust_out ./gen/ ./proto/*.proto
