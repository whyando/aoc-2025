#!/bin/sh
set -e

cargo build --release

export AOC_DAY=$1
export AOC_INPUT=$(./scripts/input_path.sh)

hyperfine './target/x86_64-unknown-linux-musl/release/aoc-2025' --warmup 100 -N
# hyperfine './target/release/test' --warmup 100 -N
