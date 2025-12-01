#!/bin/sh
set -e

./scripts/build.sh
export AOC_DAY=$1
export AOC_INPUT=$(./scripts/input_path.sh)

hyperfine './target/release/aoc-2025'
