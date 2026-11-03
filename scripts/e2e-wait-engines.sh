#!/bin/sh
set -e
i=0
while [ "$i" -lt 90 ]; do
  if mysqladmin ping -h mysql -uds -pds --silent && pg_isready -h postgres -U ds -d datasources >/dev/null 2>&1; then
    exit 0
  fi
  i=$((i + 1))
  sleep 1
done
printf 'e2e-wait-engines: mysql or postgres not ready\n' >&2
exit 1
