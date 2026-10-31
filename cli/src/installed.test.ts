import { describe, expect, it } from "vitest";
import { cargoProviders, csharpProviders, hasCsharpProjectReference, npmProviders } from "./installed.ts";

describe("installed", () => {
  it("reads npm workspace packages", () => {
    expect(
      npmProviders({
        dependencies: { "@deterministic-code/datasources-sqlite": "0.1.0", lodash: "4" },
        devDependencies: { "@deterministic-code/datasources-json": "*" },
      }),
    ).toEqual(["json", "sqlite"]);
    expect(npmProviders({})).toEqual([]);
  });

  it("reads cargo crates", () => {
    expect(
      cargoProviders(`
datasources-json = { version = "0.1.0" }
datasources-sqlite = { path = "../sqlite" }
`),
    ).toEqual(["json", "sqlite"]);
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
    expect(hasCsharpProjectReference(`<ProjectReference Include="Datasources.Sqlite.csproj" />`, "Datasources.Sqlite")).toBe(
      true,
    );
    expect(hasCsharpProjectReference(`<PackageReference Include="Datasources.Sqlite" />`, "Datasources.Sqlite")).toBe(
      false,
    );
  });
});
