#!/bin/sh
set -e
arch=$(uname -m)
case "$arch" in
  x86_64 | amd64) arch=x86_64 ;;
  aarch64 | arm64) arch=aarch64 ;;
  *)
    printf 'e2e-sccache-install: unsupported arch %s\n' "$arch" >&2
    exit 1
    ;;
esac
ver=0.10.0
curl -fsSL "https://github.com/mozilla/sccache/releases/download/v${ver}/sccache-v${ver}-${arch}-unknown-linux-musl.tar.gz" |
  tar -xz -C /usr/local/bin --strip-components=1 "sccache-v${ver}-${arch}-unknown-linux-musl/sccache"
chmod +x /usr/local/bin/sccache
