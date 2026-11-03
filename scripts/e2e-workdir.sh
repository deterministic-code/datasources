#!/bin/sh
set -e
find /work -mindepth 1 -delete

# Quick starts: empty workdir. dest copies quick_starts files, then npx from the registry.
if [ -n "${E2E_LANG:-}" ]; then
  unset DATASOURCES_REPO
  cd /work
  exec "$@"
fi

copy_tree() {
  from="$1"
  to="$2"
  [ -d "$from" ] || return 0
  mkdir -p "$to"
  tar -C "$from" --exclude=target --exclude=node_modules --exclude=bin --exclude=obj --exclude=.git -cf - . | tar -C "$to" -xf -
}

# Samples: independent image. Local checkout on tmpfs at the monorepo paths setup --local uses.
src="${DATASOURCES_REPO:-/opt/repo}"
sample="${SAMPLE:?}"
copy_tree "$src/rust" /work/rust
copy_tree "$src/typescript" /work/typescript
copy_tree "$src/csharp" /work/csharp
copy_tree "$src/cli" /work/cli
mkdir -p "/work/$sample"
cp -a /opt/src/. "/work/$sample"
cd "/work/$sample"
export DATASOURCES_REPO=/work
npm install --prefix /work/cli
npm install --prefix "/work/$sample" "file:/work/cli"
node /work/cli/scripts/build-generators.mjs
exec "$@"
