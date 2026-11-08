export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-target}/sqlite"
datasources setup --yes "$@" sqlite
datasources generate all --dialect rust
cargo run
