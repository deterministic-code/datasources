import "../generated/typescript/repositories/repositoryRegistry.ts";
import "./datasources-register.ts";
import { Datasource } from "@deterministic-code/datasources";
import { Api } from "./api.ts";
import type { Role } from "../generated/typescript/types/generated/datasource/role.ts";
import type { Group } from "../generated/typescript/types/generated/datasource/group.ts";
import type { ContactSource } from "../generated/typescript/types/generated/datasource/contactSource.ts";

await using ds = Datasource.default as Datasource & {
  users: never;
  categories: never;
  games: never;
  reviews: never;
  gameStates: never;
  contacts: never;
  roles: { create: (row: Role) => Promise<Role> };
  groups: { create: (row: Group) => Promise<Group> };
  contactSources: { create: (row: ContactSource) => Promise<ContactSource> };
};
const api = new Api(ds.users, ds.categories, ds.games, ds.reviews, ds.gameStates, ds.contacts);
const role = await ds.roles.create({ name: "player", description: "plays" } as Role);
const group = await ds.groups.create({ name: "guild", description: "guild" } as Group);
await ds.contactSources.create({ name: "Manual" } as ContactSource);

const user = await call(api, "POST", "/users", {
  username: "ada",
  password: "secret",
  settings: { theme: "dark" },
  roles: [{ id: role.id, name: role.name, description: role.description }],
  groups: [{ id: group.id, name: group.name, description: group.description }],
}, 201);
const userId = user.id as number;
const listed = await call(api, "GET", "/users?username=ada", undefined, 200);
if (!Array.isArray(listed) || listed.length !== 1) throw new Error("get users username");
await call(api, "PUT", `/users?id=${userId}`, {
  username: "ada-byron",
  password: "secret",
  settings: { theme: "sepia" },
  roles: [{ id: role.id, name: role.name }],
  groups: [{ id: group.id, name: group.name }],
}, 200);
const loaded = await call(api, "GET", `/users?id=${userId}`, undefined, 200);
if (!Array.isArray(loaded) || loaded[0]?.username !== "ada-byron") throw new Error("put users");

const root = await call(api, "POST", "/categories", { name: "arcade", description: "root" }, 201);
const rootId = root.id as number;
const child = await call(api, "POST", "/categories", { name: "shooter", description: "child", categoryId: rootId }, 201);
const childId = child.id as number;
const roots = await call(api, "GET", "/categories?category_id=", undefined, 200);
if (!Array.isArray(roots) || roots.length !== 1 || roots[0]?.id !== rootId) throw new Error("get category roots");
const children = await call(api, "GET", `/categories?categoryId=${rootId}`, undefined, 200);
if (!Array.isArray(children) || children.length !== 1 || children[0]?.id !== childId) throw new Error("get category children");

const game = await call(api, "POST", "/games", {
  name: "star",
  description: "space",
  location: "orbit",
  settings: { lives: "3" },
  categories: [{ id: childId, name: "shooter" }],
}, 201);
const gameId = game.id as number;
await call(api, "GET", `/games?id=${gameId}`, undefined, 200);
await call(api, "PUT", `/games?id=${gameId}`, {
  name: "star-2",
  description: "space",
  location: "orbit",
  settings: { lives: "5" },
  categories: [{ id: childId, name: "shooter" }],
}, 200);

const review = await call(api, "POST", "/reviews", { gameId, userId, description: "fun", rating: "4.50", isApproved: true }, 201);
const reviewId = review.id as number;
await call(api, "GET", `/reviews?gameId=${gameId}&userId=${userId}`, undefined, 200);
await call(api, "PUT", `/reviews?id=${reviewId}`, { gameId, userId, description: "great", rating: "5.00", isApproved: true }, 200);

const state = await call(api, "POST", "/game_states", { gameId, userId, state: "intro" }, 201);
const stateId = state.id as number;
await call(api, "GET", `/game_states?gameId=${gameId}&userId=${userId}`, undefined, 200);
await call(api, "PUT", `/game_states?id=${stateId}`, { gameId, userId, state: "boss" }, 200);

const contact = await call(api, "POST", "/contacts", {
  firstName: "Ada",
  lastName: "Lovelace",
  contactSourceName: "Manual",
  userId,
}, 201);
const contactId = contact.id as number;
await call(api, "GET", `/contacts?userId=${userId}`, undefined, 200);
await call(api, "PUT", `/contacts?id=${contactId}`, {
  firstName: "Ada Byron",
  lastName: "Lovelace",
  contactSourceName: "Manual",
  userId,
}, 200);

for (const target of [
  `/contacts?id=${contactId}`,
  `/reviews?id=${reviewId}`,
  `/game_states?id=${stateId}`,
  `/games?id=${gameId}`,
  `/categories?id=${childId}`,
  `/categories?id=${rootId}`,
  `/users?id=${userId}`,
]) {
  await call(api, "DELETE", target, undefined, 204);
  await call(api, "DELETE", target, undefined, 404);
  await call(api, "PUT", target, {}, 404);
}

async function call(api: Api, method: string, target: string, body: object | undefined, status: number): Promise<Record<string, never> & { id?: number; username?: string }> {
  const result = await api.handle(method, target, body == null ? undefined : JSON.stringify(body));
  if (result.status !== status) throw new Error(`${method} ${target} => ${result.status} ${result.body}`);
  return result.body.length === 0 ? {} : JSON.parse(result.body);
}
