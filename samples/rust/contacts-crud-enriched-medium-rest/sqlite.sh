unset CARGO_TARGET_DIR
export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
rm -f app.sqlite app.sqlite-wal app.sqlite-shm app.sqlite-journal
npx @deterministic-code/datasources-cli setup --yes sqlite mysql postgres
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run
