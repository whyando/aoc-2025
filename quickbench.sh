#!/bin/sh
set -e

cargo build --release

export AOC_DAY=$1
export AOC_INPUT=$(./scripts/input_path.sh)

hyperfine './target/release/aoc-2025' --warmup 100 -N
