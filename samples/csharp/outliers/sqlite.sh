set -e -o pipefail
rm -f app.sqlite app.sqlite-wal app.sqlite-shm app.sqlite-journal
export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
datasources setup --yes "$@" sqlite
datasources generate all --dialect csharp
dotnet run --project Outliers.csproj | diff -u expected.txt -
