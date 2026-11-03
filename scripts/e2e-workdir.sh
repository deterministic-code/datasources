#!/bin/sh
set -e
find /work -mindepth 1 -delete

# Quick starts: empty workdir. dest copies quick_starts files, then npx from the registry.
if [ -n "${E2E_LANG:-}" ]; then
  unset DATASOURCES_REPO
  cd /work
  exec "$@"
fi

# Samples: sample on tmpfs. --local file: and cargo --path read /opt/repo (image or :ro mount).
sample="${SAMPLE:?}"
mkdir -p "/work/$sample"
cp -a /opt/src/. "/work/$sample"
cd "/work/$sample"
export DATASOURCES_REPO=/opt/repo
npm install --install-links "file:/opt/repo/cli"
exec "$@"
