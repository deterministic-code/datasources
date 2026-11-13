export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
datasources setup --yes "$@" sqlite
datasources generate all --dialect rust --input ./deterministic --stored-procedures
PERF_ROWS="${PERF_ROWS:-25000}" cargo run --bin perf
