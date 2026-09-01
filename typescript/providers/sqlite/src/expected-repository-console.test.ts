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

  it("expands complex-crud-mappings into flattened StatementBuilder SQL", () => {
    const mapped = parse(
      readFileSync(join(repoRoot(), "specs", "repositories", "complex-crud-mappings.yaml"), "utf8"),
    ) as RepositorySpec;
    expect(repositoryQueries(mapped)).toEqual([
      "INSERT INTO users (create_string, decimal_text, id, username, usertypename, uuid, version) VALUES ($create_string, $decimal_text, $id, $username, $usertypename, $uuid, $version)",
      "INSERT INTO users (create_string, decimal_text, id, username, usertypename, uuid, version) VALUES ($create_string, $decimal_text, $id, $username, $usertypename, $uuid, $version)",
      "SELECT create_string, decimal_text, id, username, usertypename, uuid, version FROM users WHERE id = $id",
      "SELECT create_string, decimal_text, id, username, usertypename, uuid, version FROM users WHERE username = $username",
      "SELECT create_string, decimal_text, id, username, usertypename, uuid, version FROM users WHERE uuid = $uuid",
      "SELECT create_string, decimal_text, id, username, usertypename, uuid, version FROM users WHERE usertypename = $usertypename",
      "SELECT create_string, decimal_text, id, username, usertypename, uuid, version FROM users WHERE create_string = $create_string",
      "SELECT create_string, decimal_text, id, username, usertypename, uuid, version FROM users",
      "UPDATE users SET create_string = $create_string, decimal_text = $decimal_text, username = $username, usertypename = $usertypename, uuid = $uuid, version = $version WHERE uuid = $uuid",
      "SELECT create_string, decimal_text, id, username, usertypename, uuid, version FROM users WHERE uuid = $uuid",
      "SELECT create_string, decimal_text, id, username, usertypename, uuid, version FROM users WHERE create_string = $create_string",
      "SELECT create_string, decimal_text, id, username, usertypename, uuid, version FROM users WHERE usertypename = $usertypename",
      "DELETE FROM users WHERE create_string = $create_string",
      "SELECT create_string, decimal_text, id, username, usertypename, uuid, version FROM users WHERE id = $id",
      "SELECT create_string, decimal_text, id, username, usertypename, uuid, version FROM users",
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
