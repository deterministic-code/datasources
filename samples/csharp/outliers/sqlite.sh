set -e -o pipefail
export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
datasources setup --yes "$@" sqlite
datasources generate all --dialect csharp
dotnet run --project Outliers.csproj | diff -u expected.txt -
