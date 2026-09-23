import { readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const providers = join(dirname(fileURLToPath(import.meta.url)), "../../providers");

describe("provider public surface", () => {
  it("exports only the provider", () => {
    const names = readdirSync(providers, { withFileTypes: true })
      .filter((entry) => entry.isDirectory() && entry.name !== "node_modules")
      .map((entry) => entry.name);
    expect(names.length).toBeGreaterThanOrEqual(12);
    for (const name of names) {
      const source = readFileSync(join(providers, name, "src/index.ts"), "utf8");
      const match = source.match(
        /^export \{ ([A-Za-z0-9]+Provider) \} from "\.\/([A-Za-z0-9]+Provider)\.ts";\n$/,
      );
      expect(match, name).not.toBeNull();
      expect(match?.[1]).toBe(match?.[2]);
    }
  });
});
