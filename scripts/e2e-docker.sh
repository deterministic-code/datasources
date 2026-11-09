set -e -o pipefail
repo="$(cd "$(dirname "$0")/.." && pwd)"
# shellcheck disable=SC1091
. "$repo/scripts/e2e-fail-fast.sh"

samples="
samples/typescript/contacts-simple-cli
samples/typescript/contacts-crud-enriched-medium-rest
samples/typescript/complex-game-rest
samples/typescript/user-roles-simple-cli
samples/csharp/contacts-simple-cli
samples/csharp/contacts-crud-enriched-medium-rest
samples/csharp/complex-game-rest
samples/csharp/user-roles-simple-cli
samples/rust/contacts-simple-cli
samples/rust/contacts-crud-enriched-medium-rest
samples/rust/complex-game-rest
samples/rust/user-roles-simple-cli
"
quickstarts="
quick_starts/contacts_cli/typescript
quick_starts/contacts_cli/csharp
quick_starts/contacts_cli/rust
"

local=
if [ "${1:-}" = --local ]; then
  local=1
  shift
fi
if [ "${1:-}" = --quickstarts ]; then
  local=
  shift
  # shellcheck disable=SC2086
  set -- $quickstarts
elif [ $# -eq 0 ]; then
  # shellcheck disable=SC2086
  set -- $samples
else
  case "$1" in
    typescript|csharp|rust)
      out=""
      for language in "$@"; do
        for dir in $samples; do
          case "$dir" in
            samples/"$language"/*) out="$out $dir" ;;
          esac
        done
      done
      # shellcheck disable=SC2086
      set -- $out
      ;;
  esac
fi

build_image() {
  docker build -t "$1" -f "$repo/scripts/$2" "$repo/scripts"
}

node= dotnet= rust=
for dir in "$@"; do
  case "$dir" in
    samples/typescript/*|quick_starts/contacts_cli/typescript) node=1 ;;
    samples/csharp/*|quick_starts/contacts_cli/csharp) dotnet=1 ;;
    samples/rust/*|quick_starts/contacts_cli/rust) rust=1 ;;
  esac
done
if [ -n "$node$dotnet$rust" ]; then build_image datasources-e2e-node e2e-node.Dockerfile; fi
if [ -n "$dotnet" ]; then build_image datasources-e2e-dotnet e2e-dotnet.Dockerfile; fi
if [ -n "$rust" ]; then build_image datasources-e2e-rust e2e-rust.Dockerfile; fi

mkdir -p "${HOME}/.cache/datasources-npm" "${HOME}/.cache/datasources-nuget" "${HOME}/.cache/datasources-cargo"

run() {
  dir=$1
  cd "$repo/$dir"
  trap 'docker compose down -v --remove-orphans -t 0' EXIT
  docker compose build app
  docker compose up -d --wait mysql postgres
  if [ -n "$local" ]; then
    docker compose run --rm --no-deps \
      -e DATASOURCES_REPO=/datasources \
      -v "$repo:/datasources" \
      -v /datasources/cli/node_modules \
      app sh -c 'npm install --prefix "$DATASOURCES_REPO/cli" && ln -sf "$DATASOURCES_REPO/cli/bin/datasources.mjs" /usr/local/bin/datasources && sh sqlite.sh --local'
    docker compose run --rm --no-deps \
      -e DATASOURCES_REPO=/datasources \
      -v "$repo:/datasources" \
      -v /datasources/cli/node_modules \
      app sh -c 'npm install --prefix "$DATASOURCES_REPO/cli" && ln -sf "$DATASOURCES_REPO/cli/bin/datasources.mjs" /usr/local/bin/datasources && sh mysql-postgres.sh --local'
  else
    docker compose run --rm --no-deps app sh sqlite.sh
    docker compose run --rm --no-deps app sh mysql-postgres.sh
  fi
}

for dir in "$@"; do
  printf 'e2e-docker: %s %s\n' "$(date +%H:%M:%S)" "start $dir"
  run "$dir" 2>&1 | e2e_watch
  printf 'e2e-docker: %s %s\n' "$(date +%H:%M:%S)" "done $dir"
done
