export PROVIDER=mysql
export DATASOURCE_URL=mysql://ds:ds@127.0.0.1:3306/datasources
dotnet new console
cp -R ../quick_starts/contacts_cli/csharp/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
docker compose down --remove-orphans
docker compose up -d --wait --force-recreate
dotnet add reference ../csharp/Datasources/Datasources.csproj
dotnet add reference ../csharp/Providers/Mysql/Datasources.Mysql.csproj
dotnet add reference ../csharp/Providers/Postgres/Datasources.Postgres.csproj
node ../cli/bin/datasources.mjs setup --yes mysql postgres
node ../cli/bin/datasources.mjs generate all --dialect csharp
dotnet build
dotnet run | diff -u expected.txt -
export PROVIDER=postgres
export DATASOURCE_URL=postgres://ds:ds@127.0.0.1:5432/datasources
node ../cli/bin/datasources.mjs generate all --dialect csharp
dotnet build
dotnet run | diff -u expected.txt -
docker compose down -v
