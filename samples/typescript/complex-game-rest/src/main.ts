import "../generated/repositories/repositoryRegistry.ts";
import "./datasources-register.ts";
import { Datasource, Operator, Where } from "@deterministic-code/datasources";
import { CategoriesRepository } from "../generated/repositories/categoriesRepository.ts";
import { ContactSourcesRepository } from "../generated/repositories/contactSourcesRepository.ts";
import { ContactsRepository } from "../generated/repositories/contactsRepository.ts";
import { GameStatesRepository } from "../generated/repositories/gameStatesRepository.ts";
import { GamesRepository } from "../generated/repositories/gamesRepository.ts";
import { GroupsRepository } from "../generated/repositories/groupsRepository.ts";
import { ItemGraphsRepository } from "../generated/repositories/itemGraphsRepository.ts";
import { ItemLinksRepository } from "../generated/repositories/itemLinksRepository.ts";
import { ItemsRepository } from "../generated/repositories/itemsRepository.ts";
import { ReviewsRepository } from "../generated/repositories/reviewsRepository.ts";
import { RolesRepository } from "../generated/repositories/rolesRepository.ts";
import { SkillsRepository } from "../generated/repositories/skillsRepository.ts";
import { SkillTreesRepository } from "../generated/repositories/skillTreesRepository.ts";
import { UsersRepository } from "../generated/repositories/usersRepository.ts";
import type { Group } from "../generated/types/datasource/group.ts";
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
  new SkillsRepository(connection),
  new SkillTreesRepository(connection),
  new ItemsRepository(connection),
  new ItemLinksRepository(connection),
  new ItemGraphsRepository(connection),
);
const roles = new RolesRepository(connection);
const groups = new GroupsRepository(connection);
const sources = new ContactSourcesRepository(connection);
const role = await roles.create({ name: "player", description: "plays" } as Role);
const group = await groups.create({ name: "guild", description: "guild" } as Group);
const source = await sources.readSingle(new Where(new Operator.Equals("name", "Manual")));
if (source == null) throw new Error("source");

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
  contactSourceName: source.name,
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
  contactSourceName: source.name,
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

const combat = await call(api, "POST", "/skills", { name: "combat", parentSkillId: null }, 201);
const combatId = combat.id;
const melee = await call(api, "POST", "/skills", { name: "melee", parentSkillId: combatId }, 201);
const meleeId = melee.id;
const swordSkill = await call(api, "POST", "/skills", { name: "sword", parentSkillId: meleeId }, 201);
const nest = await call(api, "GET", `/skill_tree?id=${combatId}`, undefined, 200);
if (!Array.isArray(nest) || nest.length !== 1 || nest[0].children[0].name !== "melee" || nest[0].children[0].children[0].name !== "sword")
  throw new Error("skill nest");
if (nest[0].children[0].children[0].children.length !== 0) throw new Error("skill nest leaf");

const loopA = await call(api, "POST", "/skills", { name: "loop-a", parentSkillId: null }, 201);
const loopB = await call(api, "POST", "/skills", { name: "loop-b", parentSkillId: loopA.id }, 201);
const loopC = await call(api, "POST", "/skills", { name: "loop-c", parentSkillId: loopB.id }, 201);
await call(api, "PUT", `/skills?id=${loopA.id}`, { name: "loop-a", parentSkillId: loopC.id }, 200);
const cycle = await call(api, "GET", `/skill_tree?id=${loopA.id}`, undefined, 200);
if (!Array.isArray(cycle) || cycle[0].children[0].name !== "loop-b" || cycle[0].children[0].children[0].name !== "loop-c")
  throw new Error("skill cycle");
if (cycle[0].children[0].children[0].children.length !== 0) throw new Error("skill cycle cut");

const sword = await call(api, "POST", "/items", { name: "sword" }, 201);
const shield = await call(api, "POST", "/items", { name: "shield" }, 201);
const potion = await call(api, "POST", "/items", { name: "potion" }, 201);
await call(api, "POST", "/item_links", { fromItemId: sword.id, toItemId: shield.id }, 201);
await call(api, "POST", "/item_links", { fromItemId: sword.id, toItemId: potion.id }, 201);
const graph = await call(api, "GET", `/item_graph?id=${sword.id}`, undefined, 200);
const edgeNames = [...graph[0].edges.map((edge: { name: string }) => edge.name)].sort().join(",");
if (!Array.isArray(graph) || edgeNames !== "potion,shield") throw new Error("item graph");

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
