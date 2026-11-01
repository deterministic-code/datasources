export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
npm init -y
cp -R ../quick_starts/contacts_cli/typescript/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
if [ -n "${DATASOURCES_PACKS:-}" ]; then
  npm install \
    "$DATASOURCES_PACKS"/deterministic-code-datasources-[0-9]*.tgz \
    "$DATASOURCES_PACKS"/deterministic-code-datasources-sqlite-*.tgz \
    "$DATASOURCES_PACKS"/deterministic-code-datasources-cli-*.tgz
else
  npm install @deterministic-code/datasources
fi
npx @deterministic-code/datasources-cli setup --yes sqlite
if [ -n "${DATASOURCES_PACKS:-}" ]; then
  npm install \
    "$DATASOURCES_PACKS"/deterministic-code-datasources-[0-9]*.tgz \
    "$DATASOURCES_PACKS"/deterministic-code-datasources-sqlite-*.tgz \
    "$DATASOURCES_PACKS"/deterministic-code-datasources-cli-*.tgz
fi
npx @deterministic-code/datasources-cli generate all --dialect typescript
npx tsx src/main.ts | diff -u expected.txt -
