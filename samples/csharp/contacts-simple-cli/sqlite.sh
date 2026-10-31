export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
node ../../../cli/bin/datasources.mjs generate all --dialect csharp --input ../../../quick_starts/contacts_cli/shared/deterministic
dotnet run | diff -u ../../../quick_starts/contacts_cli/csharp/expected.txt -
