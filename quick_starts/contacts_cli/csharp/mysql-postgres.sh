export PROVIDER=mysql
export DATASOURCE_URL=mysql://ds:ds@127.0.0.1:3306/datasources
dotnet new console
cp -R ../quick_starts/contacts_cli/csharp/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
docker compose down --remove-orphans
docker compose up -d --wait --force-recreate
if [ -n "${DATASOURCES_PACKS:-}" ]; then
  packs=$(cd "$DATASOURCES_PACKS" && pwd)
  cat > nuget.config <<EOF
<?xml version="1.0" encoding="utf-8"?>
<configuration>
  <packageSources>
    <clear />
    <add key="datasources-local" value="$packs/nuget" />
    <add key="nuget.org" value="https://api.nuget.org/v3/index.json" />
  </packageSources>
</configuration>
EOF
  dotnet add package Datasources --source "$packs/nuget"
  npm install "$packs"/deterministic-code-datasources-cli-*.tgz
else
  dotnet add package Datasources
fi
npx @deterministic-code/datasources-cli setup --yes mysql postgres
if [ -n "${DATASOURCES_PACKS:-}" ]; then
  npm install "$(cd "$DATASOURCES_PACKS" && pwd)"/deterministic-code-datasources-cli-*.tgz
fi
npx @deterministic-code/datasources-cli generate all --dialect csharp
dotnet build
dotnet run | diff -u expected.txt -
export PROVIDER=postgres
export DATASOURCE_URL=postgres://ds:ds@127.0.0.1:5432/datasources
npx @deterministic-code/datasources-cli generate all --dialect csharp
dotnet build
dotnet run | diff -u expected.txt -
docker compose down -v
