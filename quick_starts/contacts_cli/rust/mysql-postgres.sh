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
if [ -n "${DATASOURCES_PACKS:-}" ]; then
  packs=$(cd "$DATASOURCES_PACKS" && pwd)
  mkdir -p .cargo
  {
    echo "[patch.crates-io]"
    for dir in "$packs"/crates/*; do
      [ -d "$dir" ] || continue
      name=$(basename "$dir" | sed -E 's/-[0-9]+\.[0-9]+\.[0-9]+$//')
      echo "$name = { path = \"$dir\" }"
    done
  } > .cargo/config.toml
  npm install "$packs"/deterministic-code-datasources-cli-*.tgz
fi
cargo add datasources
npx @deterministic-code/datasources-cli setup --yes mysql postgres
if [ -n "${DATASOURCES_PACKS:-}" ]; then
  npm install "$(cd "$DATASOURCES_PACKS" && pwd)"/deterministic-code-datasources-cli-*.tgz
fi
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run | diff -u expected.txt -
export PROVIDER=postgres
export DATASOURCE_URL=postgres://ds:ds@127.0.0.1:5432/datasources
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run | diff -u expected.txt -
