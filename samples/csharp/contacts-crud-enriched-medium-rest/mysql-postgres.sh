export PROVIDER=mysql
export DATASOURCE_URL=mysql://ds:ds@127.0.0.1:3306/datasources
docker compose up -d --wait
node ../../../cli/bin/datasources.mjs generate all --dialect csharp --input ../../contacts-crud-enriched-medium-rest-shared/deterministic
dotnet run
export PROVIDER=postgres
export DATASOURCE_URL=postgres://ds:ds@127.0.0.1:5432/datasources
node ../../../cli/bin/datasources.mjs generate all --dialect csharp --input ../../contacts-crud-enriched-medium-rest-shared/deterministic
dotnet run
docker compose down -v
