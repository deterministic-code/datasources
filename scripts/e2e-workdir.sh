#!/bin/sh
set -e
find /work -mindepth 1 -delete

# Quick starts: empty workdir. dest copies quick_starts files, then npx from the registry.
if [ -n "${E2E_LANG:-}" ]; then
  unset DATASOURCES_REPO
  cd /work
  exec "$@"
fi

# Samples: sample on tmpfs. --local file: and cargo --path read /opt/repo.
sample="${SAMPLE:?}"
src="/opt/repo/$sample"
if [ ! -d "$src" ]; then
  printf 'e2e-workdir: missing %s\n' "$src" >&2
  exit 1
fi
mkdir -p "/work/$sample"
(cd "$src" && tar cf - \
  --exclude=node_modules \
  --exclude=target \
  --exclude=bin \
  --exclude=obj \
  --exclude=.env \
  .) | tar xf - -C "/work/$sample"
cache="/e2e-cache/$sample"
mkdir -p "$cache/bin" "$cache/obj" "$cache/node_modules"
ln -sfn "$cache/bin" "/work/$sample/bin"
ln -sfn "$cache/obj" "/work/$sample/obj"
ln -sfn "$cache/node_modules" "/work/$sample/node_modules"
cd "/work/$sample"
export DATASOURCES_REPO=/opt/repo
npm install --install-links "file:/opt/repo/cli"
exec "$@"
