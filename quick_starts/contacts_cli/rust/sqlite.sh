unset CARGO_TARGET_DIR
export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
cargo init
cp -R ../quick_starts/contacts_cli/rust/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
if [ -n "${DATASOURCES_PACKS:-}" ]; then
  packs=$(cd "$DATASOURCES_PACKS" && pwd)
  mkdir -p .cargo
  {
    echo "[patch.crates-io]"
    for dir in "$packs"/crates/*; do
      [ -d "$dir" ] || continue
      name=$(basename "$dir" | sed -E 's/-[0-9]+\.[0-9]+\.[0-9]+$//')
      echo "$name $dir"
    done | awk '{ path[$1] = $2 } END { for (n in path) printf "%s = { path = \"%s\" }\n", n, path[n] }'
  } > .cargo/config.toml
  npm install "$packs"/deterministic-code-datasources-cli-*.tgz
fi
cargo add datasources
npx @deterministic-code/datasources-cli setup --yes sqlite
if [ -n "${DATASOURCES_PACKS:-}" ]; then
  npm install "$(cd "$DATASOURCES_PACKS" && pwd)"/deterministic-code-datasources-cli-*.tgz
fi
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run | diff -u expected.txt -
