set -e
compose="docker compose -f docker/e2e/compose.yml"
trap "$compose down -v --remove-orphans" EXIT
$compose run --rm --build runner "$@"
