#!/bin/sh
set -e
find /work -mindepth 1 -delete
cp -a /opt/src/. /work
if [ -n "${DATASOURCES_REPO:-}" ]; then
  npm install --prefix /work "file:${DATASOURCES_REPO}/cli"
  node /work/node_modules/@deterministic-code/datasources-cli/scripts/build-generators.mjs
fi
cd /work
exec "$@"
