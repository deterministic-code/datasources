import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { cliDatasourcesRepo, requireDatasourcesRepo } from "./localRepo.ts";

describe("localRepo", () => {
  it("returns this checkout from cwd", () => {
    expect(requireDatasourcesRepo()).toBe(cliDatasourcesRepo);
  });

  it("walks from a nested consumer directory", () => {
    expect(requireDatasourcesRepo(join(cliDatasourcesRepo, "samples/csharp/contacts-simple-cli"))).toBe(
      cliDatasourcesRepo,
    );
  });

  it("uses DATASOURCES_REPO when it is this checkout", () => {
    const previous = process.env.DATASOURCES_REPO;
    process.env.DATASOURCES_REPO = cliDatasourcesRepo;
    try {
      expect(requireDatasourcesRepo("/tmp")).toBe(cliDatasourcesRepo);
    } finally {
      if (previous === undefined) delete process.env.DATASOURCES_REPO;
      else process.env.DATASOURCES_REPO = previous;
    }
  });

  it("uses the CLI checkout when cwd is not inside it", () => {
    const previous = process.env.DATASOURCES_REPO;
    delete process.env.DATASOURCES_REPO;
    try {
      expect(requireDatasourcesRepo("/tmp")).toBe(cliDatasourcesRepo);
    } finally {
      if (previous === undefined) delete process.env.DATASOURCES_REPO;
      else process.env.DATASOURCES_REPO = previous;
    }
  });

  it("throws when this checkout is missing", () => {
    const previous = process.env.DATASOURCES_REPO;
    delete process.env.DATASOURCES_REPO;
    try {
      expect(() => requireDatasourcesRepo("/tmp", "/tmp")).toThrow("--local is for developing this datasources checkout");
    } finally {
      if (previous === undefined) delete process.env.DATASOURCES_REPO;
      else process.env.DATASOURCES_REPO = previous;
    }
  });
});
