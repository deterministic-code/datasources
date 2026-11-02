export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
rm -f app.sqlite app.sqlite-wal app.sqlite-shm app.sqlite-journal
node ../../../cli/bin/datasources.mjs setup --yes --local sqlite mysql postgres
node ../../../cli/bin/datasources.mjs generate all --dialect csharp --input ../../contacts-crud-enriched-medium-rest-shared/deterministic
dotnet run
