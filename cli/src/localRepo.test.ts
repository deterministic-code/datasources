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

  it("rejects a path that is not this checkout", () => {
    expect(() => requireDatasourcesRepo("/tmp")).toThrow("--local is for developing this datasources checkout");
  });
});
