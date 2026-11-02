set -e
compose="docker compose -f docker/e2e/compose.yml"
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

lang() {
  case "$1" in
    samples/typescript/*|typescript) printf typescript ;;
    samples/csharp/*|csharp) printf csharp ;;
    samples/rust/*|rust) printf rust ;;
    *)
      echo "unknown e2e language for $1" >&2
      exit 1
      ;;
  esac
}

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

if [ $# -gt 0 ] && [ ! -d "$1" ]; then
  projects="e2e-quickstart-typescript e2e-quickstart-csharp e2e-quickstart-rust"
  down() {
    for project in $projects; do
      COMPOSE_PROJECT_NAME=$project $compose $down_flags
    done
  }
  trap down EXIT
  for language in typescript csharp rust; do
    await_slot
    project="e2e-quickstart-$language"
    (
      COMPOSE_PROJECT_NAME=$project E2E_LANG=$language $compose run --rm --build runner "$@"
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

projects=""
for dir in "$@"; do
  projects="$projects e2e-$(slug "$dir")"
done

down() {
  for project in $projects; do
    COMPOSE_PROJECT_NAME=$project $compose $down_flags
  done
}
trap down EXIT

for dir in "$@"; do
  await_slot
  project="e2e-$(slug "$dir")"
  language="$(lang "$dir")"
  (
    COMPOSE_PROJECT_NAME=$project E2E_SAMPLE=$dir E2E_LANG=$language $compose run --rm --build runner
  ) &
  live="$live $!"
done
await_all
exit "$fail"
