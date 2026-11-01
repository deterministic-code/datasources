export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
dotnet new console
cp -R ../quick_starts/contacts_cli/csharp/. ./
cp -R ../quick_starts/contacts_cli/shared/deterministic ./deterministic
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
npx @deterministic-code/datasources-cli setup --yes sqlite
if [ -n "${DATASOURCES_PACKS:-}" ]; then
  npm install "$(cd "$DATASOURCES_PACKS" && pwd)"/deterministic-code-datasources-cli-*.tgz
fi
npx @deterministic-code/datasources-cli generate all --dialect csharp
dotnet build
dotnet run | diff -u expected.txt -
