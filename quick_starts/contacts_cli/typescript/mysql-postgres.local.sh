export PROVIDER=mysql
export DATASOURCE_URL=mysql://ds:ds@127.0.0.1:3306/datasources
npm init -y
cp -R ../quick_starts/contacts_cli/typescript/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
docker compose down --remove-orphans
docker compose up -d --wait --force-recreate
npm install file:../typescript/datasources file:../typescript/providers/mysql file:../typescript/providers/postgres
node ../cli/bin/datasources.mjs setup --yes mysql postgres
node ../cli/bin/datasources.mjs generate all --dialect typescript
npx tsx src/main.ts | diff -u expected.txt -
export PROVIDER=postgres
export DATASOURCE_URL=postgres://ds:ds@127.0.0.1:5432/datasources
node ../cli/bin/datasources.mjs generate all --dialect typescript
npx tsx src/main.ts | diff -u expected.txt -
docker compose down -v
