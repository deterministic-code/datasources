export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
node ../../../cli/bin/datasources.mjs generate all --dialect rust --input ../../../quick_starts/contacts_cli/shared/deterministic
cargo run | diff -u ../../../quick_starts/contacts_cli/rust/expected.txt -
