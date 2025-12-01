#!/bin/sh
set -e

padded=$(printf "%02d" "$AOC_DAY")
solution="./target/release/${padded}"

if [ ! -f "$solution" ]; then
    echo "not implemented"
    exit 0
fi

"$solution" < "$AOC_INPUT"
