import { describe, expect, it } from "vitest";
import { CRUDRepositoryBuilder } from "./CRUDRepositoryBuilder.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import { MockConnection } from "./mocks/mock-connection.ts";
import { RepositoryBuilderOptionsFactory } from "./RepositoryBuilderOptionsFactory.ts";

describe("RepositoryBuilderOptionsFactory", () => {
  it("returns the dialect options or the default", () => {
    const fallback: CRUDRepositoryOptions = { createProperties: ["name"] };
    const mysql: CRUDRepositoryOptions = { createProperties: ["id", "name"] };
    const factory = new RepositoryBuilderOptionsFactory().add(fallback).add("mysql", mysql);
    expect(factory.for("mysql")).toBe(mysql);
    expect(factory.for("sqlite")).toBe(fallback);
  });

  it("builder selects by connection provider", () => {
    const fallback: CRUDRepositoryOptions = { createProperties: ["name"] };
    const mysql: CRUDRepositoryOptions = { createProperties: ["id"] };
    const builder = new CRUDRepositoryBuilder().add(fallback).add("mysql", mysql);
    expect(builder.for(new MockConnection(undefined, "mysql"))).toBe(mysql);
    expect(builder.for(new MockConnection(undefined, "sqlite"))).toBe(fallback);
  });

  it("throws when no options were added", () => {
    expect(() => new RepositoryBuilderOptionsFactory().for("mysql")).toThrow("repository options required");
  });
});
