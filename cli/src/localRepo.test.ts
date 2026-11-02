import { describe, expect, it } from "vitest";
import { cliDatasourcesRepo, requireDatasourcesRepo } from "./localRepo.ts";

describe("localRepo", () => {
  it("returns this checkout", () => {
    expect(requireDatasourcesRepo()).toBe(cliDatasourcesRepo);
  });

  it("rejects a path that is not this checkout", () => {
    expect(() => requireDatasourcesRepo("/tmp")).toThrow("--local is for developing this datasources checkout");
  });
});
