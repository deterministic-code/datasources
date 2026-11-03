export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
npx @deterministic-code/datasources-cli setup --yes "$@" sqlite
npx @deterministic-code/datasources-cli generate all --dialect typescript
npx tsx src/main.ts | diff -u expected.txt -
