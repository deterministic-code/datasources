import assert from "node:assert/strict";
import { describe, it } from "vitest";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { generate as generateSql } from "@deterministic-code/datasources-code-generators/sql/generate-sql";
import "./register-sql-generate.ts";

const typesYaml = `types:
  - gender:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - uuid:
            type: uuid
            default_value: NewId
`;

const datasourceYaml = `includes:
  - types:
      filter: tag == "datasource_type"
types: []
`;

describe("generate sql", () => {
  it("omits named default constraints from mysql migrations", async () => {
    const files = await generateSql(
      {
        reader: memoryReader({
          "types.yaml": typesYaml,
          "datasource.yaml": datasourceYaml,
        }),
        settings: {},
      },
      ["mysql"],
    );
    const up = files.find((entry) => entry.filename.endsWith("mysql/migrations/0001_initial_up.sql"));
    assert.ok(up, "expected mysql/migrations/0001_initial_up.sql");
    const sql = "contents" in up ? String(up.contents) : up.content;
    assert.match(sql, /DEFAULT \(UUID\(\)\)/);
    assert.doesNotMatch(sql, /CONSTRAINT `[^`]*_default_constraint` DEFAULT/);
  });

  it("infers a reference column type from the parent id", async () => {
    const files = await generateSql(
      {
        reader: memoryReader({
          "types.yaml": `types:
  - gender:
      tags: [datasource_type]
      inherits: set
      fields:
        - name:
            type: string
            size: 128
  - contact:
      tags: [datasource_type]
      inherits: set
      fields:
        - gender_type_id:
            references: gender.id
`,
        }),
        settings: {},
      },
      ["mysql"],
    );
    const up = files.find((entry) => entry.filename.endsWith("0001_initial_up.sql"));
    assert.ok(up, "expected initial up migration");
    const sql = "contents" in up ? String(up.contents) : up.content;
    const contacts = sql.match(/CREATE TABLE `contacts` \(([\s\S]*?)\);/)?.[1];
    assert.ok(contacts, "expected contacts table body");
    assert.match(contacts, /`gender_type_id` INT/);
    assert.doesNotMatch(contacts, /`gender_type_id` TEXT/);
  });
});
