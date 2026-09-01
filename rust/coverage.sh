#!/bin/sh
set -e
if command -v cargo-llvm-cov >/dev/null 2>&1; then
  cargo llvm-cov --workspace --fail-under-lines 100
else
  echo "cargo-llvm-cov not installed; running cargo test"
  RUSTFLAGS="-D warnings" cargo test --workspace
fi
