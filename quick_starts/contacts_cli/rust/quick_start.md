# Contacts CLI

From this folder, install the published Datasources packages, generate, and run. Generate writes SQL for the providers `setup` installed (`INSTALLED_PROVIDERS` in `.env`), not every engine listed in `settings.yaml`. The program prints [`expected.txt`](expected.txt).

## sqlite

```sh
export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
cargo add datasources
npx @deterministic-code/datasources-cli setup --yes sqlite
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run
```

## mysql, postgres

```sh
export PROVIDER=mysql
export DATASOURCE_URL=mysql://ds:ds@127.0.0.1:3306/datasources
docker compose up -d --wait
cargo add datasources
npx @deterministic-code/datasources-cli setup --yes mysql postgres
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run
export PROVIDER=postgres
export DATASOURCE_URL=postgres://ds:ds@127.0.0.1:5432/datasources
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run
docker compose down -v
```
