unset CARGO_TARGET_DIR
export PROVIDER=mysql
export DATASOURCE_URL=mysql://ds:ds@127.0.0.1:3306/datasources
docker compose down --remove-orphans
docker compose up -d --wait --force-recreate
node ../../../cli/bin/datasources.mjs generate all --dialect rust --input ../../../quick_starts/contacts_cli/shared/deterministic
cargo run | diff -u ../../../quick_starts/contacts_cli/rust/expected.txt -
export PROVIDER=postgres
export DATASOURCE_URL=postgres://ds:ds@127.0.0.1:5432/datasources
node ../../../cli/bin/datasources.mjs generate all --dialect rust --input ../../../quick_starts/contacts_cli/shared/deterministic
cargo run | diff -u ../../../quick_starts/contacts_cli/rust/expected.txt -
docker compose down -v
