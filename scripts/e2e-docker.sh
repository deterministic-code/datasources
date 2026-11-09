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

use_local=
if [ "${1:-}" = --local ]; then
  use_local=1
  shift
fi
if [ "${1:-}" = --quickstarts ]; then
  use_local=
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

if [ -n "$use_local" ]; then
  parallel=${E2E_PARALLEL:-1}
else
  parallel=${E2E_PARALLEL:-3}
fi

status_dir=$(mktemp -d "${TMPDIR:-/tmp}/e2e-docker.XXXXXX")
running=""
projects=""
token=0

cleanup() {
  status=$?
  trap - EXIT
  set +e
  for entry in $projects; do
    rest=${entry#*:}
    project=${rest%%:*}
    dir=${rest#*:}
    (
      cd "$repo/$dir" || exit 0
      COMPOSE_PROJECT_NAME=$project docker compose down -v --remove-orphans -t 0
    )
  done
  for entry in $running; do
    rest=${entry#*:}
    pid=${rest%%:*}
    kill "$pid" 2>/dev/null
  done
  wait
  rm -rf "$status_dir"
  exit "$status"
}
trap cleanup EXIT

reap() {
  kept=""
  n=0
  rust_n=0
  for entry in $running; do
    token_n=${entry%%:*}
    rest=${entry#*:}
    pid=${rest%%:*}
    kind=${rest#*:}
    if [ -f "$status_dir/$token_n" ]; then
      wait "$pid"
      kept_projects=""
      for project_entry in $projects; do
        if [ "${project_entry%%:*}" != "$token_n" ]; then
          kept_projects="$kept_projects $project_entry"
        fi
      done
      projects=$kept_projects
    else
      kept="$kept $entry"
      n=$((n + 1))
      if [ "$kind" = rust ]; then
        rust_n=$((rust_n + 1))
      fi
    fi
  done
  running=$kept
}

await_slot() {
  waiting=$1
  while :; do
    reap
    case "$waiting" in
      samples/rust/*|quick_starts/contacts_cli/rust)
        [ "$rust_n" -eq 0 ] && [ "$n" -lt "$parallel" ] && return
        ;;
      *)
        [ "$n" -lt "$parallel" ] && return
        ;;
    esac
    sleep 1
  done
}

await_idle() {
  while :; do
    reap
    [ "$n" -eq 0 ] && return
    sleep 1
  done
}

run_scripts() {
  if [ -n "$use_local" ]; then
    docker compose run --rm --no-deps \
      -e DATASOURCES_REPO=/datasources \
      -v "$repo:/datasources" \
      -v /datasources/cli/node_modules \
      app sh -c 'npm install --prefix "$DATASOURCES_REPO/cli" && npm run prepare --prefix "$DATASOURCES_REPO/cli" && ln -sf "$DATASOURCES_REPO/cli/bin/datasources.mjs" /usr/local/bin/datasources && sh sqlite.sh --local && sh mysql-postgres.sh --local'
  else
    docker compose run --rm --no-deps app sh -c 'sh sqlite.sh && sh mysql-postgres.sh'
  fi
}

for dir in "$@"; do
  await_slot "$dir"
  token=$((token + 1))
  project="e2e-$(printf '%s' "$dir" | tr '/_' '-')"
  projects="$projects $token:$project:$dir"
  kind=other
  case "$dir" in
    samples/rust/*|quick_starts/contacts_cli/rust) kind=rust ;;
  esac
  (
    trap - EXIT
    code=0
    (
      trap - EXIT
      cd "$repo/$dir"
      export COMPOSE_PROJECT_NAME=$project
      trap 'status=$?; trap - EXIT; docker compose down -v --remove-orphans -t 0 || true; exit "$status"' EXIT
      start=$(date +%s)
      printf 'e2e-docker: %s start %s\n' "$(date +%H:%M:%S)" "$dir"
      code=0
      {
        docker compose build app || exit $?
        docker compose up -d --wait mysql postgres || exit $?
        run_scripts || exit $?
      } 2>&1 | while IFS= read -r line || [ -n "$line" ]; do
        printf '%s\n' "$dir: $line"
      done | e2e_watch || code=$?
      if [ "$code" -eq 0 ]; then
        printf 'e2e-docker: %s done %s %ss\n' "$(date +%H:%M:%S)" "$dir" "$(($(date +%s) - start))"
      fi
      exit "$code"
    ) || code=$?
    printf '%s\n' "$code" > "$status_dir/$token"
    exit "$code"
  ) &
  running="$running $token:$!:$kind"
done

await_idle
