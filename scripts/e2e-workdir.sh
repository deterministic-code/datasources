#!/bin/sh
set -e
find /work -mindepth 1 -delete
if [ -e /repo/typescript/datasources/package.json ]; then
  cp -a /repo/. /work
else
  cp -a /opt/src/. /work
fi
cd "${E2E_WORKDIR:-/work}"
exec "$@"
