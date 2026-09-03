const stamp = "2024-01-01T00:00:00Z";
const uuid = (n: number) =>
  `00000000-0000-4000-8000-${n.toString().padStart(12, "0")}`;

export const seedContactsSql = (count = 250): string[] => {
  const statements: string[] = [];
  let addressId = 1;
  let phoneId = 1;
  for (let id = 1; id <= count; id++) {
    const source = (id % 3) + 1;
    const type = (id % 2) + 1;
    statements.push(
      `INSERT INTO "contacts_base" ("id", "uuid", "created", "updated", "version", "contact_source_id", "contact_type_id", "first_name", "last_name", "email", "notes") VALUES (${id}, '${uuid(id)}', '${stamp}', '${stamp}', X'', ${source}, ${type}, 'First${id}', 'Last${id}', 'c${id}@ex.com', NULL)`,
    );
    for (let a = 0; a < id % 4; a++) {
      statements.push(
        `INSERT INTO "address" ("id", "uuid", "created", "updated", "version", "contact_id", "line1", "line2", "city", "region", "postal_code", "country") VALUES (${addressId}, '${uuid(1000 + addressId)}', '${stamp}', '${stamp}', X'', ${id}, '${id} Line ${a + 1}', NULL, 'City${id}', NULL, NULL, NULL)`,
      );
      addressId += 1;
    }
    for (let p = 0; p < id % 3; p++) {
      statements.push(
        `INSERT INTO "phone" ("id", "uuid", "created", "updated", "version", "contact_id", "number", "label") VALUES (${phoneId}, '${uuid(2000 + phoneId)}', '${stamp}', '${stamp}', X'', ${id}, '555-000-${id.toString().padStart(4, "0")}-${p}', '${p === 0 ? "mobile" : "work"}')`,
      );
      phoneId += 1;
    }
  }
  return statements;
};

export const seedContactCount = 250;
