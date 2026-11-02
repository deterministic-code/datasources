unset CARGO_TARGET_DIR
export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
cargo init
cp -R ../quick_starts/contacts_cli/rust/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
cargo add datasources
npx @deterministic-code/datasources-cli setup --yes sqlite
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run | diff -u expected.txt -
