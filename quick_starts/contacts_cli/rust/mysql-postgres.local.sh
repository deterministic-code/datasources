unset CARGO_TARGET_DIR
export PROVIDER=mysql
export DATASOURCE_URL=mysql://ds:ds@127.0.0.1:3306/datasources
cargo init
cp -R ../quick_starts/contacts_cli/rust/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
docker compose down --remove-orphans
docker compose up -d --wait --force-recreate
cargo add --path ../rust/datasources
cargo add --path ../rust/providers/mysql
cargo add --path ../rust/providers/postgres
cargo add tokio --features macros,rt-multi-thread
node ../cli/bin/datasources.mjs setup --yes mysql postgres
node ../cli/bin/datasources.mjs generate all --dialect rust
cargo run | diff -u expected.txt -
export PROVIDER=postgres
export DATASOURCE_URL=postgres://ds:ds@127.0.0.1:5432/datasources
node ../cli/bin/datasources.mjs generate all --dialect rust
cargo run | diff -u expected.txt -
docker compose down -v
