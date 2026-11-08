#!/bin/sh
set -e

export npm_config_cache=/tmp/npm-cache
mkdir -p "$npm_config_cache"

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
  (cd "$1" && tar cf - \
    --exclude=node_modules --exclude=target --exclude=bin --exclude=obj \
    --exclude=.env --exclude=generated --exclude=sql --exclude=app.sqlite \
    --exclude=.migraters.yaml .) | tar xf - -C "$2"
}

install_cli() {
  rm -rf /tmp/repo
  mkdir -p /tmp/repo
  (cd /opt/repo && tar cf - \
    --exclude=node_modules --exclude=target --exclude=obj --exclude=.git --exclude=.env \
    cli typescript rust csharp) | tar xf - -C /tmp/repo
  (cd /tmp/repo/cli && npm install --install-links)
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
if [ -d /sample ]; then
  src=/sample
else
  src="/opt/repo/$sample"
  if [ ! -d "$src" ]; then
    printf 'e2e-workdir: missing %s\n' "$src" >&2
    exit 1
  fi
fi
e2e_step copy copy_tree "$src" /work
e2e_step cli-install install_cli
export DATASOURCES_REPO=/tmp/repo
export PATH="/tmp/repo/cli/bin:/tmp/repo/cli/node_modules/.bin:$PATH"
cd /work
printf 'e2e-workdir: start scripts\n'
start=$(date +%s)
case "$sample" in
  samples/csharp/*)
    ( flock 8; "$@" ) 8>/e2e-cache/dotnet.lock
    ;;
  samples/typescript/*)
    ( flock 7; "$@" ) 7>/e2e-cache/npm-build.lock
    ;;
  samples/rust/*)
    export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
    case "${CARGO_TARGET_DIR:-}" in
      /cargo-target*)
        export CARGO_TARGET_DIR="/tmp${CARGO_TARGET_DIR}"
        mkdir -p "$CARGO_TARGET_DIR"
        ;;
    esac
    "$@"
    ;;
  *)
    "$@"
    ;;
esac
printf 'e2e-workdir: done scripts %ss\n' "$(($(date +%s) - start))"
