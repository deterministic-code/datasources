#!/bin/sh
set -e

e2e_step() {
  label=$1
  shift
  start=$(date +%s)
  printf 'e2e-workdir: start %s\n' "$label"
  "$@"
  printf 'e2e-workdir: done %s %ss\n' "$label" "$(($(date +%s) - start))"
}

copy_tree() {
  mkdir -p "$2"
  (cd "$1" && tar cf - --exclude=node_modules --exclude=target --exclude=bin --exclude=obj --exclude=.env .) | tar xf - -C "$2"
}

install_cli() {
  dest=/e2e-cache/cli
  if [ -d "$dest/node_modules/tsx" ] && [ -f "$dest/node_modules/@deterministic-code/deterministic-generators-common/typescript/parser/dist/specification-parser.js" ]; then
    return 0
  fi
  copy_tree /opt/repo/cli "$dest"
  node /e2e-abs-file-deps.mjs "$dest"
  (
    cd "$dest"
    npm install --install-links --ignore-scripts
    node node_modules/esbuild/install.js
    node scripts/build-generators.mjs
  )
}

find /work -mindepth 1 -delete

# Quick starts: empty workdir. dest copies quick_starts files, then npx from the registry.
if [ -n "${E2E_LANG:-}" ]; then
  unset DATASOURCES_REPO
  cd /work
  exec "$@"
fi

# Sample on /work. Local packages are file:/opt/repo/..., same prefix as setup --local.
sample="${SAMPLE:?}"
src="/opt/repo/$sample"
if [ ! -d "$src" ]; then
  printf 'e2e-workdir: missing %s\n' "$src" >&2
  exit 1
fi
e2e_step copy copy_tree "$src" /work
export DATASOURCES_REPO=/opt/repo
node /e2e-abs-file-deps.mjs /work
cache="/e2e-cache/$sample"
mkdir -p "$cache/bin" "$cache/obj"
ln -sfn "$cache/bin" /work/bin
ln -sfn "$cache/obj" /work/obj
if [ -d "$cache/node_modules" ]; then
  mkdir -p /work/node_modules
  cp -a "$cache/node_modules/." /work/node_modules/
fi
cd /work
e2e_step cli-install install_cli
mkdir -p node_modules/@deterministic-code
ln -sfn /e2e-cache/cli node_modules/@deterministic-code/datasources-cli
if [ -f package.json ]; then
  e2e_step npm-install npm install --install-links --ignore-scripts
  if [ -d node_modules/better-sqlite3 ]; then
    e2e_step rebuild-sqlite npm rebuild better-sqlite3
  fi
  mkdir -p "$cache/node_modules"
  cp -a node_modules/. "$cache/node_modules/"
fi
printf 'e2e-workdir: start scripts\n'
start=$(date +%s)
"$@"
printf 'e2e-workdir: done scripts %ss\n' "$(($(date +%s) - start))"
