export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
npm init -y
cp -R ../quick_starts/contacts_cli/typescript/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
npm install file:../typescript/datasources file:../typescript/providers/sqlite ../dest-cli/deterministic-code-datasources-cli-*.tgz
node ./node_modules/@deterministic-code/datasources-cli/bin/datasources.js setup --yes sqlite
node ./node_modules/@deterministic-code/datasources-cli/bin/datasources.js generate all --dialect typescript
npx tsx src/main.ts | diff -u expected.txt -
