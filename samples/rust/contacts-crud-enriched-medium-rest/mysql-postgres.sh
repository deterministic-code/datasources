root=${CARGO_TARGET_DIR:-target}
export PROVIDER=mysql
export DATASOURCE_URL="${MYSQL_URL:-mysql://ds:ds@mysql:3306/datasources}"
export CARGO_TARGET_DIR="$root/mysql"
datasources setup --yes "$@" mysql
datasources generate all --dialect rust
cargo run
export PROVIDER=postgres
export DATASOURCE_URL="${POSTGRES_URL:-postgres://ds:ds@postgres:5432/datasources}"
export CARGO_TARGET_DIR="$root/postgres"
datasources setup --yes "$@" postgres
datasources generate all --dialect rust
cargo run
