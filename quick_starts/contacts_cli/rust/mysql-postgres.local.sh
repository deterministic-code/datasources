unset CARGO_TARGET_DIR
export COMPOSE_PROJECT_NAME=datasources-contacts-rust
export MYSQL_PUBLISH_PORT="${MYSQL_PUBLISH_PORT:-13306}"
export POSTGRES_PUBLISH_PORT="${POSTGRES_PUBLISH_PORT:-15432}"
export PROVIDER=mysql
export DATASOURCE_URL="mysql://ds:ds@127.0.0.1:${MYSQL_PUBLISH_PORT}/datasources"
cargo init
cp -R ../quick_starts/contacts_cli/rust/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
wipe() {
  docker compose down -v --remove-orphans --timeout 30 || true
  for project in datasources-contacts-rust dest; do
    ids=$(docker ps -aq --filter "label=com.docker.compose.project=${project}")
    if [ -n "$ids" ]; then
      docker rm -f $ids || true
    fi
  done
}
refuse_port() {
  holder=$(docker ps -q --filter "publish=$1")
  if [ -n "$holder" ]; then
    echo "port $1 is published by another container" >&2
    docker ps --filter "publish=$1" >&2
    exit 1
  fi
}
trap wipe EXIT
wipe
refuse_port "$MYSQL_PUBLISH_PORT"
refuse_port "$POSTGRES_PUBLISH_PORT"
docker compose up -d --wait --force-recreate --renew-anon-volumes
cargo add --path ../rust/datasources
cargo add --path ../rust/providers/mysql
cargo add --path ../rust/providers/postgres
cargo add tokio --features macros,rt-multi-thread
node ../cli/bin/datasources.mjs setup --yes mysql postgres
node ../cli/bin/datasources.mjs generate all --dialect rust
cargo run | diff -u expected.txt -
export PROVIDER=postgres
export DATASOURCE_URL="postgres://ds:ds@127.0.0.1:${POSTGRES_PUBLISH_PORT}/datasources"
node ../cli/bin/datasources.mjs generate all --dialect rust
cargo run | diff -u expected.txt -
