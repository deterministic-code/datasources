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

copy_checkout() {
  checkout=$1
  mkdir -p "$checkout"
  (cd /opt/repo && tar cf - \
    --exclude=node_modules --exclude=target --exclude=obj --exclude=.git --exclude=.env \
    cli typescript rust csharp) | tar xf - -C "$checkout"
}

copy_generators() {
  root=$1
  src=${GENERATORS_ROOT:-/opt/projects}
  for name in generators-common generators-typescript generators-csharp generators-rust deterministic-generators-common; do
    if [ ! -d "$src/$name" ]; then
      printf 'e2e-workdir: missing %s/%s\n' "$src" "$name" >&2
      exit 1
    fi
    copy_tree "$src/$name" "$root/$name"
    node -e '
      const fs = require("fs");
      const file = process.argv[1];
      const pkg = JSON.parse(fs.readFileSync(file, "utf8"));
      if (!pkg.scripts) process.exit(0);
      delete pkg.scripts.prepare;
      delete pkg.scripts.prepack;
      delete pkg.scripts.build;
      fs.writeFileSync(file, JSON.stringify(pkg, null, 2) + "\n");
    ' "$root/$name/package.json"
  done
}

drop_deterministic_npm_cache() {
  [ -d /npm-cache ] || return 0
  find /npm-cache \( \
    -iname '*deterministic-code*' -o \
    -iname '*generators-common*' -o \
    -iname '*generators-typescript*' -o \
    -iname '*generators-csharp*' -o \
    -iname '*generators-rust*' -o \
    -iname '*deterministic-generators-common*' \
  \) -prune -exec rm -rf {} +
}

install_cli() {
  dest=$1
  drop_deterministic_npm_cache
  copy_checkout "$dest"
  copy_generators "$(dirname "$dest")"
  cp "$dest/cli/package.local.json" "$dest/cli/package.json"
  rm -f "$dest/cli/package-lock.json"
  (
    cd "$dest/cli"
    printf 'ignore-scripts=true\n' > .npmrc
    npm_config_ignore_scripts=true npm install --install-links --ignore-scripts
    rm -f .npmrc
    node scripts/build-generators.mjs
  )
}

rm -rf /e2e-cache/repo /e2e-cache/repo.tmp /e2e-cache/repo.lock
find /work -mindepth 1 -delete

if [ -n "${E2E_CLI_ONLY:-}" ]; then
  e2e_step cli-install install_cli /tmp/repo
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
e2e_step cli-install install_cli /work/repo
export DATASOURCES_REPO=/work/repo
export PATH="/work/repo/cli/bin:/work/repo/cli/node_modules/.bin:$PATH"
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
