#!/bin/bash

set -eux;

cargo fmt
cargo build --all
cargo build --all --release

exit 0