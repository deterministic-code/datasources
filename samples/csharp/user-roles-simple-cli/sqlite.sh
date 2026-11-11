export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
datasources setup --yes "$@" sqlite
datasources generate all --dialect csharp
cli() { dotnet run --project UserRolesSimpleCli.csproj -- "$@"; }
cli add-role admin
cli add-role editor
cli add-group staff
cli add-user ada secret
cli add-user alan secret
cli add-user-role ada admin
cli update-memberships | diff -u expected.txt -
