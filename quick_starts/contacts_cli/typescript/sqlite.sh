export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
npm init -y
cp -R ../quick_starts/contacts_cli/typescript/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
npm install @deterministic-code/datasources
npx @deterministic-code/datasources-cli setup --yes sqlite
npx @deterministic-code/datasources-cli generate all --dialect typescript
npx tsx src/main.ts | diff -u expected.txt -
