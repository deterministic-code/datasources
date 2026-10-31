export PROVIDER=mysql
export DATASOURCE_URL=mysql://ds:ds@127.0.0.1:3306/datasources
docker compose up -d --wait
node ../../../cli/bin/datasources.mjs generate all --dialect typescript --input ../../contacts-crud-enriched-medium-rest-shared/deterministic
npx tsx src/main.ts
export PROVIDER=postgres
export DATASOURCE_URL=postgres://ds:ds@127.0.0.1:5432/datasources
node ../../../cli/bin/datasources.mjs generate all --dialect typescript --input ../../contacts-crud-enriched-medium-rest-shared/deterministic
npx tsx src/main.ts
docker compose down -v
