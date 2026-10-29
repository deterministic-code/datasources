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

  it("separates table and trigger with datasource.batch_separator", async () => {
    const auditYaml = `types:
  - gender:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - created:
            type: datetime
            default_value: UtcNow
        - updated:
            type: datetime
            default_value: UtcNow
`;
    const files = await generateSql(
      {
        reader: memoryReader({
          "types.yaml": auditYaml,
          "datasource.yaml": datasourceYaml,
        }),
        settings: {},
      },
      ["mysql"],
    );
    const up = files.find((entry) => entry.filename.endsWith("mysql/migrations/0001_initial_up.sql"));
    assert.ok(up, "expected mysql/migrations/0001_initial_up.sql");
    const sql = "contents" in up ? String(up.contents) : up.content;
    assert.match(sql, /;\nGO\nCREATE TRIGGER `trg_genders_updated_at`/);
    const custom = await generateSql(
      {
        reader: memoryReader({
          "types.yaml": auditYaml,
          "datasource.yaml": datasourceYaml,
        }),
        settings: { "datasource.batch_separator": "BATCH" },
      },
      ["mysql"],
    );
    const customUp = custom.find((entry) => entry.filename.endsWith("mysql/migrations/0001_initial_up.sql"));
    assert.ok(customUp, "expected mysql/migrations/0001_initial_up.sql");
    const customSql = "contents" in customUp ? String(customUp.contents) : customUp.content;
    assert.match(customSql, /;\nBATCH\nCREATE TRIGGER `trg_genders_updated_at`/);
    assert.doesNotMatch(customSql, /\nGO\n/);
  });

  it("separates seed inserts with datasource.batch_separator", async () => {
    const files = {
      "types.yaml": `types:
  - gender:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
`,
      "datasource_seeds.yaml": `version: 1.0.0
seeds:
  - gender:
      - id1:
          name: Male
      - id2:
          name: Female
`,
    };
    const up = (await generateSql({ reader: memoryReader(files), settings: {} }, ["mysql"])).find(
      (entry) => entry.filename.endsWith("mysql/migrations/0001_initial_up.sql"),
    );
    assert.ok(up, "expected mysql/migrations/0001_initial_up.sql");
    const sql = "contents" in up ? String(up.contents) : up.content;
    assert.match(sql, /INSERT INTO `genders`[\s\S]*;\nGO\nINSERT INTO `genders`/);
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
    assert.match(sql, /;\nGO\nCREATE TABLE `contacts`/);
    const contacts = sql.match(/CREATE TABLE `contacts` \(([\s\S]*?)\);/)?.[1];
    assert.ok(contacts, "expected contacts table body");
    assert.match(contacts, /`gender_type_id` INT/);
    assert.doesNotMatch(contacts, /`gender_type_id` TEXT/);
  });
});
