export PROVIDER=mysql
export DATASOURCE_URL="${MYSQL_URL:-mysql://ds:ds@mysql:3306/datasources}"
datasources setup --yes "$@" mysql
datasources generate all --dialect typescript --input ./deterministic --stored-procedures
PERF_ROWS="${PERF_ROWS:-25000}" npx tsx src/perf.ts
export PROVIDER=postgres
export DATASOURCE_URL="${POSTGRES_URL:-postgres://ds:ds@postgres:5432/datasources}"
datasources setup --yes "$@" postgres
datasources generate all --dialect typescript --input ./deterministic --stored-procedures
PERF_ROWS="${PERF_ROWS:-25000}" npx tsx src/perf.ts
