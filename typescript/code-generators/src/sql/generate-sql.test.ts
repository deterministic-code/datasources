import assert from "node:assert/strict";
import { describe, it } from "vitest";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { generate } from "./generate-sql.ts";

describe("generate-sql", () => {
  it("throws when the dialect is not installed", async () => {
    await assert.rejects(
      () =>
        generate(
          {
            reader: memoryReader({ "types.yaml": "types: []" }),
            settings: {},
          },
          ["acme"],
        ),
      /Unknown SQL dialect/,
    );
  });
});
