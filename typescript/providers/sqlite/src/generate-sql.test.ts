import assert from "node:assert/strict";
import { describe, it } from "vitest";
import { parse, stringify } from "yaml";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { generate as generateSql } from "@deterministic-code/datasources-code-generators/sql/generate-sql";
import "./register-sql-generate.ts";

const TYPES_YAML = `types:
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
`;

const ctx = (settings: Record<string, string>) => ({
  reader: memoryReader({ "types.yaml": TYPES_YAML }),
  settings,
});

const names = (
  entries: { filename: string }[],
): string[] => entries.map((e) => e.filename).sort();

describe("generate-sql datasource.use_stored_procedures", () => {


  it("omits stored-procedure migrations on sqlite even when the flag is true", async () => {
    const files = names(
      await generateSql(
        ctx({ "datasource.use_stored_procedures": "true" }),
        ["sqlite"],
      ),
    );
    assert.ok(files.includes("sqlite/migrations/0001_initial_up.sql"));
    assert.equal(
      files.some((f) => f.includes("stored_procedures")),
      false,
    );
  });
});

describe("generate-sql datasource.batch_separator", () => {
  const upSql = async (settings: Record<string, string>) => {
    const files = await generateSql(ctx(settings), ["sqlite"]);
    const up = files.find((entry) => entry.filename.endsWith("sqlite/migrations/0001_initial_up.sql"));
    assert.ok(up, "expected sqlite/migrations/0001_initial_up.sql");
    return "contents" in up ? String(up.contents) : up.content;
  };

  it("defaults to a whole-line GO between table and trigger", async () => {
    assert.match(await upSql({}), /;\nGO\nCREATE TRIGGER "trg_users_updated_at"/);
  });

  it("puts GO between tables", async () => {
    const files = await generateSql(
      {
        reader: memoryReader({
          "types.yaml": `types:
  - gender:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - created:
            type: datetime
        - updated:
            type: datetime
  - contact:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - created:
            type: datetime
        - updated:
            type: datetime
`,
        }),
        settings: {},
      },
      ["sqlite"],
    );
    const up = files.find((entry) => entry.filename.endsWith("sqlite/migrations/0001_initial_up.sql"));
    assert.ok(up, "expected sqlite/migrations/0001_initial_up.sql");
    const sql = "contents" in up ? String(up.contents) : up.content;
    assert.match(sql, /trg_genders_updated_at[\s\S]*;\nGO\nCREATE TABLE "contacts"/);
  });

  it("uses the configured separator", async () => {
    const sql = await upSql({ "datasource.batch_separator": "BATCH" });
    assert.match(sql, /;\nBATCH\nCREATE TRIGGER "trg_users_updated_at"/);
    assert.doesNotMatch(sql, /\nGO\n/);
  });

  it("separates seed inserts with datasource.batch_separator", async () => {
    const files = await generateSql(
      {
        reader: memoryReader({
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
        }),
        settings: {},
      },
      ["sqlite"],
    );
    const up = files.find((entry) => entry.filename.endsWith("sqlite/migrations/0001_initial_up.sql"));
    assert.ok(up, "expected sqlite/migrations/0001_initial_up.sql");
    const sql = "contents" in up ? String(up.contents) : up.content;
    assert.match(sql, /INSERT INTO "genders"[\s\S]*;\nGO\nINSERT INTO "genders"/);
  });
});

const identityYaml = `types:
  - person:
      tags: [datasource_type]
      inherits: set
      fields:
        - id:
            type: integer
            is_id: true
        - code:
            type: integer
        - email:
            type: string
  - link:
      tags: [datasource_type]
      inherits: set
      ids: [left_id, right_id]
      fields:
        - left_id:
            type: integer
        - right_id:
            type: integer
  - country:
      tags: [datasource_type]
      inherits: set
      fields:
        - name:
            type: string
`;

const identityDatasource = `includes:
  - types:
      filter: tag == "datasource_type"
types:
  - country:
      fields:
        - id:
            is_fixed_id: true
            is_readonly: true
`;

const identityCtx = {
  reader: memoryReader({
    "types.yaml": identityYaml,
    "datasource.yaml": identityDatasource,
  }),
  settings: {},
};

const upBody = async (
  settings: Record<string, string> = {},
) => {
  const files = await generateSql({
    reader: identityCtx.reader,
    settings,
  }, ["postgres"]);
  const up = files.find((e) => e.filename.endsWith("0001_initial_up.sql"));
  assert.ok(up, "expected initial up migration");
  return "contents" in up ? String(up.contents) : up.content;
};

describe("generate-sql identity keys", () => {



  it("throws when id is missing is_id", async () => {
    await assert.rejects(
      () =>
        generateSql(
          {
            reader: memoryReader({
              "types.yaml": `types:
  - card:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
        - label:
            type: string
`,
            }),
            settings: {},
          },
          ["sqlite"],
        ),
      /card\.id must have is_id: true/,
    );
  });
});

describe("generate-sql base defaults", () => {
  it("defaults uuid created updated and version on a base table", async () => {
    const files = await generateSql({
      reader: memoryReader({
        "types.yaml": `types:
  - base:
      fields:
        - id:
            type: integer
            is_id: true
        - uuid:
            type: uuid
            default_value: NewId
        - created:
            type: datetime
            default_value: UtcNow
        - updated:
            type: datetime
            default_value: UtcNow
        - version:
            type: binary
            default_value: Hex('00')
  - card:
      tags: [datasource_type]
      inherits: base
      fields:
        - label:
            type: string
`,
      }),
      settings: {},
    }, ["sqlite"]);
    const up = files.find((e) => e.filename.endsWith("0001_initial_up.sql"));
    assert.ok(up, "expected initial up migration");
    const sql = "contents" in up ? String(up.contents) : up.content;
    const cards = sql.match(/CREATE TABLE "cards" \(([\s\S]*?)\);/)?.[1];
    assert.ok(cards, "expected cards table body");
    assert.match(cards, /"uuid" TEXT .*DEFAULT \(lower\(hex\(randomblob/);
    assert.match(cards, /"created" TEXT .*strftime\('%Y-%m-%dT%H:%M:%fZ'/);
    assert.match(cards, /"updated" TEXT .*strftime\('%Y-%m-%dT%H:%M:%fZ'/);
    assert.match(cards, /"version" BLOB .*DEFAULT X'00'/);
  });

  it("infers a reference column type from the parent id", async () => {
    const files = await generateSql({
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
    }, ["sqlite"]);
    const up = files.find((e) => e.filename.endsWith("0001_initial_up.sql"));
    assert.ok(up, "expected initial up migration");
    const sql = "contents" in up ? String(up.contents) : up.content;
    const contacts = sql.match(/CREATE TABLE "contacts" \(([\s\S]*?)\);/)?.[1];
    assert.ok(contacts, "expected contacts table body");
    assert.match(contacts, /"gender_type_id" INTEGER/);
    assert.doesNotMatch(contacts, /"gender_type_id" TEXT/);
  });
});

describe("generate-sql collection fields", () => {
  it("omits view collections from table columns and keeps the child FK", async () => {
    const files = await generateSql({
      reader: memoryReader({
        "types.yaml": `types:
  - contact:
      tags: [datasource_type, view_type]
      inherits: set
      fields:
        - email:
            type: string
        - addresses:
            type: address[]
            references: address.contact_id
  - address:
      tags: [datasource_type, view_type]
      inherits: set
      fields:
        - contact_id:
            type: integer
            references: contact.id
        - city:
            type: string
`,
      }),
      settings: {},
    }, ["sqlite"]);
    const up = files.find((e) => e.filename.endsWith("0001_initial_up.sql"));
    assert.ok(up, "expected initial up migration");
    const sql = "contents" in up ? String(up.contents) : up.content;
    assert.match(sql, /CREATE TABLE "contacts"/);
    assert.match(sql, /CREATE TABLE "addresses"/);
    const contacts = sql.match(/CREATE TABLE "contacts" \(([\s\S]*?)\);/)?.[1];
    assert.ok(contacts, "expected contacts table body");
    assert.match(contacts, /"email"/);
    assert.doesNotMatch(contacts, /"addresses"/);
    assert.match(sql, /"contact_id"/);
    assert.match(sql, /FOREIGN KEY \("contact_id"\) REFERENCES "contacts"/);
  });

  it("emits a dictionary table with unique owner and key, flattening object values", async () => {
    const files = await generateSql({
      reader: memoryReader({
        "types.yaml": `types:
  - contact:
      tags: [datasource_type]
      inherits: set
      fields:
        - email:
            type: string
  - locale_preferences:
      tags: [view_type]
      fields:
        - locale:
            type: string
        - timezone:
            type: string
  - contact_settings:
      tags: [datasource_type]
      inherits: set
      fields:
        - contact_id:
            type: integer
            references: contact.id
        - key:
            type: string
        - value:
            type: string
  - contact_preferences:
      tags: [datasource_type]
      inherits: set
      fields:
        - contact_id:
            type: integer
            references: contact.id
        - key:
            type: string
        - locale:
            type: string
        - timezone:
            type: string
  - contact_settings_map:
      tags: [view_type]
      inherits: dictionary
      fields:
        - key:
            type: string
        - value:
            type: string
  - contact_preferences_map:
      tags: [view_type]
      inherits: dictionary
      fields:
        - key:
            type: string
        - value:
            type: locale_preferences
  - contact_view:
      tags: [view_type]
      inherits: contact
      fields:
        - settings:
            type: contact_settings_map{}
            references: contact_settings.contact_id
        - prefs:
            type: contact_preferences_map{}
            references: contact_preferences.contact_id
  - phone:
      tags: [datasource_type]
      inherits: set
  - phone_attribute:
      tags: [datasource_type]
      inherits: set
      fields:
        - phone_id:
            type: integer
            references: phone.id
        - attribute_name:
            type: string
        - attribute_value:
            type: string
  - phone_attribute_map:
      tags: [view_type]
      inherits: dictionary
      fields:
        - key:
            type: string
            references: phone_attribute.attribute_name
        - value:
            type: string
            references: phone_attribute.attribute_value
  - phone_view:
      tags: [view_type]
      inherits: phone
      fields:
        - attributes:
            type: phone_attribute_map{}
            references: phone_attribute.phone_id
  - card_labels:
      tags: [view_type]
      inherits: dictionary
      fields:
        - key:
            type: string
        - value:
            type: string
`,
      }),
      settings: {},
    }, ["sqlite"]);
    const up = files.find((e) => e.filename.endsWith("0001_initial_up.sql"));
    assert.ok(up, "expected initial up migration");
    const sql = "contents" in up ? String(up.contents) : up.content;
    assert.match(sql, /CREATE TABLE "contact_settings"/);
    assert.match(sql, /CREATE TABLE "contact_preferences"/);
    assert.doesNotMatch(sql, /CREATE TABLE "card_labels"/);
    const settings = sql.match(
      /CREATE TABLE "contact_settings" \(([\s\S]*?)\);/,
    )?.[1];
    assert.ok(settings, "expected contact_settings table body");
    assert.match(settings, /"contact_id"/);
    assert.match(settings, /"key"/);
    assert.match(settings, /"value"/);
    assert.match(settings, /"id"/);
    const prefs = sql.match(/CREATE TABLE "contact_preferences" \(([\s\S]*?)\);/)?.[1];
    assert.ok(prefs, "expected contact_preferences table body");
    assert.match(prefs, /"locale"/);
    assert.match(prefs, /"timezone"/);
    assert.doesNotMatch(prefs, /"value"/);
    assert.match(sql, /CREATE UNIQUE INDEX .* ON "contact_settings"/);
    assert.match(sql, /CREATE UNIQUE INDEX .* ON "contact_preferences"/);
    assert.match(sql, /CREATE UNIQUE INDEX .* ON "phone_attributes".*"phone_id".*"attribute_name"/);
  });
});

const DEFAULT_DATASOURCE_YAML = `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type" || tag == "view_type"
types: []
`;

const ENGINE_DATASOURCE_YAML = `version: 1.0.0
types:
  - user:
      fields:
        - email:
            type: string
`;

describe("generate-sql version diffs", () => {
  it("emits numbered up/down files from frozen diffs", async () => {
    const path = "version-20261016T000000Z-001/diff.yaml";
    const diff = stringify({
      "types.yaml": { added: true, content: parse(TYPES_YAML) },
    });
    const files = names(
      await generateSql(
        {
          reader: memoryReader({
            "types.yaml": TYPES_YAML,
            "datasource.yaml": DEFAULT_DATASOURCE_YAML,
            [path]: diff,
          }),
          settings: { "version.diffs": path },
        },
        ["sqlite"],
      ),
    );
    assert.deepEqual(files, ["sqlite/migrations/0001_down.sql", "sqlite/migrations/0001_up.sql"]);
  });

  it("accepts a dialect covered by datasource.<dialect>.yaml", async () => {
    const path = "version-20261016T000000Z-001/diff.yaml";
    const diff = stringify({
      "types.yaml": { added: true, content: parse(TYPES_YAML) },
    });
    const files = names(
      await generateSql(
        {
          reader: memoryReader({
            "types.yaml": TYPES_YAML,
            "datasource.sqlite.yaml": ENGINE_DATASOURCE_YAML,
            [path]: diff,
          }),
          settings: { "version.diffs": path },
        },
        ["sqlite"],
      ),
    );
    assert.deepEqual(files, ["sqlite/migrations/0001_down.sql", "sqlite/migrations/0001_up.sql"]);
  });

  it("throws when live yaml drifted from frozen diffs", async () => {
    const path = "version-20261016T000000Z-001/diff.yaml";
    const diff = stringify({
      "types.yaml": { added: true, content: parse(TYPES_YAML) },
    });
    await assert.rejects(
      () =>
        generateSql(
          {
            reader: memoryReader({
              "types.yaml": `types:
  - extra:
      tags: [datasource_type]
      inherits: set
      fields:
        - name:
            type: string
`,
              "datasource.yaml": DEFAULT_DATASOURCE_YAML,
              [path]: diff,
            }),
            settings: { "version.diffs": path },
          },
          ["sqlite"],
        ),
      /run datasources version first/,
    );
  });

  it("emits a later version for added seeds", async () => {
    const gender = `types:
  - gender:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
`;
    const seeds = `version: 1.0.0
seeds:
  - gender:
      - id1:
          name: Male
`;
    const first = "version-20261016T000000Z-001/diff.yaml";
    const second = "version-20261016T000000Z-002/diff.yaml";
    const files = names(
      await generateSql(
        {
          reader: memoryReader({
            "types.yaml": gender,
            "datasource.yaml": DEFAULT_DATASOURCE_YAML,
            "datasource_seeds.yaml": seeds,
            [first]: stringify({ "types.yaml": { added: true, content: parse(gender) } }),
            [second]: stringify({ "datasource_seeds.yaml": { added: true, content: parse(seeds) } }),
          }),
          settings: { "version.diffs": `${first}\n${second}` },
        },
        ["sqlite"],
      ),
    );
    assert.deepEqual(files, [
      "sqlite/migrations/0001_down.sql",
      "sqlite/migrations/0001_up.sql",
      "sqlite/migrations/0002_down.sql",
      "sqlite/migrations/0002_up.sql",
    ]);
    const entries = await generateSql(
      {
        reader: memoryReader({
          "types.yaml": gender,
          "datasource.yaml": DEFAULT_DATASOURCE_YAML,
          "datasource_seeds.yaml": seeds,
          [first]: stringify({ "types.yaml": { added: true, content: parse(gender) } }),
          [second]: stringify({ "datasource_seeds.yaml": { added: true, content: parse(seeds) } }),
        }),
        settings: { "version.diffs": `${first}\n${second}` },
      },
      ["sqlite"],
    );
    const up2 = entries.find((entry) => entry.filename === "sqlite/migrations/0002_up.sql");
    const sql = up2 != null && "contents" in up2 ? String(up2.contents) : up2?.content ?? "";
    assert.match(sql, /INSERT INTO "genders"/);
    assert.match(sql, /Male/);
    const down2 = entries.find((entry) => entry.filename === "sqlite/migrations/0002_down.sql");
    const down = down2 != null && "contents" in down2 ? String(down2.contents) : down2?.content ?? "";
    assert.match(down, /DELETE FROM "genders" WHERE "id" = 1/);
  });

  it("throws when a root datasource.yaml drifted from the frozen diffs", async () => {
    const path = "version-20261016T000000Z-001/diff.yaml";
    const diff = stringify({
      "types.yaml": { added: true, content: parse(TYPES_YAML) },
    });
    await assert.rejects(
      () =>
        generateSql(
          {
            reader: memoryReader({
              "types.yaml": TYPES_YAML,
              "datasource.yaml": ENGINE_DATASOURCE_YAML,
              [path]: diff,
            }),
            settings: { "version.diffs": path },
          },
          ["sqlite"],
        ),
      /run datasources version first/,
    );
  });
});
