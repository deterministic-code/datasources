import assert from "node:assert/strict";
import { describe, it } from "vitest";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { generate } from "@deterministic-code/datasources-code-generators/sql/generate-sql";

const TYPES = `types:
  - user:
      tags: [datasource_type]
      inherits: set
      fields:
        - email:
            type: string
  - notification_type:
      tags: [datasource_type]
      inherits: set
      fields:
        - channel_name:
            type: string
        - user_id:
            type: integer
            references: user.id
`;

describe("generate-sql quotes", () => {
  it("keywords lower keeps sqlite quoted object names", async () => {
    const files = await generate(
      {
        reader: memoryReader({ "types.yaml": TYPES }),
        settings: {
          "datasource.casing.keywords": "lower",
          "datasource.casing.objects": "upper",
        },
      },
      ["sqlite"],
    );
    const entry = files.find((file) => file.filename === "sqlite/migrations/0001_initial_up.sql");
    const sql = entry === undefined ? "" : "contents" in entry && entry.contents !== undefined ? String(entry.contents) : String(entry.content ?? "");
    assert.match(sql, /create table "NOTIFICATION_TYPES"/);
    assert.match(sql, /not null/);
  });
});
