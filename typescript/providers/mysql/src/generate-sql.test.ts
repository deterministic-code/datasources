import assert from "node:assert/strict";
import { describe, it } from "vitest";
import {
  CreateIndexStatement,
  CreateTriggerStatement,
  ForeignKey,
  PrimaryKey,
} from "@deterministic-code/datasources";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { generate as generateSql } from "@deterministic-code/datasources-code-generators/sql/generate-sql";
import { generate as generateStoredProcedures } from "@deterministic-code/datasources-code-generators/sql/generate-stored-procedures";
import { createCasing } from "../../../code-generators/src/sql/common/default-casing.ts";
import { fitMysqlIdentifier } from "./MysqlIdentifier.ts";
import "./register-sql-generate.ts";

const longEntity = "game_journey_setting_category_preference_option_value";
const longTypesYaml = `types:
  - category_type:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
  - ${longEntity}:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - created:
            type: datetime
        - updated:
            type: datetime
        - category_type_id:
            type: integer
            references: category_type.id
        - external_reference_code:
            type: string
`;
const longDatasourceYaml = `includes:
  - types:
      filter: tag == "datasource_type"
types:
  - ${longEntity}:
      indexes:
        - category_type_id:
            fields:
              - category_type_id
`;
const sqlOf = (entry: { contents?: string; content?: string } | undefined): string => {
  assert.ok(entry, "expected migration file");
  return "contents" in entry && entry.contents !== undefined ? String(entry.contents) : String(entry.content);
};

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
    assert.match(sql, /;\nGO\nCREATE TRIGGER trg_genders_updated_at/);
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
    assert.match(customSql, /;\nBATCH\nCREATE TRIGGER trg_genders_updated_at/);
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
    assert.match(sql, /INSERT INTO genders[\s\S]*;\nGO\nINSERT INTO genders/);
  });

  it("joins stored procedures without GO; or consecutive GO", async () => {
    const files = await generateStoredProcedures(
      {
        reader: memoryReader({
          "types.yaml": `types:
  - user:
      tags: [datasource_type]
      inherits: set
      fields:
        - email:
            type: string
        - created:
            type: datetime
        - updated:
            type: datetime
`,
        }),
        settings: { "datasource.use_stored_procedures": "true" },
      },
      ["mysql"],
    );
    const up = files.find((entry) => entry.filename.endsWith("mysql/migrations/0002_stored_procedures_up.sql"));
    assert.ok(up, "expected stored procedures up migration");
    const sql = "contents" in up ? String(up.contents) : up.content;
    assert.match(sql, /CREATE PROCEDURE create_users/);
    assert.match(sql, /bulk_create_users/);
    assert.doesNotMatch(sql, /GO;/);
    assert.doesNotMatch(sql, /\nGO\nGO\n/);
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
    assert.match(sql, /;\nGO\nCREATE TABLE contacts/);
    const contacts = sql.match(/CREATE TABLE contacts \(([\s\S]*?)\);/)?.[1];
    assert.ok(contacts, "expected contacts table body");
    assert.match(contacts, /gender_type_id INT/);
    assert.doesNotMatch(contacts, /gender_type_id TEXT/);
  });

  it("shortens index, foreign key, and trigger names over 64 characters in migrations", async () => {
    const naming = createCasing({}).naming;
    const index = CreateIndexStatement.nameOf(longEntity, naming, "category_type_id", "index");
    const foreignKey = ForeignKey.nameOf(longEntity, naming, "category_type_id", "foreign_key");
    const trigger = CreateTriggerStatement.nameOf(longEntity, naming);
    const primaryKey = PrimaryKey.nameOf(longEntity, naming, "primary_key");
    for (const name of [index, foreignKey, trigger, primaryKey]) assert.ok(name.length > 64, name);
    const files = await generateSql(
      {
        reader: memoryReader({
          "types.yaml": longTypesYaml,
          "datasource.yaml": longDatasourceYaml,
        }),
        settings: {},
      },
      ["mysql"],
    );
    const up = sqlOf(files.find((entry) => entry.filename.endsWith("mysql/migrations/0001_initial_up.sql")));
    const down = sqlOf(files.find((entry) => entry.filename.endsWith("mysql/migrations/0001_initial_down.sql")));
    const fitted = (name: string) => {
      const short = fitMysqlIdentifier(name);
      assert.ok(short.length <= 64, short);
      assert.notEqual(short, name);
      return short;
    };
    const ident = (name: string) => `(?:\\\`${name}\\\`|${name})`;
    const indexName = fitted(index);
    const foreignName = fitted(foreignKey);
    const triggerName = fitted(trigger);
    const primaryName = fitted(primaryKey);
    assert.match(up, new RegExp(`CREATE INDEX ${ident(indexName)} ON`));
    assert.match(up, new RegExp(`CONSTRAINT ${ident(foreignName)} FOREIGN KEY`));
    assert.match(up, new RegExp(`CONSTRAINT ${ident(primaryName)} PRIMARY KEY`));
    assert.match(up, new RegExp(`CREATE TRIGGER ${ident(triggerName)}`));
    for (const name of [index, foreignKey, trigger, primaryKey]) {
      assert.equal(up.includes(name), false, name);
      assert.equal(down.includes(name), false, name);
    }
    assert.equal(up.includes(indexName), true);
    assert.equal(up.includes(foreignName), true);
    assert.equal(up.includes(triggerName), true);
    assert.equal(up.includes(primaryName), true);
    for (const name of [indexName, foreignName, triggerName, primaryName]) {
      assert.ok(name.length <= 64, name);
    }
  });
});
