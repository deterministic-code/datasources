set -e
repo="$(cd "$(dirname "$0")/.." && pwd)"
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
down_flags="down -v --remove-orphans -t 0"
parallel="${E2E_PARALLEL:-3}"
fail=0
live=""

down_one() {
  project=$1
  file=$2
  COMPOSE_PROJECT_NAME=$project docker compose -f "$file" $down_flags
}

maybe_prune() {
  [ "${E2E_PRUNE:-}" = 1 ] || return 0
  docker builder prune -af
  docker system prune -f
}

e2e_log() {
  printf 'e2e-docker: %s %s\n' "$(date +%H:%M:%S)" "$*"
}

e2e_step() {
  label=$1
  shift
  start=$(date +%s)
  e2e_log "start $label"
  "$@"
  e2e_log "done $label $(($(date +%s) - start))s"
}

build_image() {
  tag=$1
  file=$2
  if [ -n "${GITHUB_ACTIONS:-}" ]; then
    e2e_step "image $tag" docker buildx build --load \
      --cache-from "type=gha,scope=$tag" \
      --cache-to "type=gha,mode=max,scope=$tag" \
      -t "$tag" -f "$repo/scripts/$file" "$repo/scripts"
  else
    e2e_step "image $tag" docker build -t "$tag" -f "$repo/scripts/$file" "$repo/scripts"
  fi
}

build_toolchains() {
  node= dotnet= rust=
  for dir in "$@"; do
    case "$dir" in
      samples/typescript/*) node=1 ;;
      samples/csharp/*) dotnet=1 ;;
      samples/rust/*) rust=1 ;;
    esac
  done
  # CLI install is Node. Always build the node image so warmup can run once.
  if [ -n "$node$dotnet$rust" ]; then build_image datasources-e2e-node e2e-node.Dockerfile; fi
  if [ -n "$dotnet" ]; then build_image datasources-e2e-dotnet e2e-dotnet.Dockerfile; fi
  if [ -n "$rust" ]; then build_image datasources-e2e-rust e2e-rust.Dockerfile; fi
}

warmup_cli() {
  mkdir -p "${HOME}/.cache/datasources-e2e" "${HOME}/.cache/datasources-npm"
  e2e_step cli-install docker run --rm \
    -v "$repo:/opt/repo:ro" \
    -v "${HOME}/.cache/datasources-npm:/npm-cache" \
    -v "${HOME}/.cache/datasources-e2e:/e2e-cache" \
    -e E2E_CLI_ONLY=1 \
    -e npm_config_cache=/npm-cache \
    -e DATASOURCES_REPO=/opt/repo \
    datasources-e2e-node
}

if [ "${1:-}" = --local ]; then
  shift
fi
quickstarts=""
if [ "${1:-}" = --quickstarts ]; then
  quickstarts=1
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

run_runner() {
  docker compose run --rm --no-deps -v "$repo:/opt/repo:ro" runner "$@"
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
    export COMPOSE_PROJECT_NAME=$project
    start=$(date +%s)
    e2e_log "start $dir"
    docker compose up -d --wait mysql postgres
    run_runner sh -ec '
      start=$(date +%s)
      printf "e2e-workdir: start sqlite\n"
      sh -e sqlite.sh --local
      printf "e2e-workdir: done sqlite %ss\n" "$(($(date +%s) - start))"
      start=$(date +%s)
      printf "e2e-workdir: start mysql-postgres\n"
      sh -e mysql-postgres.sh --local
      printf "e2e-workdir: done mysql-postgres %ss\n" "$(($(date +%s) - start))"
    '
    e2e_log "done $dir $(($(date +%s) - start))s"
  )
}

if [ -n "$quickstarts" ]; then
  parallel=1
  down() {
    for language in typescript csharp rust; do
      down_one "e2e-quickstart-$language" "$repo/quick_starts/contacts_cli/$language/compose.yml"
    done
    maybe_prune
  }
  trap down EXIT
  build_image datasources-e2e-node e2e-node.Dockerfile
  build_image datasources-e2e-dotnet e2e-dotnet.Dockerfile
  build_image datasources-e2e-rust e2e-rust.Dockerfile
  for language in typescript csharp rust; do
    await_slot
    project="e2e-quickstart-$language"
    compose="$repo/quick_starts/contacts_cli/$language/compose.yml"
    (
      cd "$repo/quick_starts/contacts_cli/$language"
      status=0
      export COMPOSE_PROJECT_NAME=$project
      docker compose up -d --wait mysql postgres
      docker compose run --rm --no-deps -v "$repo:/repo:ro" -e E2E_LANG="$language" runner sh /e2e-quickstarts.sh || status=$?
      down_one "$project" "$compose"
      exit $status
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
if [ -z "${E2E_PARALLEL:-}" ]; then
  rust_only=1
  for dir in $sample_dirs; do
    case "$dir" in
      samples/rust/*) ;;
      *) rust_only= ;;
    esac
  done
  [ -n "$rust_only" ] && parallel=1
fi
down() {
  for dir in $sample_dirs; do
    down_one "e2e-$(slug "$dir")" "$repo/$dir/compose.yml"
  done
  maybe_prune
}
trap down EXIT

suite_start=$(date +%s)
e2e_log "suite start"
# shellcheck disable=SC2086
build_toolchains $sample_dirs
warmup_cli

for dir in "$@"; do
  await_slot
  project="e2e-$(slug "$dir")"
  compose="$repo/$dir/compose.yml"
  (
    status=0
    run_compose "$dir" "$project" || status=$?
    e2e_step "down $dir" down_one "$project" "$compose"
    exit $status
  ) &
  live="$live $!"
done
await_all
e2e_log "suite done $(($(date +%s) - suite_start))s"
exit "$fail"
