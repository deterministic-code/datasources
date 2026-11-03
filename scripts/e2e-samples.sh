set -e
cd /work
sample="${E2E_SAMPLE:?}"
npm install --prefix cli
node cli/scripts/build-generators.mjs
npm install --omit=dev --prefix typescript/datasources
npm install --omit=dev --prefix typescript/code-generators
case "$sample" in
samples/typescript/*) npm install --prefix "$sample" ;;
esac
(cd "$sample" && sh -e sqlite.sh)
sh scripts/e2e-reset-engines.sh
(cd "$sample" && sh -e mysql-postgres.sh)
