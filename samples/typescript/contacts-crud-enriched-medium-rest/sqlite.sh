export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
node ../../../cli/bin/datasources.mjs generate all --dialect typescript --input ../../contacts-crud-enriched-medium-rest-shared/deterministic
npx tsx src/main.ts
