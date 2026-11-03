export PROVIDER=mysql
export DATASOURCE_URL="${MYSQL_URL:-mysql://ds:ds@mysql:3306/datasources}"
npx @deterministic-code/datasources-cli setup --yes "$@" mysql
npx @deterministic-code/datasources-cli generate all --dialect csharp
dotnet run
export PROVIDER=postgres
export DATASOURCE_URL="${POSTGRES_URL:-postgres://ds:ds@postgres:5432/datasources}"
npx @deterministic-code/datasources-cli setup --yes "$@" postgres
npx @deterministic-code/datasources-cli generate all --dialect csharp
dotnet run
