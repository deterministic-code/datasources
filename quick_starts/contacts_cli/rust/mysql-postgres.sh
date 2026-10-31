unset CARGO_TARGET_DIR
export PROVIDER=mysql
export DATASOURCE_URL=mysql://ds:ds@127.0.0.1:3306/datasources
cargo init
cp -R ../quick_starts/contacts_cli/rust/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
wipe() {
  docker compose down -v --remove-orphans --timeout 30 || true
}
trap wipe EXIT
wipe
docker compose up -d --wait --force-recreate --renew-anon-volumes
cargo add datasources
npx @deterministic-code/datasources-cli setup --yes mysql postgres
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run | diff -u expected.txt -
export PROVIDER=postgres
export DATASOURCE_URL=postgres://ds:ds@127.0.0.1:5432/datasources
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run | diff -u expected.txt -
