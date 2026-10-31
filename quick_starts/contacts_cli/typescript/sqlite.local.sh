export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
npm init -y
cp -R ../quick_starts/contacts_cli/typescript/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
npm install file:../typescript/datasources file:../typescript/providers/sqlite
node ../cli/bin/datasources.mjs setup --yes sqlite
node ../cli/bin/datasources.mjs generate all --dialect typescript
npx tsx src/main.ts | diff -u expected.txt -
