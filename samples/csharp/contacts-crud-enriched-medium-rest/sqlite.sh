export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
node ../../../cli/bin/datasources.mjs generate all --dialect csharp --input ../../contacts-crud-enriched-medium-rest-shared/deterministic
dotnet run
