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
  settings: { theme: "dark", locale: "en-GB" },
  roles: [{ name: role.name, description: role.description }],
  groups: [{ name: group.name, description: group.description }],
}, 201);
const userId = user.id as number;
const listed = await call(api, "GET", "/users?username=ada", undefined, 200);
if (!Array.isArray(listed) || listed.length !== 1) throw new Error("get users username");
requireUser(listed[0], "ada", { theme: "dark", locale: "en-GB" }, role.name, group.name);
await call(api, "PUT", `/users?id=${userId}`, {
  username: "ada-byron",
  password: "secret",
  settings: { theme: "sepia", locale: "en-US" },
  roles: [{ name: role.name }],
  groups: [{ name: group.name }],
}, 200);
const loaded = await call(api, "GET", `/users?id=${userId}`, undefined, 200);
if (!Array.isArray(loaded) || loaded[0]?.username !== "ada-byron") throw new Error("put users");
requireUser(loaded[0], "ada-byron", { theme: "sepia", locale: "en-US" }, role.name, group.name);
await call(api, "PUT", `/users?id=${userId}`, {
  username: "ada-byron",
  password: "secret",
  settings: {},
  roles: [{ name: role.name }],
  groups: [{ name: group.name }],
}, 200);
requireUser((await call(api, "GET", `/users?id=${userId}`, undefined, 200))[0], "ada-byron", {}, role.name, group.name);

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
  settings: { lives: "3", mode: "arcade" },
  categories: [{ id: childId, name: "shooter" }],
}, 201);
const gameId = game.id as number;
const createdGame = await call(api, "GET", `/games?id=${gameId}`, undefined, 200);
requireGame(createdGame, "star", { lives: "3", mode: "arcade" }, childId);
await call(api, "PUT", `/games?id=${gameId}`, {
  name: "star-2",
  description: "space",
  location: "orbit",
  settings: { lives: "5", mode: "hard" },
  categories: [{ id: childId, name: "shooter" }],
}, 200);
const updatedGame = await call(api, "GET", `/games?id=${gameId}`, undefined, 200);
requireGame(updatedGame, "star-2", { lives: "5", mode: "hard" }, childId);
await call(api, "PUT", `/games?id=${gameId}`, {
  name: "star-2",
  description: "space",
  location: "orbit",
  settings: {},
  categories: [{ id: childId, name: "shooter" }],
}, 200);
requireGame(await call(api, "GET", `/games?id=${gameId}`, undefined, 200), "star-2", {}, childId);

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
  settings: { theme: "dark", volume: "loud" },
  prefs: { home: { locale: "en-GB", timezone: "Europe/London" } },
  addresses: [
    { addressTypeName: "Home", line1: "12 Analytical Engine", city: "London", country: "GB" },
    { addressTypeName: "Work", line1: "1 Strand", city: "London", country: "GB" },
  ],
  phones: [{ phoneTypeName: "Mobile", number: "555-0100" }],
}, 201);
const contactId = contact.id as number;
const created = await call(api, "GET", `/contacts?id=${contactId}`, undefined, 200);
requireAdaChildren(created, "Home", "London", "555-0100");
requireContactMaps(created, { theme: "dark", volume: "loud" }, "en-GB", "Europe/London");
if (addressByLine1(contactRow(created), "1 Strand").addressTypeName !== "Work")
  throw new Error("create work address");
const engine = addressByLine1(contactRow(created), "12 Analytical Engine");
const phone = contactRow(created).phones[0];
await call(api, "PUT", `/contacts?id=${contactId}`, {
  firstName: "Ada Byron",
  lastName: "Lovelace",
  contactSourceName: "Manual",
  userId,
  settings: { theme: "sepia", volume: "quiet" },
  prefs: { home: { locale: "en-US", timezone: "America/New_York" } },
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
requireContactMaps(updated, { theme: "sepia", volume: "quiet" }, "en-US", "America/New_York");
if (addressByLine1(contactRow(updated), "1 College").addressTypeName !== "Home")
  throw new Error("update home address");
const college = addressByLine1(contactRow(updated), "1 College");
await call(api, "PUT", `/contacts?id=${contactId}`, {
  firstName: "Ada Byron",
  lastName: "Lovelace",
  contactSourceName: "Manual",
  userId,
  settings: {},
  prefs: { home: { locale: "en-US", timezone: "America/New_York" } },
  addresses: [
    {
      id: engine.id,
      addressTypeName: "Work",
      line1: "12 Analytical Engine",
      city: "Cambridge",
      country: "FR",
    },
    {
      id: college.id,
      addressTypeName: "Home",
      line1: "1 College",
      city: "Cambridge",
      country: "GB",
    },
  ],
  phones: [{ id: phone.id, phoneTypeName: "Mobile", number: "555-0111" }],
}, 200);
requireContactMaps(await call(api, "GET", `/contacts?id=${contactId}`, undefined, 200), {}, "en-US", "America/New_York");

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

type ContactJson = {
  id: number;
  settings: Record<string, string>;
  prefs: Record<string, { locale: string; timezone: string }>;
  addresses: Array<{ id?: number; line1: string; addressTypeName: string; city: string; contactId: number }>;
  phones: Array<{ id?: number; number: string; phoneTypeName: string; contactId: number }>;
};

type UserJson = {
  username: string;
  settings: Record<string, string>;
  roles: Array<{ name: string }>;
  groups: Array<{ name: string }>;
};

type GameJson = {
  name: string;
  settings: Record<string, string>;
  categories: Array<{ id: number; name: string }>;
};

function rowOf<T>(listed: unknown, label: string): T {
  if (!Array.isArray(listed) || listed.length !== 1) throw new Error(label);
  return listed[0] as T;
}

function requireSettings(got: Record<string, string> | undefined, expected: Record<string, string>, label: string) {
  const settings = got ?? {};
  const keys = Object.keys(expected);
  if (Object.keys(settings).length !== keys.length) throw new Error(label);
  for (const key of keys) {
    if (settings[key] !== expected[key]) throw new Error(label);
  }
}

function requireUser(row: unknown, username: string, settings: Record<string, string>, roleName: string, groupName: string) {
  const user = row as UserJson;
  if (user.username !== username) throw new Error("user username");
  requireSettings(user.settings, settings, "user settings");
  if (!user.roles?.some((item) => item.name === roleName)) throw new Error("user roles");
  if (!user.groups?.some((item) => item.name === groupName)) throw new Error("user groups");
}

function requireGame(listed: unknown, name: string, settings: Record<string, string>, categoryId: number) {
  const game = rowOf<GameJson>(listed, "game list");
  if (game.name !== name) throw new Error("game name");
  requireSettings(game.settings, settings, "game settings");
  if (!game.categories?.some((item) => item.id === categoryId && item.name === "shooter")) throw new Error("game categories");
}

function requireContactMaps(listed: unknown, settings: Record<string, string>, locale: string, timezone: string) {
  const contact = contactRow(listed);
  requireSettings(contact.settings, settings, "contact settings");
  if (contact.prefs?.home?.locale !== locale || contact.prefs.home.timezone !== timezone) throw new Error("contact prefs");
}

function contactRow(listed: unknown): ContactJson {
  if (!Array.isArray(listed) || listed.length !== 1) throw new Error("contact list");
  return listed[0] as ContactJson;
}

function addressByLine1(contact: ContactJson, line1: string) {
  const address = contact.addresses.find((row) => row.line1 === line1);
  if (address == null) throw new Error(line1);
  return address;
}

function requireAdaChildren(listed: unknown, engineType: string, engineCity: string, number: string) {
  const contact = contactRow(listed);
  if (contact.addresses.length !== 2 || contact.phones.length !== 1) throw new Error("child counts");
  const engine = addressByLine1(contact, "12 Analytical Engine");
  if (engine.addressTypeName !== engineType || engine.city !== engineCity) throw new Error("engine address");
  const addressFk: typeof contact.id = engine.contactId;
  if (typeof engine.contactId !== "number" || addressFk !== contact.id) throw new Error("address contactId");
  const phone = contact.phones[0];
  if (phone.number !== number || phone.phoneTypeName !== "Mobile") throw new Error("phone");
  const phoneFk: typeof contact.id = phone.contactId;
  if (typeof phone.contactId !== "number" || phoneFk !== contact.id) throw new Error("phone contactId");
}
