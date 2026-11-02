export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
rm -f app.sqlite app.sqlite-wal app.sqlite-shm app.sqlite-journal
node ../../../cli/bin/datasources.mjs setup --yes --local sqlite mysql postgres
node ../../../cli/bin/datasources.mjs generate all --dialect csharp --input ../../../quick_starts/contacts_cli/shared/deterministic
dotnet run | diff -u ../../../quick_starts/contacts_cli/csharp/expected.txt -
