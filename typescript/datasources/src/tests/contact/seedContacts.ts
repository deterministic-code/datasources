const stamp = "2024-01-01 00:00:00";
const uuid = (n: number) =>
  `00000000-0000-4000-8000-${n.toString().padStart(12, "0")}`;

export const seedContactsSql = (count = 250, version = "X''"): string[] => {
  const statements: string[] = [];
  let addressId = 1;
  let phoneId = 1;
  for (let id = 1; id <= count; id++) {
    const source = (id % 3) + 1;
    const type = (id % 2) + 1;
    statements.push(
      `INSERT INTO "contacts_base" ("id", "uuid", "created", "updated", "version", "contact_source_id", "contact_type_id", "first_name", "last_name", "email", "notes") VALUES (${id}, '${uuid(id)}', '${stamp}', '${stamp}', ${version}, ${source}, ${type}, 'First${id}', 'Last${id}', 'c${id}@ex.com', NULL)`,
    );
    for (let a = 0; a < id % 4; a++) {
      statements.push(
        `INSERT INTO "address" ("id", "uuid", "created", "updated", "version", "contact_id", "address_type_id", "line1", "line2", "city", "region", "postal_code", "country") VALUES (${addressId}, '${uuid(1000 + addressId)}', '${stamp}', '${stamp}', ${version}, ${id}, ${a === 0 ? 1 : 2}, '${id} Line ${a + 1}', NULL, 'City${id}', NULL, NULL, NULL)`,
      );
      addressId += 1;
    }
    for (let p = 0; p < id % 3; p++) {
      statements.push(
        `INSERT INTO "phone" ("id", "uuid", "created", "updated", "version", "contact_id", "phone_type_id", "number") VALUES (${phoneId}, '${uuid(2000 + phoneId)}', '${stamp}', '${stamp}', ${version}, ${id}, ${p === 0 ? 1 : 2}, '555-000-${id.toString().padStart(4, "0")}-${p}')`,
      );
      phoneId += 1;
    }
  }
  return statements;
};

export const seedContactCount = 250;

export const seedIdentitySql = (engine: string): string[] => {
  const tables = ["contacts_base", "address", "phone"] as const;
  return engine === "postgres"
    ? tables.map(
        (table) =>
          `SELECT setval(pg_get_serial_sequence('${table}', 'id'), COALESCE((SELECT MAX(id) FROM "${table}"), 1))`,
      )
    : engine === "mysql"
      ? tables.map((table) => `ALTER TABLE ${table} AUTO_INCREMENT = ${seedContactCount + 1}`)
      : [];
};
