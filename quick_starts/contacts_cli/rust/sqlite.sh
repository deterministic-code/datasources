export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
cargo add datasources
npx @deterministic-code/datasources-cli setup --yes sqlite
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run | diff -u expected.txt -
