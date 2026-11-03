set -e
cd /work
unset DATASOURCES_REPO
ln -sfn /repo/quick_starts /work/quick_starts
lang="${E2E_LANG:?}"
export npm_config_prefer_online=true
for script in sqlite.sh mysql-postgres.sh; do
  rm -rf dest
  mkdir dest
  if [ "$script" = mysql-postgres.sh ]; then sh /repo/scripts/e2e-reset-engines.sh; fi
  (cd dest && sh -e "/repo/quick_starts/contacts_cli/$lang/$script")
  rm -rf dest
done
