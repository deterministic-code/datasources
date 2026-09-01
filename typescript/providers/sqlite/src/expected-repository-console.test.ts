import { readFileSync } from "node:fs";
import { join } from "node:path";
import { parse } from "yaml";
import { describe, expect, it } from "vitest";
import {
  expectedRepositoryConsoleSignatures,
  repositoryQueries,
  type RepositorySpec,
} from "../../../specs/expected-repository-console.ts";
import { repoRoot } from "../../../sql-spec-runner.ts";

describe("expected repository console", () => {
  const spec = parse(
    readFileSync(join(repoRoot(), "specs", "repositories", "user-crud.yaml"), "utf8"),
  ) as RepositorySpec;

  it("expands user-crud into StatementBuilder SQL", () => {
    expect(repositoryQueries(spec)).toEqual([
      "INSERT INTO user (balance, created, id, is_enabled, location, username, uuid, version) VALUES ($balance, $created, $id, $is_enabled, $location, $username, $uuid, $version)",
      "INSERT INTO user (balance, created, id, is_enabled, location, username, uuid, version) VALUES ($balance, $created, $id, $is_enabled, $location, $username, $uuid, $version)",
      "SELECT balance, created, id, is_enabled, location, username, uuid, version FROM user WHERE id = $id",
      "SELECT balance, created, id, is_enabled, location, username, uuid, version FROM user WHERE username = $username",
      "SELECT balance, created, id, is_enabled, location, username, uuid, version FROM user WHERE uuid = $uuid",
      "SELECT balance, created, id, is_enabled, location, username, uuid, version FROM user WHERE is_enabled = $is_enabled",
      "SELECT balance, created, id, is_enabled, location, username, uuid, version FROM user WHERE created = $created",
      "SELECT balance, created, id, is_enabled, location, username, uuid, version FROM user",
      "UPDATE user SET balance = $balance, created = $created, is_enabled = $is_enabled, location = $location, username = $username, uuid = $uuid, version = $version WHERE uuid = $uuid",
      "SELECT balance, created, id, is_enabled, location, username, uuid, version FROM user WHERE uuid = $uuid",
      "SELECT balance, created, id, is_enabled, location, username, uuid, version FROM user WHERE created = $created",
      "SELECT balance, created, id, is_enabled, location, username, uuid, version FROM user WHERE is_enabled = $is_enabled",
      "DELETE FROM user WHERE created = $created",
      "SELECT balance, created, id, is_enabled, location, username, uuid, version FROM user WHERE id = $id",
      "SELECT balance, created, id, is_enabled, location, username, uuid, version FROM user",
    ]);
  });

  it("pairs Start and Finish signatures", () => {
    const signatures = expectedRepositoryConsoleSignatures(spec);
    expect(signatures).toHaveLength(30);
    expect(signatures[0]).toBe(
      "[datasource][Start]-[INSERT INTO user (balance, created, id, is_enabled, location, username, uuid, version) VALUES ($balance, $created, $id, $is_enabled, $location, $username, $uuid, $version)]",
    );
    expect(signatures[1]).toBe(
      "[datasource][Finish]-[INSERT INTO user (balance, created, id, is_enabled, location, username, uuid, version) VALUES ($balance, $created, $id, $is_enabled, $location, $username, $uuid, $version)]",
    );
  });
});
