#!/bin/sh
set -e
find /work -mindepth 1 -delete

copy_tree() {
  from="$1"
  to="$2"
  [ -d "$from" ] || return 0
  mkdir -p "$to"
  tar -C "$from" --exclude=target --exclude=node_modules --exclude=bin --exclude=obj --exclude=.git -cf - . | tar -C "$to" -xf -
}

src="${DATASOURCES_REPO:-/opt/repo}"
if [ -d "$src/rust" ] || [ -d "$src/typescript" ] || [ -d "$src/csharp" ] || [ -d "$src/cli" ]; then
  copy_tree "$src/rust" /work/rust
  copy_tree "$src/typescript" /work/typescript
  copy_tree "$src/csharp" /work/csharp
  copy_tree "$src/cli" /work/cli
  mkdir -p /work/samples/run/app
  cp -a /opt/src/. /work/samples/run/app
  cd /work/samples/run/app
  if [ -n "${DATASOURCES_REPO:-}" ]; then
    export DATASOURCES_REPO=/work
    npm install --prefix /work/samples/run/app "file:/work/cli"
    node /work/samples/run/app/node_modules/@deterministic-code/datasources-cli/scripts/build-generators.mjs
  fi
else
  cp -a /opt/src/. /work
  cd /work
fi
exec "$@"
