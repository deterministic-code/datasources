# Contacts CLI

Dest is a new folder inside this clone. From the clone, `mkdir dest && cd dest`, then copy this sample onto `./`, install Datasources, generate, and run.

Generate writes SQL for the providers `setup` installed (`INSTALLED_PROVIDERS` in `.env`), not every engine listed in `settings.yaml`. The program prints [`expected.txt`](expected.txt).

## sqlite

```sh
export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
npm init -y
cp -R ../quick_starts/contacts_cli/typescript/. ./
npm install @deterministic-code/datasources
npx @deterministic-code/datasources-cli setup --yes sqlite
npx @deterministic-code/datasources-cli generate all --dialect typescript
npx tsx src/main.ts
```

## mysql, postgres

```sh
export PROVIDER=mysql
export DATASOURCE_URL=mysql://ds:ds@127.0.0.1:3306/datasources
npm init -y
cp -R ../quick_starts/contacts_cli/typescript/. ./
docker compose up -d --wait
npm install @deterministic-code/datasources
npx @deterministic-code/datasources-cli setup --yes mysql postgres
npx @deterministic-code/datasources-cli generate all --dialect typescript
npx tsx src/main.ts
export PROVIDER=postgres
export DATASOURCE_URL=postgres://ds:ds@127.0.0.1:5432/datasources
npx @deterministic-code/datasources-cli generate all --dialect typescript
npx tsx src/main.ts
docker compose down -v
```
