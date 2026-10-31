export PROVIDER=mysql
export DATASOURCE_URL=mysql://ds:ds@127.0.0.1:3306/datasources
docker compose up -d --wait
node ../../../cli/bin/datasources.mjs generate all --dialect typescript --input ../../../quick_starts/contacts_cli/shared/deterministic
npx tsx src/main.ts | diff -u ../../../quick_starts/contacts_cli/typescript/expected.txt -
export PROVIDER=postgres
export DATASOURCE_URL=postgres://ds:ds@127.0.0.1:5432/datasources
node ../../../cli/bin/datasources.mjs generate all --dialect typescript --input ../../../quick_starts/contacts_cli/shared/deterministic
npx tsx src/main.ts | diff -u ../../../quick_starts/contacts_cli/typescript/expected.txt -
docker compose down -v
