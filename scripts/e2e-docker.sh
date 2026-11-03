set -e
repo="$(cd "$(dirname "$0")/.." && pwd)"
samples="
samples/typescript/contacts-simple-cli
samples/typescript/contacts-crud-enriched-medium-rest
samples/typescript/user-roles-simple-cli
samples/csharp/contacts-simple-cli
samples/csharp/contacts-crud-enriched-medium-rest
samples/csharp/user-roles-simple-cli
samples/rust/contacts-simple-cli
samples/rust/contacts-crud-enriched-medium-rest
samples/rust/user-roles-simple-cli
"
down_flags="down -v --remove-orphans --rmi local -t 0"
parallel="${E2E_PARALLEL:-3}"
fail=0
live=""
local=""
if [ "${1:-}" = --local ]; then
  local=--local
  shift
fi

case "${1:-}" in
  typescript|csharp|rust)
    out=""
    for language in "$@"; do
      case "$language" in
        typescript|csharp|rust) ;;
        *)
          printf 'e2e-docker: expected a language or sample path, got %s\n' "$language" >&2
          exit 1
          ;;
      esac
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

slug() {
  printf '%s' "$1" | tr '/_' '-'
}

await_slot() {
  while :; do
    next=""
    n=0
    for pid in $live; do
      if kill -0 "$pid" 2>/dev/null; then
        next="$next $pid"
        n=$((n + 1))
      else
        wait "$pid" || fail=1
      fi
    done
    live=$next
    [ "$n" -lt "$parallel" ] && return
    sleep 1
  done
}

await_all() {
  for pid in $live; do
    wait "$pid" || fail=1
  done
  live=""
}

run_compose() {
  dir="$1"
  project="$2"
  if [ ! -f "$repo/$dir/compose.yml" ]; then
    printf 'e2e-docker: %s has no compose.yml\n' "$dir" >&2
    exit 1
  fi
  (
    cd "$repo/$dir"
    if [ -n "$local" ]; then
      COMPOSE_PROJECT_NAME=$project docker compose run --rm --build -v "$repo:/repo:ro" -e DATASOURCES_REPO=/repo runner \
        sh -ec "sh -e sqlite.sh --local && sh -e mysql-postgres.sh --local"
    else
      COMPOSE_PROJECT_NAME=$project docker compose run --rm --build runner
    fi
  )
}

if [ $# -gt 0 ] && [ ! -d "$1" ]; then
  down() {
    for language in typescript csharp rust; do
      COMPOSE_PROJECT_NAME=e2e-quickstart-$language docker compose -f "$repo/samples/$language/contacts-simple-cli/compose.yml" $down_flags
    done
  }
  trap down EXIT
  for language in typescript csharp rust; do
    await_slot
    project="e2e-quickstart-$language"
    (
      cd "$repo/samples/$language/contacts-simple-cli"
      COMPOSE_PROJECT_NAME=$project E2E_LANG=$language docker compose run --rm --build -v "$repo:/repo:ro" -e E2E_LANG runner \
        sh /repo/scripts/e2e-quickstarts.sh
    ) &
    live="$live $!"
  done
  await_all
  exit "$fail"
fi

if [ $# -eq 0 ]; then
  # shellcheck disable=SC2086
  set -- $samples
fi

sample_dirs="$*"
down() {
  for dir in $sample_dirs; do
    project="e2e-$(slug "$dir")"
    COMPOSE_PROJECT_NAME=$project docker compose -f "$repo/$dir/compose.yml" $down_flags
  done
}
trap down EXIT

for dir in "$@"; do
  await_slot
  project="e2e-$(slug "$dir")"
  (
    run_compose "$dir" "$project"
  ) &
  live="$live $!"
done
await_all
exit "$fail"
