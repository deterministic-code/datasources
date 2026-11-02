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

if [ $# -gt 0 ] && [ ! -d "$1" ]; then
  trap "$compose down -v --remove-orphans" EXIT
  $compose run --rm --build runner "$@"
  exit
fi

if [ $# -eq 0 ]; then
  # shellcheck disable=SC2086
  set -- $samples
fi

$compose build runner

slug() {
  printf '%s' "$1" | tr '/_' '-'
}

projects=""
for dir in "$@"; do
  projects="$projects e2e-$(slug "$dir")"
done

down() {
  for project in $projects; do
    COMPOSE_PROJECT_NAME=$project $compose down -v --remove-orphans
  done
}
trap down EXIT

fail=0
pids=""
for dir in "$@"; do
  project="e2e-$(slug "$dir")"
  (
    COMPOSE_PROJECT_NAME=$project E2E_SAMPLE=$dir $compose run --rm --no-build runner
  ) &
  pids="$pids $!"
done
for pid in $pids; do
  wait "$pid" || fail=1
done
exit "$fail"
