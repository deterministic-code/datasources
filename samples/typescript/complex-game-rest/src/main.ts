import "../generated/repositories/repositoryRegistry.ts";
import "./datasources-register.ts";
import { Datasource } from "@deterministic-code/datasources";
import { AddressTypesRepository } from "../generated/repositories/addressTypesRepository.ts";
import { CategoriesRepository } from "../generated/repositories/categoriesRepository.ts";
import { ContactSourcesRepository } from "../generated/repositories/contactSourcesRepository.ts";
import { ContactsRepository } from "../generated/repositories/contactsRepository.ts";
import { GameStatesRepository } from "../generated/repositories/gameStatesRepository.ts";
import { GamesRepository } from "../generated/repositories/gamesRepository.ts";
import { GroupsRepository } from "../generated/repositories/groupsRepository.ts";
import { PhoneTypesRepository } from "../generated/repositories/phoneTypesRepository.ts";
import { ReviewsRepository } from "../generated/repositories/reviewsRepository.ts";
import { RolesRepository } from "../generated/repositories/rolesRepository.ts";
import { UsersRepository } from "../generated/repositories/usersRepository.ts";
import type { AddressType } from "../generated/types/datasource/addressType.ts";
import type { ContactSource } from "../generated/types/datasource/contactSource.ts";
import type { Group } from "../generated/types/datasource/group.ts";
import type { PhoneType } from "../generated/types/datasource/phoneType.ts";
import type { Role } from "../generated/types/datasource/role.ts";
import { Api } from "./api.ts";
import { toJson } from "./jsonObject.ts";

await using ds = Datasource.default;
const connection = await ds.createConnection();
await connection.open();
const api = new Api(
  new UsersRepository(connection),
  new CategoriesRepository(connection),
  new GamesRepository(connection),
  new ReviewsRepository(connection),
  new GameStatesRepository(connection),
  new ContactsRepository(connection),
);
const roles = new RolesRepository(connection);
const groups = new GroupsRepository(connection);
const sources = new ContactSourcesRepository(connection);
const role = await roles.create({ name: "player", description: "plays" } as Role);
const group = await groups.create({ name: "guild", description: "guild" } as Group);
await sources.create({ name: "Manual" } as ContactSource);
const addressTypes = new AddressTypesRepository(connection);
await addressTypes.create({ name: "Home" } as AddressType);
await addressTypes.create({ name: "Work" } as AddressType);
await new PhoneTypesRepository(connection).create({ name: "Mobile" } as PhoneType);

const user = await call(api, "POST", "/users", {
  username: "ada",
  password: "secret",
  settings: { theme: "dark" },
  roles: [{ name: role.name, description: role.description }],
  groups: [{ name: group.name, description: group.description }],
}, 201);
const userId = user.id;
const listed = await call(api, "GET", "/users?username=ada", undefined, 200);
if (!Array.isArray(listed) || listed.length !== 1) throw new Error("get users username");
await call(api, "PUT", `/users?id=${userId}`, {
  username: "ada-byron",
  password: "secret",
  settings: { theme: "sepia" },
  roles: [{ name: role.name }],
  groups: [{ name: group.name }],
}, 200);
const loaded = await call(api, "GET", `/users?id=${userId}`, undefined, 200);
if (!Array.isArray(loaded) || loaded[0].username !== "ada-byron") throw new Error("put users");

const root = await call(api, "POST", "/categories", { name: "arcade", description: "root" }, 201);
const rootId = root.id;
const child = await call(api, "POST", "/categories", { name: "shooter", description: "child", categoryId: rootId }, 201);
const childId = child.id;
const roots = await call(api, "GET", "/categories?category_id=", undefined, 200);
if (!Array.isArray(roots) || roots.length !== 1 || roots[0].id !== rootId) throw new Error("get category roots");
const children = await call(api, "GET", `/categories?categoryId=${rootId}`, undefined, 200);
if (!Array.isArray(children) || children.length !== 1 || children[0].id !== childId) throw new Error("get category children");

const game = await call(api, "POST", "/games", {
  name: "star",
  description: "space",
  location: "orbit",
  settings: { lives: "3" },
  categories: [{ id: childId, name: "shooter" }],
}, 201);
const gameId = game.id;
await call(api, "GET", `/games?id=${gameId}`, undefined, 200);
await call(api, "PUT", `/games?id=${gameId}`, {
  name: "star-2",
  description: "space",
  location: "orbit",
  settings: { lives: "5" },
  categories: [{ id: childId, name: "shooter" }],
}, 200);

const review = await call(api, "POST", "/reviews", { gameId, userId, description: "fun", rating: "4.50", isApproved: true }, 201);
const reviewId = review.id;
await call(api, "GET", `/reviews?gameId=${gameId}&userId=${userId}`, undefined, 200);
await call(api, "PUT", `/reviews?id=${reviewId}`, { gameId, userId, description: "great", rating: "5.00", isApproved: true }, 200);

const state = await call(api, "POST", "/game_states", { gameId, userId, state: "intro" }, 201);
const stateId = state.id;
await call(api, "GET", `/game_states?gameId=${gameId}&userId=${userId}`, undefined, 200);
await call(api, "PUT", `/game_states?id=${stateId}`, { gameId, userId, state: "boss" }, 200);

const contact = await call(api, "POST", "/contacts", {
  firstName: "Ada",
  lastName: "Lovelace",
  contactSourceName: "Manual",
  userId,
  addresses: [
    { addressTypeName: "Home", line1: "12 Analytical Engine", city: "London", country: "GB" },
    { addressTypeName: "Work", line1: "1 Strand", city: "London", country: "GB" },
  ],
  phones: [{ phoneTypeName: "Mobile", number: "555-0100" }],
}, 201);
const contactId = contact.id;
const created = await call(api, "GET", `/contacts?id=${contactId}`, undefined, 200);
requireAdaChildren(created, "Home", "London", "555-0100");
if (addressByLine1(contactRow(created), "1 Strand").addressTypeName !== "Work")
  throw new Error("create work address");
const engine = addressByLine1(contactRow(created), "12 Analytical Engine");
const phone = contactRow(created).phones[0];
await call(api, "PUT", `/contacts?id=${contactId}`, {
  firstName: "Ada Byron",
  lastName: "Lovelace",
  contactSourceName: "Manual",
  userId,
  addresses: [
    {
      id: engine.id,
      addressTypeName: "Work",
      line1: "12 Analytical Engine",
      city: "Cambridge",
      country: "FR",
    },
    { addressTypeName: "Home", line1: "1 College", city: "Cambridge", country: "GB" },
  ],
  phones: [{ id: phone.id, phoneTypeName: "Mobile", number: "555-0111" }],
}, 200);
const updated = await call(api, "GET", `/contacts?id=${contactId}`, undefined, 200);
requireAdaChildren(updated, "Work", "Cambridge", "555-0111");
if (addressByLine1(contactRow(updated), "1 College").addressTypeName !== "Home")
  throw new Error("update home address");

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

async function call(api: Api, method: string, target: string, body: object | undefined, status: number) {
  const result = await api.handle(method, target, body == null ? undefined : toJson(body));
  if (result.status !== status) throw new Error(`${method} ${target} => ${result.status} ${result.body}`);
  return result.body.length === 0 ? {} : JSON.parse(result.body);
}

function contactRow(listed: any) {
  if (!Array.isArray(listed) || listed.length !== 1) throw new Error("contact list");
  return listed[0];
}

function addressByLine1(contact: any, line1: string) {
  const address = contact.addresses.find((row) => row.line1 === line1);
  if (address == null) throw new Error(line1);
  return address;
}

function requireAdaChildren(listed: any, engineType: string, engineCity: string, number: string) {
  const contact = contactRow(listed);
  if (contact.addresses.length !== 2 || contact.phones.length !== 1) throw new Error("child counts");
  const engine = addressByLine1(contact, "12 Analytical Engine");
  if (engine.addressTypeName !== engineType || engine.city !== engineCity) throw new Error("engine address");
  if (engine.contactId !== contact.id) throw new Error("address contactId");
  const phone = contact.phones[0];
  if (phone.number !== number || phone.phoneTypeName !== "Mobile") throw new Error("phone");
  if (phone.contactId !== contact.id) throw new Error("phone contactId");
}
