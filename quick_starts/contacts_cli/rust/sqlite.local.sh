export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
cargo init
cp -R ../quick_starts/contacts_cli/rust/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
cargo add --path ../rust/datasources
cargo add --path ../rust/providers/sqlite
cargo add tokio --features macros,rt-multi-thread
node ../cli/bin/datasources.mjs setup --yes sqlite
node ../cli/bin/datasources.mjs generate all --dialect rust
cargo run | diff -u expected.txt -
