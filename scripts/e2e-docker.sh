set -e
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
down_flags="down -v --remove-orphans -t 0"
fail=0
live=""
rust_live=""
live_projects=""
sample_dirs=""
suite_pid=$$

down_one() {
  project=$1
  file=$2
  COMPOSE_PROJECT_NAME=$project docker compose -f "$file" $down_flags
}

kill_suite() {
  if [ $$ -ne "$suite_pid" ]; then
    kill -USR1 "$suite_pid" 2>/dev/null || true
    exit 1
  fi
  trap - USR1 EXIT
  e2e_log "fail $*"
  for pid in $live; do
    kill "$pid" 2>/dev/null || true
  done
  for entry in $live_projects; do
    project=${entry%%:*}
    file=${entry#*:}
    down_one "$project" "$file"
  done
  for pid in $live; do
    kill -9 "$pid" 2>/dev/null || true
  done
  wait 2>/dev/null || true
  exit 1
}
trap 'kill_suite signal' USR1

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
      samples/typescript/*|quick_starts/contacts_cli/typescript) node=1 ;;
      samples/csharp/*|quick_starts/contacts_cli/csharp) dotnet=1 ;;
      samples/rust/*|quick_starts/contacts_cli/rust) rust=1 ;;
    esac
  done
  # CLI install is Node.
  if [ -n "$node$dotnet$rust" ]; then build_image datasources-e2e-node e2e-node.Dockerfile; fi
  if [ -n "$dotnet" ]; then build_image datasources-e2e-dotnet e2e-dotnet.Dockerfile; fi
  if [ -n "$rust" ]; then build_image datasources-e2e-rust e2e-rust.Dockerfile; fi
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

prune_live() {
  kept=""
  n=0
  for pid in $1; do
    if kill -0 "$pid" 2>/dev/null; then
      kept="$kept $pid"
      n=$((n + 1))
    else
      wait "$pid" || kill_suite "sample exited $?"
    fi
  done
}

await_slot() {
  waiting=$1
  [ "$fail" -eq 0 ] || kill_suite "already failed"
  while :; do
    prune_live "$live"
    live=$kept
    running=$n
    rust_kept=""
    rust_n=0
    for pid in $rust_live; do
      if kill -0 "$pid" 2>/dev/null; then
        rust_kept="$rust_kept $pid"
        rust_n=$((rust_n + 1))
      fi
    done
    rust_live=$rust_kept
    case "$waiting" in
      samples/rust/*|quick_starts/contacts_cli/rust)
        [ "$rust_n" -eq 0 ] && [ "$running" -lt "$parallel" ] && return
        ;;
      *)
        [ "$running" -lt "$parallel" ] && return
        ;;
    esac
    sleep 1
  done
}

await_all() {
  for pid in $live; do
    wait "$pid" || kill_suite "sample exited $?"
  done
  live=""
}

run_watched() {
  status_file=/tmp/e2e-watch.$$.$RANDOM
  line=0
  (
    "$@" 2>&1
    echo $? >"$status_file"
  ) | e2e_watch || line=$?
  cmd=$(cat "$status_file" 2>/dev/null || echo 1)
  rm -f "$status_file"
  if [ "$line" -ne 0 ]; then
    return 1
  fi
  return "$cmd"
}

run_compose() {
  dir="$1"
  project="$2"
  if [ ! -f "$repo/$dir/compose.yml" ]; then
    printf 'e2e-docker: %s has no compose.yml\n' "$dir" >&2
    return 1
  fi
  cd "$repo/$dir"
  export COMPOSE_PROJECT_NAME=$project
  start=$(date +%s)
  e2e_log "start $dir"
  built=0
  run_watched docker compose build app || built=$?
  if [ "$built" -ne 0 ]; then
    return "$built"
  fi
  up=0
  run_watched docker compose up -d --wait mysql postgres || up=$?
  if [ "$up" -ne 0 ]; then
    return "$up"
  fi
  run=0
  run_watched docker compose run --rm --no-deps app sh sqlite.sh || run=$?
  if [ "$run" -ne 0 ]; then
    return "$run"
  fi
  run=0
  run_watched docker compose run --rm --no-deps app sh mysql-postgres.sh || run=$?
  if [ "$run" -ne 0 ]; then
    return "$run"
  fi
  e2e_log "done $dir $(($(date +%s) - start))s"
  return 0
}

if [ -n "$quickstarts" ]; then
  set -- \
    quick_starts/contacts_cli/typescript \
    quick_starts/contacts_cli/csharp \
    quick_starts/contacts_cli/rust
elif [ $# -eq 0 ]; then
  # shellcheck disable=SC2086
  set -- $samples
fi

sample_dirs="$*"
rust_only=1
for dir in $sample_dirs; do
  case "$dir" in
    samples/rust/*|quick_starts/contacts_cli/rust) ;;
    *) rust_only= ;;
  esac
done
if [ -n "${E2E_PARALLEL:-}" ]; then
  parallel=$E2E_PARALLEL
elif [ -n "$rust_only" ]; then
  parallel=1
else
  parallel=3
fi
down() {
  for dir in $sample_dirs; do
    down_one "e2e-$(slug "$dir")" "$repo/$dir/compose.yml"
  done
  maybe_prune
}
trap 'down; trap - USR1' EXIT

suite_start=$(date +%s)
e2e_log "suite start"
mkdir -p "${HOME}/.cache/datasources-npm" "${HOME}/.cache/datasources-nuget" "${HOME}/.cache/datasources-cargo"
# shellcheck disable=SC2086
build_toolchains $sample_dirs

for dir in "$@"; do
  await_slot "$dir"
  project="e2e-$(slug "$dir")"
  compose="$repo/$dir/compose.yml"
  live_projects="$live_projects $project:$compose"
  (
    run=0
    ( run_compose "$dir" "$project" ) || run=$?
    if [ "$run" -ne 0 ]; then
      kill_suite "$dir $run"
    fi
    e2e_step "down $dir" down_one "$project" "$compose"
    exit 0
  ) &
  live="$live $!"
  case "$dir" in
    samples/rust/*|quick_starts/contacts_cli/rust) rust_live="$rust_live $!" ;;
  esac
done
await_all
e2e_log "suite done $(($(date +%s) - suite_start))s"
exit "$fail"
