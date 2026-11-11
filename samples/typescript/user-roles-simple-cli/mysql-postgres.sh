run_engine() {
  datasources setup --yes "$@" "$PROVIDER"
  datasources generate all --dialect typescript
  cli() { npx tsx src/main.ts "$@"; }
  cli add-role admin
  cli add-role editor
  cli add-group staff
  cli add-user ada secret
  cli add-user alan secret
  cli add-user-role ada admin
  cli update-memberships | diff -u expected.txt -
}
export PROVIDER=mysql
export DATASOURCE_URL="${MYSQL_URL:-mysql://ds:ds@mysql:3306/datasources}"
run_engine "$@"
export PROVIDER=postgres
export DATASOURCE_URL="${POSTGRES_URL:-postgres://ds:ds@postgres:5432/datasources}"
run_engine "$@"
