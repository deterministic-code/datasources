unset CARGO_TARGET_DIR
export PROVIDER=mysql
export DATASOURCE_URL="${MYSQL_URL:-mysql://ds:ds@mysql:3306/datasources}"
npx @deterministic-code/datasources-cli setup --yes "$@" sqlite mysql postgres
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run -- add-role admin
export PROVIDER=postgres
export DATASOURCE_URL="${POSTGRES_URL:-postgres://ds:ds@postgres:5432/datasources}"
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run -- add-role admin
