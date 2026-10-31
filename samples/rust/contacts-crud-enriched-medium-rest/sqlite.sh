unset CARGO_TARGET_DIR
export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
node ../../../cli/bin/datasources.mjs generate all --dialect rust --input ../../contacts-crud-enriched-medium-rest-shared/deterministic
cargo run
