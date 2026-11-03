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

cli_ready() {
  dest=/e2e-cache/repo/cli
  [ -f /e2e-cache/repo/.ready ] || return 1
  [ -f "$dest/bin/datasources.mjs" ] || return 1
  [ -d "$dest/node_modules/tsx" ] || return 1
  [ -f "$dest/node_modules/@deterministic-code/deterministic-generators-common/typescript/parser/dist/specification-parser.js" ]
}

install_cli() {
  mkdir -p /e2e-cache
  (
    flock 9
    cli_ready && exit 0
    tmp=/e2e-cache/repo.tmp
    rm -rf "$tmp"
    mkdir -p "$tmp"
    (cd /opt/repo && tar cf - \
      --exclude=node_modules --exclude=target --exclude=obj --exclude=.git --exclude=.env \
      cli typescript rust csharp) | tar xf - -C "$tmp"
    (
      cd "$tmp/cli"
      npm_config_ignore_scripts=true npm install --install-links --ignore-scripts
      node node_modules/esbuild/install.js
      node scripts/build-generators.mjs
    )
    chmod +x "$tmp/cli/bin/datasources" "$tmp/cli/bin/datasources.mjs"
    [ -f "$tmp/cli/bin/datasources.mjs" ]
    [ -d "$tmp/cli/node_modules/tsx" ]
    [ -f "$tmp/cli/node_modules/@deterministic-code/deterministic-generators-common/typescript/parser/dist/specification-parser.js" ]
    rm -rf /e2e-cache/repo
    mv "$tmp" /e2e-cache/repo
    : >/e2e-cache/repo/.ready
  ) 9>/e2e-cache/repo.lock
}

find /work -mindepth 1 -delete

if [ -n "${E2E_CLI_ONLY:-}" ]; then
  e2e_step cli-install install_cli
  exit 0
fi

# Quick starts: empty workdir. dest copies quick_starts files, then npx from the registry.
if [ -n "${E2E_LANG:-}" ]; then
  unset DATASOURCES_REPO
  cd /work
  exec "$@"
fi

sample="${SAMPLE:?}"
src="/opt/repo/$sample"
if [ ! -d "$src" ]; then
  printf 'e2e-workdir: missing %s\n' "$src" >&2
  exit 1
fi
e2e_step copy copy_tree "$src" /work
e2e_step cli-install install_cli
export DATASOURCES_REPO=/e2e-cache/repo
export PATH="/e2e-cache/repo/cli/bin:/e2e-cache/repo/cli/node_modules/.bin:$PATH"
cd /work
if [ -f package.json ]; then
  e2e_step npm-install npm install --install-links --ignore-scripts
  if [ -d node_modules/better-sqlite3 ]; then
    e2e_step rebuild-sqlite npm rebuild better-sqlite3
  fi
fi
printf 'e2e-workdir: start scripts\n'
start=$(date +%s)
"$@"
printf 'e2e-workdir: done scripts %ss\n' "$(($(date +%s) - start))"
