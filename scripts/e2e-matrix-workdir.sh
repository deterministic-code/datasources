#!/bin/sh
set -e
find /work -mindepth 1 -delete
mkdir -p /work
(cd /opt/repo && tar cf - --exclude=node_modules --exclude=target --exclude=obj --exclude=.git --exclude=.env .) | tar xf - -C /work
cd /work
exec node scripts/e2e-matrix.mjs
