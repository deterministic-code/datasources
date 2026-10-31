export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
dotnet new console
cp -R ../quick_starts/contacts_cli/csharp/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
dotnet add reference ../csharp/Datasources/Datasources.csproj
dotnet add reference ../csharp/Providers/Sqlite/Datasources.Sqlite.csproj
node ../cli/bin/datasources.mjs setup --yes sqlite
node ../cli/bin/datasources.mjs generate all --dialect csharp
dotnet build
dotnet run | diff -u expected.txt -
