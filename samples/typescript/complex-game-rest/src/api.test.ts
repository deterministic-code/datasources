import { existsSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { Datasource, Native, type IConnection } from "@deterministic-code/datasources";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";
import { describe, expect, it } from "vitest";
import { Api } from "./api.ts";
import "./datasources-register.ts";
import { UsersRepository } from "../generated/repositories/usersRepository.ts";
import { CategoriesRepository } from "../generated/repositories/categoriesRepository.ts";
import { GamesRepository } from "../generated/repositories/gamesRepository.ts";
import { ReviewsRepository } from "../generated/repositories/reviewsRepository.ts";
import { GameStatesRepository } from "../generated/repositories/gameStatesRepository.ts";
import { ContactsRepository } from "../generated/repositories/contactsRepository.ts";
import { RolesRepository } from "../generated/repositories/rolesRepository.ts";
import { GroupsRepository } from "../generated/repositories/groupsRepository.ts";
import { ContactSourcesRepository } from "../generated/repositories/contactSourcesRepository.ts";
import type { Role } from "../generated/types/datasource/role.ts";
import type { Group } from "../generated/types/datasource/group.ts";
import type { ContactSource } from "../generated/types/datasource/contactSource.ts";

const apiOf = (connection: IConnection) =>
  new Api(
    new UsersRepository(connection),
    new CategoriesRepository(connection),
    new GamesRepository(connection),
    new ReviewsRepository(connection),
    new GameStatesRepository(connection),
    new ContactsRepository(connection),
  );

const open = async () => {
  const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
  const connection = await ds.createConnection();
  await connection.open();
  return { api: apiOf(connection), connection };
};

const migrated = async () => {
  const opened = await open();
  const sql = await readFile(join(root(), "sql/sqlite/migrations/0001_initial_up.sql"), "utf8");
  for (const batch of sql.split("GO").map((part) => part.trim()).filter((part) => part.length > 0))
    await opened.connection.createCommand().executeNative(new Native(batch));
  return opened;
};

const root = (): string => {
  let dir: string | undefined = dirname(fileURLToPath(import.meta.url));
  while (dir != null && !existsSync(join(dir, "sql/sqlite/migrations/0001_initial_up.sql")))
    dir = dirname(dir) === dir ? undefined : dirname(dir);
  if (dir == null) throw new Error("sql/sqlite/migrations/0001_initial_up.sql");
  return dir;
};

const idOf = (result: { status: number; body: string }): number => {
  expect(result.status).toBe(201);
  return JSON.parse(result.body).id as number;
};

describe("api", () => {
  it("maps datasource errors", async () => {
    const { api } = await open();
    const result = await api.handle("GET", "/users", undefined);
    expect(result.status).toBe(400);
    expect(result.body.length).toBeGreaterThan(0);
  });

  it("rejects unknown routes lookups and missing rows", async () => {
    const { api } = await migrated();
    expect((await api.handle("GET", "/", undefined)).status).toBe(404);
    expect((await api.handle("GET", "", undefined)).status).toBe(404);
    expect((await api.handle("GET", "/nope", undefined)).status).toBe(404);
    expect((await api.handle("POST", "/users/", "null")).status).toBe(400);
    expect((await api.handle("POST", "/users/", "{")).status).toBe(400);
    expect((await api.handle("PUT", "/users/?id=abc", "{}")).status).toBe(400);
    expect((await api.handle("PUT", "/users/?id=99", JSON.stringify({ username: "ada", password: "x" }))).status).toBe(404);
    expect((await api.handle("DELETE", "/users/?id=99", undefined)).status).toBe(404);
    expect((await api.handle("DELETE", "/users/", undefined)).status).toBe(400);
    expect((await api.handle("GET", "/categories?categoryId=nope", undefined)).status).toBe(400);
    expect((await api.handle("PUT", "/games/?id=99", JSON.stringify({ name: "star" }))).status).toBe(404);
    expect((await api.handle("DELETE", "/reviews/?id=99", undefined)).status).toBe(404);
    expect((await api.handle("PUT", "/game_states/?id=99", JSON.stringify({ state: "x" }))).status).toBe(404);
    expect((await api.handle("PUT", "/contacts/?id=99", JSON.stringify({ firstName: "Ada" }))).status).toBe(404);
    expect((await api.handle("PUT", "/categories/?id=99", JSON.stringify({ name: "root" }))).status).toBe(404);
  });

  it("writes and reads each route", async () => {
    const { api, connection } = await migrated();
    await new RolesRepository(connection).create({ name: "player", description: "plays" } as Role);
    await new GroupsRepository(connection).create({ name: "guild", description: "guild" } as Group);
    await new ContactSourcesRepository(connection).create({ name: "Manual" } as ContactSource);
    let userId = idOf(await api.handle("POST", "/users/", JSON.stringify({
      username: "ada",
      password: "x",
      uuid: "11111111-1111-1111-1111-111111111111",
      created: "2020-01-02T03:04:05.000Z",
      version: "AA==",
      settings: { theme: "dark" },
      roles: [{ name: "player" }],
      groups: [{ name: "guild" }],
    })));
    expect((await api.handle("GET", "/users", undefined)).status).toBe(200);
    expect((await api.handle("GET", `/users/?id=${userId}&username=ada`, undefined)).status).toBe(200);
    expect((await api.handle("PUT", `/users/?id=${userId}`, JSON.stringify({ username: "ada", password: "x" }))).status).toBe(200);
    expect((await api.handle("DELETE", `/users/?id=${userId}`, undefined)).status).toBe(204);
    expect((await api.handle("GET", "/categories", undefined)).status).toBe(200);
    expect((await api.handle("GET", "/categories?categoryId=", undefined)).status).toBe(200);
    const categoryId = idOf(await api.handle("POST", "/categories/", JSON.stringify({ name: "arcade" })));
    expect((await api.handle("GET", `/categories?category_id=${categoryId}`, undefined)).status).toBe(200);
    expect((await api.handle("PUT", `/categories/?id=${categoryId}`, JSON.stringify({ name: "arcade" }))).status).toBe(200);
    expect((await api.handle("DELETE", `/categories/?id=${categoryId}`, undefined)).status).toBe(204);
    let gameId = idOf(await api.handle("POST", "/games/", JSON.stringify({ name: "star", settings: { lives: "3" }, categories: [{ name: "arcade" }] })));
    expect((await api.handle("GET", `/games/?id=${gameId}&name=star`, undefined)).status).toBe(200);
    expect((await api.handle("PUT", `/games/?id=${gameId}`, JSON.stringify({ name: "star" }))).status).toBe(200);
    expect((await api.handle("DELETE", `/games/?id=${gameId}`, undefined)).status).toBe(204);
    userId = idOf(await api.handle("POST", "/users/", JSON.stringify({ username: "ada", password: "x" })));
    gameId = idOf(await api.handle("POST", "/games/", JSON.stringify({ name: "star" })));
    const reviewId = idOf(await api.handle("POST", "/reviews/", JSON.stringify({ gameId, userId, description: "fun", rating: "4.5" })));
    expect((await api.handle("GET", `/reviews/?gameId=${gameId}&userId=${userId}`, undefined)).status).toBe(200);
    expect((await api.handle("PUT", `/reviews/?id=${reviewId}`, JSON.stringify({ gameId, userId, description: "great" }))).status).toBe(200);
    expect((await api.handle("DELETE", `/reviews/?id=${reviewId}`, undefined)).status).toBe(204);
    const stateId = idOf(await api.handle("POST", "/game_states/", JSON.stringify({ gameId, userId, state: "intro" })));
    expect((await api.handle("GET", `/game_states/?id=${stateId}`, undefined)).status).toBe(200);
    expect((await api.handle("PUT", `/game_states/?id=${stateId}`, JSON.stringify({ gameId, userId, state: "boss" }))).status).toBe(200);
    expect((await api.handle("DELETE", `/game_states/?id=${stateId}`, undefined)).status).toBe(204);
    const contactId = idOf(await api.handle("POST", "/contacts/", JSON.stringify({
      firstName: "Ada",
      lastName: "Lovelace",
      email: "ada@example.com",
      notes: "poet",
      contactSourceName: "Manual",
      userId,
      uuid: "22222222-2222-2222-2222-222222222222",
      created: "2020-01-02T03:04:05.000Z",
      version: "AA==",
    })));
    expect((await api.handle("GET", `/contacts/?userId=${userId}`, undefined)).status).toBe(200);
    expect((await api.handle("PUT", `/contacts/?id=${contactId}`, JSON.stringify({ firstName: "Ada", lastName: "Lovelace", contactSourceName: "Manual", userId }))).status).toBe(200);
    expect((await api.handle("DELETE", `/contacts/?id=${contactId}`, undefined)).status).toBe(204);
    expect((await api.handle("POST", "/users/", JSON.stringify({
      username: "x",
      password: "y",
      uuid: "",
      settings: null,
      roles: null,
      groups: null,
    }))).status).toBe(201);
    expect((await api.handle("POST", "/games/", JSON.stringify({ name: "z", settings: null, categories: null }))).status).toBe(201);
    expect((await api.handle("POST", "/contacts/", JSON.stringify({
      firstName: "A",
      lastName: "B",
      notes: "",
      uuid: "",
      addresses: null,
      phones: null,
      settings: null,
      prefs: null,
    }))).status).toBe(201);
    expect((await api.handle("POST", "/users/", " ")).status).toBe(201);
    expect((await api.handle("GET", "/users/?id=&lonely&username=x", undefined)).status).toBe(200);
  });
});
