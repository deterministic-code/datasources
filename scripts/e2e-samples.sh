set -e
cd /work
npm install --prefix typescript
npm install --prefix cli
for dir in \
  samples/typescript/contacts-simple-cli \
  samples/typescript/contacts-crud-enriched-medium-rest \
  samples/typescript/user-roles-simple-cli
do
  npm install --prefix "$dir"
  (cd "$dir" && sh -e sqlite.sh)
  sh scripts/e2e-reset-engines.sh
  (cd "$dir" && sh -e mysql-postgres.sh)
done
for dir in \
  samples/csharp/contacts-simple-cli \
  samples/csharp/contacts-crud-enriched-medium-rest \
  samples/csharp/user-roles-simple-cli \
  samples/rust/contacts-simple-cli \
  samples/rust/contacts-crud-enriched-medium-rest \
  samples/rust/user-roles-simple-cli
do
  (cd "$dir" && sh -e sqlite.sh)
  sh scripts/e2e-reset-engines.sh
  (cd "$dir" && sh -e mysql-postgres.sh)
done
