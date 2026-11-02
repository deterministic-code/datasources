export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
rm -f app.sqlite app.sqlite-wal app.sqlite-shm app.sqlite-journal
node ../../../cli/bin/datasources.mjs setup --yes --local sqlite mysql postgres
node ../../../cli/bin/datasources.mjs generate all --dialect typescript --input ../../../quick_starts/contacts_cli/shared/deterministic
npx tsx src/main.ts | diff -u ../../../quick_starts/contacts_cli/typescript/expected.txt -
