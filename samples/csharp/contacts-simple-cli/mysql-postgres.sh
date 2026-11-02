export PROVIDER=mysql
export DATASOURCE_URL="${MYSQL_URL:-mysql://ds:ds@mysql:3306/datasources}"
node ../../../cli/bin/datasources.mjs setup --yes --local sqlite mysql postgres
node ../../../cli/bin/datasources.mjs generate all --dialect csharp --input ../../../quick_starts/contacts_cli/shared/deterministic
dotnet run | diff -u ../../../quick_starts/contacts_cli/csharp/expected.txt -
export PROVIDER=postgres
export DATASOURCE_URL="${POSTGRES_URL:-postgres://ds:ds@postgres:5432/datasources}"
node ../../../cli/bin/datasources.mjs generate all --dialect csharp --input ../../../quick_starts/contacts_cli/shared/deterministic
dotnet run | diff -u ../../../quick_starts/contacts_cli/csharp/expected.txt -
