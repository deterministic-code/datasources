import { describe, expect, it } from "vitest";
import { cargoProviders, csharpProviders, npmProviders, withRegisterFeatures } from "./installed.ts";

describe("installed", () => {
  it("reads npm workspace packages", () => {
    expect(
      npmProviders({
        dependencies: { "@datasources/sqlite": "0.1.0", lodash: "4" },
        devDependencies: { "@datasources/json": "*" },
      }),
    ).toEqual(["json", "sqlite"]);
    expect(npmProviders({})).toEqual([]);
  });

  it("reads cargo crates and register features", () => {
    expect(
      cargoProviders(`
datasources-register = { version = "0.1.0", features = ["json", "memory"] }
datasources-sqlite = { path = "../sqlite" }
`),
    ).toEqual(["json", "memory", "sqlite"]);
    expect(cargoProviders(`datasources-json = "0.1.0"`)).toEqual(["json"]);
    expect(cargoProviders(`foo = { package = "datasources-json" }`)).toEqual(["json"]);
    expect(cargoProviders("")).toEqual([]);
  });

  it("reads csharp package and project references", () => {
    expect(
      csharpProviders(`
<PackageReference Include="Datasources.Sqlite" />
<ProjectReference Include="..\\Json\\Datasources.Json.csproj" />
`),
    ).toEqual(["json", "sqlite"]);
    expect(csharpProviders("")).toEqual([]);
  });

  it("rewrites datasources-register features", () => {
    const toml = `datasources-register = { version = "0.1.0", features = ["json", "memory"] }`;
    expect(withRegisterFeatures(toml, ["sqlite"])).toBe(
      `datasources-register = { version = "0.1.0", features = ["sqlite"] }`,
    );
    expect(withRegisterFeatures("datasources = \"0.1.0\"", ["json"])).toBe(
      `datasources = "0.1.0"`,
    );
  });
});
