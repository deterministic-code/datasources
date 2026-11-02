set -e
cd /work
lang="${E2E_LANG:?}"
for script in sqlite.sh mysql-postgres.sh; do
  rm -rf dest
  mkdir dest
  if [ "$script" = mysql-postgres.sh ]; then sh scripts/e2e-reset-engines.sh; fi
  (cd dest && sh -e "../quick_starts/contacts_cli/$lang/$script")
  rm -rf dest
done
