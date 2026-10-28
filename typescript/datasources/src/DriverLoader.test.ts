import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import { DriverLoader } from "./DriverLoader.ts";
import { ProviderCatalog } from "./ProviderCatalog.ts";
import { ProviderNotInstalledError } from "./ProviderNotInstalledError.ts";

const roots: string[] = [];
const cwd = process.cwd();

afterEach(() => {
  process.chdir(cwd);
  for (const root of roots.splice(0)) rmSync(root, { recursive: true, force: true });
});

const project = (files: Record<string, string>): string => {
  const root = mkdtempSync(join(tmpdir(), "datasources-loader-"));
  roots.push(root);
  writeFileSync(join(root, "package.json"), "{}");
  for (const [name, text] of Object.entries(files)) {
    const path = join(root, name);
    mkdirSync(join(path, ".."), { recursive: true });
    writeFileSync(path, text);
  }
  process.chdir(root);
  return root;
};

describe("DriverLoader", () => {
  it("constructs the exported driver from cwd", () => {
    project({
      "node_modules/fixture-driver/package.json": JSON.stringify({ name: "fixture-driver", main: "index.cjs" }),
      "node_modules/fixture-driver/index.cjs": "exports.FixtureDriver = class FixtureDriver {};\n",
    });
    const driver = DriverLoader.load(new ProviderCatalog("fixture", "fixture-driver", "FixtureDriver"));
    expect(driver.constructor.name).toBe("FixtureDriver");
  });

  it("throws when the export is missing", () => {
    project({
      "node_modules/fixture-driver/package.json": JSON.stringify({ name: "fixture-driver", main: "index.cjs" }),
      "node_modules/fixture-driver/index.cjs": "exports.OtherDriver = class OtherDriver {};\n",
    });
    expect(() => DriverLoader.load(new ProviderCatalog("fixture", "fixture-driver", "FixtureDriver"))).toThrow(
      ProviderNotInstalledError,
    );
  });

  it("throws when the package is not in cwd", () => {
    project({});
    expect(() =>
      DriverLoader.load(new ProviderCatalog("missing", "datasources-not-installed-driver", "MissingDriver")),
    ).toThrow(ProviderNotInstalledError);
  });
});
