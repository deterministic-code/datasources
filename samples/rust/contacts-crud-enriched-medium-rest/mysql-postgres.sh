export PROVIDER=mysql
export DATASOURCE_URL="${MYSQL_URL:-mysql://ds:ds@mysql:3306/datasources}"
datasources setup --yes "$@" mysql
datasources generate all --dialect rust
cargo run
export PROVIDER=postgres
export DATASOURCE_URL="${POSTGRES_URL:-postgres://ds:ds@postgres:5432/datasources}"
datasources setup --yes "$@" postgres
datasources generate all --dialect rust
cargo run
