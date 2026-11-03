export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
npx @deterministic-code/datasources-cli setup --yes "$@" sqlite
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run | diff -u expected.txt -
rm -rf target
