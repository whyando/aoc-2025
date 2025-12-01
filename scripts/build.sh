#!/bin/sh

RUSTFLAGS="-C target-cpu=native -C target-feature=+crt-static" cargo build --release
