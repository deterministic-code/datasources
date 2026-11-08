export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
dotnet add package Datasources
npx @deterministic-code/datasources-cli setup --yes sqlite
npx @deterministic-code/datasources-cli generate all --dialect csharp
dotnet build
dotnet run | diff -u expected.txt -
