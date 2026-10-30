export PROVIDER=mysql
export DATASOURCE_URL=mysql://ds:ds@127.0.0.1:3306/datasources
npm init -y
cp -R ../quick_starts/contacts_cli/typescript/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
docker compose up -d --wait
npm install @deterministic-code/datasources
npx @deterministic-code/datasources-cli setup --yes mysql postgres
npx @deterministic-code/datasources-cli generate all --dialect typescript
npx tsx src/main.ts | diff -u expected.txt -
export PROVIDER=postgres
export DATASOURCE_URL=postgres://ds:ds@127.0.0.1:5432/datasources
npx @deterministic-code/datasources-cli generate all --dialect typescript
npx tsx src/main.ts | diff -u expected.txt -
docker compose down -v
