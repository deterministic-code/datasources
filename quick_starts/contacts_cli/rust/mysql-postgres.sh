export PROVIDER=mysql
export DATASOURCE_URL="${MYSQL_URL:-mysql://ds:ds@mysql:3306/datasources}"
cargo init
cp -R ../quick_starts/contacts_cli/rust/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
cargo add datasources
npx @deterministic-code/datasources-cli setup --yes mysql postgres
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run | diff -u expected.txt -
export PROVIDER=postgres
export DATASOURCE_URL="${POSTGRES_URL:-postgres://ds:ds@postgres:5432/datasources}"
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run | diff -u expected.txt -
