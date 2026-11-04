import { DatasourceError } from "@deterministic-code/datasources";
import { describe, expect, it } from "vitest";
import { Api } from "./api.ts";
import "./datasources-register.ts";

const repo = (opts?: { read?: object[]; single?: object | undefined; create?: object; error?: Error }) =>
  ({
    read: async function* () {
      if (opts?.error) throw opts.error;
      for (const row of opts?.read ?? []) yield row;
    },
    readSingle: async () => {
      if (opts?.error) throw opts.error;
      return opts?.single;
    },
    create: async (row: object) => opts?.create ?? { ...row, id: 1 },
    update: async () => undefined,
    delete: async () => undefined,
  }) as never;

const apiOf = (opts?: { users?: object; error?: Error }) =>
  new Api(
    repo({ create: opts?.users ?? { id: 1, username: "ada" }, single: opts?.users ?? { id: 1, username: "ada" }, read: [opts?.users ?? { id: 1, username: "ada" }], error: opts?.error }),
    repo({ create: { id: 2, name: "arcade" }, single: { id: 2, name: "arcade" }, read: [{ id: 2, name: "arcade" }] }),
    repo({ create: { id: 3, name: "star" }, single: { id: 3, name: "star" }, read: [{ id: 3, name: "star" }] }),
    repo({ create: { id: 4, description: "fun" }, single: { id: 4, description: "fun" }, read: [{ id: 4 }] }),
    repo({ create: { id: 5, state: "intro" }, single: { id: 5, state: "intro" }, read: [{ id: 5 }] }),
    repo({ create: { id: 6, firstName: "Ada" }, single: { id: 6, firstName: "Ada" }, read: [{ id: 6 }] }),
  );

describe("api", () => {
  it("maps datasource errors", async () => {
    expect((await apiOf({ error: new DatasourceError("missing") }).handle("GET", "/users", undefined)).status).toBe(400);
  });

  it("rethrows unexpected errors", async () => {
    await expect(apiOf({ error: new Error("boom") }).handle("GET", "/users", undefined)).rejects.toThrow("boom");
  });

  it("rejects unknown routes lookups and missing rows", async () => {
    const api = new Api(repo(), repo(), repo(), repo(), repo(), repo());
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
    const api = apiOf();
    expect((await api.handle("POST", "/users/", JSON.stringify({
      username: "ada",
      password: "x",
      uuid: "11111111-1111-1111-1111-111111111111",
      created: "2020-01-02T03:04:05.000Z",
      version: "AA==",
      settings: { theme: "dark" },
      roles: [{ name: "player" }],
      groups: [{ name: "guild" }],
    }))).status).toBe(201);
    expect((await api.handle("GET", "/users", undefined)).status).toBe(200);
    expect((await api.handle("GET", "/users/?id=1&username=ada", undefined)).status).toBe(200);
    expect((await api.handle("PUT", "/users/?id=1", JSON.stringify({ username: "ada", password: "x" }))).status).toBe(200);
    expect((await api.handle("DELETE", "/users/?id=1", undefined)).status).toBe(204);
    expect((await api.handle("GET", "/categories", undefined)).status).toBe(200);
    expect((await api.handle("GET", "/categories?categoryId=", undefined)).status).toBe(200);
    expect((await api.handle("GET", "/categories?category_id=2", undefined)).status).toBe(200);
    expect((await api.handle("POST", "/categories/", JSON.stringify({ name: "arcade" }))).status).toBe(201);
    expect((await api.handle("PUT", "/categories/?id=2", JSON.stringify({ name: "arcade" }))).status).toBe(200);
    expect((await api.handle("DELETE", "/categories/?id=2", undefined)).status).toBe(204);
    expect((await api.handle("POST", "/games/", JSON.stringify({ name: "star", settings: { lives: "3" }, categories: [{ name: "arcade" }] }))).status).toBe(201);
    expect((await api.handle("GET", "/games/?id=3&name=star", undefined)).status).toBe(200);
    expect((await api.handle("PUT", "/games/?id=3", JSON.stringify({ name: "star" }))).status).toBe(200);
    expect((await api.handle("DELETE", "/games/?id=3", undefined)).status).toBe(204);
    expect((await api.handle("POST", "/reviews/", JSON.stringify({ gameId: 3, userId: 1, description: "fun", rating: "4.5" }))).status).toBe(201);
    expect((await api.handle("GET", "/reviews/?gameId=3&userId=1", undefined)).status).toBe(200);
    expect((await api.handle("PUT", "/reviews/?id=4", JSON.stringify({ description: "great" }))).status).toBe(200);
    expect((await api.handle("DELETE", "/reviews/?id=4", undefined)).status).toBe(204);
    expect((await api.handle("POST", "/game_states/", JSON.stringify({ gameId: 3, userId: 1, state: "intro" }))).status).toBe(201);
    expect((await api.handle("GET", "/game_states/?id=5", undefined)).status).toBe(200);
    expect((await api.handle("PUT", "/game_states/?id=5", JSON.stringify({ state: "boss" }))).status).toBe(200);
    expect((await api.handle("DELETE", "/game_states/?id=5", undefined)).status).toBe(204);
    expect((await api.handle("POST", "/contacts/", JSON.stringify({
      firstName: "Ada",
      lastName: "Lovelace",
      email: "ada@example.com",
      notes: "poet",
      contactSourceName: "Manual",
      userId: 1,
      uuid: "22222222-2222-2222-2222-222222222222",
      created: "2020-01-02T03:04:05.000Z",
      version: "AA==",
    }))).status).toBe(201);
    expect((await api.handle("GET", "/contacts/?userId=1", undefined)).status).toBe(200);
    expect((await api.handle("PUT", "/contacts/?id=6", JSON.stringify({ firstName: "Ada", lastName: "Lovelace", contactSourceName: "Manual", userId: 1 }))).status).toBe(200);
    expect((await api.handle("DELETE", "/contacts/?id=6", undefined)).status).toBe(204);
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
    expect((await api.handle("GET", "/users/?id=&lonely&username=ada", undefined)).status).toBe(200);
  });
});
