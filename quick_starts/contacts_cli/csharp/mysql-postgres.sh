export PROVIDER=mysql
export DATASOURCE_URL=mysql://ds:ds@127.0.0.1:3306/datasources
dotnet new console
cp -R ../quick_starts/contacts_cli/csharp/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
docker compose up -d --wait
dotnet add package Datasources
npx @deterministic-code/datasources-cli setup --yes mysql postgres
npx @deterministic-code/datasources-cli generate all --dialect csharp
dotnet build
dotnet run | diff -u expected.txt -
export PROVIDER=postgres
export DATASOURCE_URL=postgres://ds:ds@127.0.0.1:5432/datasources
npx @deterministic-code/datasources-cli generate all --dialect csharp
dotnet build
dotnet run | diff -u expected.txt -
docker compose down -v
