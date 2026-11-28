import {
  DatasourceError,
  EnrichmentSelection,
  Identifier,
  Operator,
  Where,
  type ICRUDRepository,
  type IWhere,
} from "@deterministic-code/datasources";
import { CategoriesRepository } from "../generated/repositories/categoriesRepository.ts";
import { ContactsRepository } from "../generated/repositories/contactsRepository.ts";
import { GameStatesRepository } from "../generated/repositories/gameStatesRepository.ts";
import { GamesRepository } from "../generated/repositories/gamesRepository.ts";
import { ItemGraphsRepository } from "../generated/repositories/itemGraphsRepository.ts";
import { ItemLinksRepository } from "../generated/repositories/itemLinksRepository.ts";
import { ItemsRepository } from "../generated/repositories/itemsRepository.ts";
import { ReviewsRepository } from "../generated/repositories/reviewsRepository.ts";
import { SkillsRepository } from "../generated/repositories/skillsRepository.ts";
import { SkillTreesRepository } from "../generated/repositories/skillTreesRepository.ts";
import { UsersRepository } from "../generated/repositories/usersRepository.ts";
import type { Contact } from "../generated/types/view/contact.ts";
import type { User } from "../generated/types/view/user.ts";
import type { Game } from "../generated/types/view/game.ts";
import type { Category } from "../generated/types/view/category.ts";
import type { Review } from "../generated/types/datasource/review.ts";
import type { GameState } from "../generated/types/datasource/gameState.ts";
import { toJson } from "./jsonObject.ts";

export type HttpResult = { status: number; body: string };

export class Api {
  constructor(
    private readonly users: UsersRepository,
    private readonly categories: CategoriesRepository,
    private readonly games: GamesRepository,
    private readonly reviews: ReviewsRepository,
    private readonly states: GameStatesRepository,
    private readonly contacts: ContactsRepository,
    private readonly skills: SkillsRepository,
    private readonly skillTrees: SkillTreesRepository,
    private readonly items: ItemsRepository,
    private readonly itemLinks: ItemLinksRepository,
    private readonly itemGraphs: ItemGraphsRepository,
  ) {}

  handle = async (method: string, target: string, body: string | undefined): Promise<HttpResult> => {
    try {
      const split = target.indexOf("?");
      const path = (split < 0 ? target : target.slice(0, split)).replace(/\/$/, "") || "/";
      const query = parse(split < 0 ? "" : target.slice(split + 1));
      return await ({
        "GET /users": () => get(this.users, query, ["id", "username"]),
        "POST /users": () => post(this.users, read(body)),
        "PUT /users": () => put(this.users, query, read(body), (row, id) => { row.id = id; }),
        "DELETE /users": () => remove(this.users, query),
        "GET /categories": () => this.getCategories(query),
        "POST /categories": () => post(this.categories, read(body)),
        "PUT /categories": () => put(this.categories, query, read(body), (row, id) => { row.id = id; }),
        "DELETE /categories": () => remove(this.categories, query),
        "GET /games": () => this.getGames(query),
        "POST /games": () => post(this.games, read(body)),
        "PUT /games": () => put(this.games, query, read(body), (row, id) => { row.id = id; }),
        "DELETE /games": () => remove(this.games, query),
        "GET /reviews": () => get(this.reviews, query, ["id", "gameId", "userId"]),
        "POST /reviews": () => post(this.reviews, read(body)),
        "PUT /reviews": () => put(this.reviews, query, read(body), (row, id) => { row.id = id; }),
        "DELETE /reviews": () => remove(this.reviews, query),
        "GET /game_states": () => get(this.states, query, ["id", "gameId", "userId"]),
        "POST /game_states": () => post(this.states, read(body)),
        "PUT /game_states": () => put(this.states, query, read(body), (row, id) => { row.id = id; }),
        "DELETE /game_states": () => remove(this.states, query),
        "GET /contacts": () => get(this.contacts, query, ["id", "userId"]),
        "POST /contacts": () => post(this.contacts, read(body)),
        "PUT /contacts": () => put(this.contacts, query, read(body), (row, id) => { row.id = id; }),
        "DELETE /contacts": () => remove(this.contacts, query),
        "GET /skills": () => get(this.skills, query, ["id", "name"]),
        "POST /skills": () => post(this.skills, read(body)),
        "PUT /skills": () => put(this.skills, query, read(body), (row, id) => { row.id = id; }),
        "DELETE /skills": () => remove(this.skills, query),
        "GET /skill_tree": () => get(this.skillTrees, query, ["id", "name"]),
        "GET /items": () => get(this.items, query, ["id", "name"]),
        "POST /items": () => post(this.items, read(body)),
        "PUT /items": () => put(this.items, query, read(body), (row, id) => { row.id = id; }),
        "DELETE /items": () => remove(this.items, query),
        "POST /item_links": () => post(this.itemLinks, read(body)),
        "GET /item_graph": () => get(this.itemGraphs, query, ["id", "name"]),
      }[`${method.toUpperCase()} ${path}`] ?? (() => Promise.resolve({ status: 404, body: "" })))();
    } catch (error) {
      return error instanceof RequestError
        ? { status: error.status, body: error.message }
        : error instanceof SyntaxError
            || error instanceof DatasourceError
            || error instanceof TypeError
          ? { status: 400, body: error.message }
          : Promise.reject(error);
    }
  };

  private getGames = async (query: Record<string, string>): Promise<HttpResult> => {
    const options = { enrich: EnrichmentSelection.only("settings") };
    if ((query.id ?? "").length > 0 && (query.name ?? "").length === 0) {
      const game = await this.games.readSingle(new Where(new Operator.Equals("id", Number(query.id))), undefined, options);
      return { status: 200, body: toJson(game ? [game] : []) };
    }
    return { status: 200, body: toJson(await list(this.games, whereOf(query, ["id", "name"]), options)) };
  };

  private getCategories = async (query: Record<string, string>): Promise<HttpResult> => {
    const raw = query.parentId ?? query.parent_id ?? query.categoryId ?? query.category_id ?? "";
    if (raw.length > 0 && Number.isNaN(Number(raw))) throw new TypeError("parentId is required");
    const filter = raw.length === 0
      ? new Where(new Operator.IsNull(new Identifier("categoryId")))
      : new Where(new Operator.Equals("categoryId", Number(raw)));
    return { status: 200, body: toJson(await list(this.categories, filter)) };
  };
}

class RequestError extends Error {
  constructor(
    message: string,
    readonly status = 400,
  ) {
    super(message);
  }
}

const get = async <T>(
  repo: { read: (query?: IWhere) => AsyncIterable<T> },
  query: Record<string, string>,
  keys: string[],
): Promise<HttpResult> =>
  ({ status: 200, body: toJson(await list(repo, whereOf(query, keys))) });

const post = async <T>(repo: ICRUDRepository<T>, row: T): Promise<HttpResult> =>
  ({ status: 201, body: toJson(await repo.create(row)) });

const put = async <T>(
  repo: ICRUDRepository<T>,
  query: Record<string, string>,
  row: T,
  setId: (row: T, id: number) => void,
): Promise<HttpResult> => {
  const id = requiredNumber(query, "id");
  if ((await repo.readSingle(new Where(new Operator.Equals("id", id)))) == null) return { status: 404, body: "" };
  setId(row, id);
  await repo.update(row);
  return { status: 200, body: toJson(await repo.readSingle(new Where(new Operator.Equals("id", id)))) };
};

const remove = async <T>(repo: ICRUDRepository<T>, query: Record<string, string>): Promise<HttpResult> => {
  const id = requiredNumber(query, "id");
  const row = await repo.readSingle(new Where(new Operator.Equals("id", id)));
  if (row == null) return { status: 404, body: "" };
  await repo.delete(row);
  return { status: 204, body: "" };
};

const list = async <T>(
  repo: { read: (query?: IWhere, parameters?: undefined, options?: { enrich?: EnrichmentSelection }) => AsyncIterable<T> },
  query: IWhere | undefined,
  options?: { enrich?: EnrichmentSelection },
): Promise<T[]> => {
  const rows: T[] = [];
  for await (const row of query == null ? repo.read(undefined, undefined, options) : repo.read(query, undefined, options))
    rows.push(row);
  return rows;
};

const whereOf = (query: Record<string, string>, keys: string[]): IWhere | undefined => {
  const parts = keys
    .filter((key) => (query[key] ?? "").length > 0)
    .map((key) => new Operator.Equals(key, numberKey(key) ? Number(query[key]) : query[key]));
  return parts.length === 0 ? undefined : new Where(parts.reduce((left, right) => new Operator.And(left, right)));
};

const numberKey = (key: string): boolean => key === "id" || key === "userId" || key === "gameId" || key === "categoryId" || key === "parentId";

const read = (body: string | undefined) => JSON.parse(body || "{}");

const requiredNumber = (query: Record<string, string>, key: string): number => {
  const value = Number(required(query, key));
  if (Number.isNaN(value)) throw new TypeError(`${key} is required`);
  return value;
};

const required = (query: Record<string, string>, key: string): string =>
  (query[key] ?? "").length > 0 ? query[key]! : (() => { throw new RequestError(`${key} is required`); })();

const parse = (query: string): Record<string, string> =>
  Object.fromEntries(
    query
      .split("&")
      .filter((part) => part.length > 0)
      .map((part) => {
        const eq = part.indexOf("=");
        return [
          decodeURIComponent(eq < 0 ? part : part.slice(0, eq)),
          decodeURIComponent(eq < 0 ? "" : part.slice(eq + 1)),
        ];
      }),
  );

export type { User, Game, Category, Review, GameState, Contact };
