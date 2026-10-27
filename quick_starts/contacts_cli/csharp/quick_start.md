# Contacts CLI

The cwd is dest. Copy this sample onto `./`, then install Datasources, generate, build, and run.

`deterministic/settings.yaml` lists sqlite, mysql, and postgres, and generate writes SQL for each of those engines. The program prints [`expected.txt`](expected.txt).

## sqlite

```sh
export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
dotnet new console
cp -R quick_starts/contacts_cli/csharp/. ./
dotnet add package Datasources
npx @deterministic-code/datasources-cli setup --yes sqlite
npx @deterministic-code/datasources-cli generate all --dialect csharp
dotnet build
dotnet run
```

## mysql, postgres

```sh
export PROVIDER=mysql
export DATASOURCE_URL=mysql://ds:ds@127.0.0.1:3306/datasources
dotnet new console
cp -R quick_starts/contacts_cli/csharp/. ./
docker compose up -d --wait
dotnet add package Datasources
npx @deterministic-code/datasources-cli setup --yes mysql postgres
npx @deterministic-code/datasources-cli generate all --dialect csharp
dotnet build
dotnet run
export PROVIDER=postgres
export DATASOURCE_URL=postgres://ds:ds@127.0.0.1:5432/datasources
npx @deterministic-code/datasources-cli generate all --dialect csharp
dotnet build
dotnet run
```
