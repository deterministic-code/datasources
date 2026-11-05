import {
  DatasourceError,
  Identifier,
  Operator,
  Where,
  type ICRUDRepository,
  type IWhere,
} from "@deterministic-code/datasources";
import { UsersRepository } from "../generated/repositories/usersRepository.ts";
import { CategoriesRepository } from "../generated/repositories/categoriesRepository.ts";
import { GamesRepository } from "../generated/repositories/gamesRepository.ts";
import { ReviewsRepository } from "../generated/repositories/reviewsRepository.ts";
import { GameStatesRepository } from "../generated/repositories/gameStatesRepository.ts";
import { ContactsRepository } from "../generated/repositories/contactsRepository.ts";
import type { Contact } from "../generated/types/view/contact.ts";
import type { User } from "../generated/types/view/user.ts";
import type { Game } from "../generated/types/view/game.ts";
import type { Category } from "../generated/types/view/category.ts";
import type { Review } from "../generated/types/datasource/review.ts";
import type { GameState } from "../generated/types/datasource/gameState.ts";
import { jsonObject } from "./jsonObject.ts";

export type HttpResult = { status: number; body: string };

export class Api {
  constructor(
    private readonly users: UsersRepository,
    private readonly categories: CategoriesRepository,
    private readonly games: GamesRepository,
    private readonly reviews: ReviewsRepository,
    private readonly states: GameStatesRepository,
    private readonly contacts: ContactsRepository,
  ) {}

  handle = async (method: string, target: string, body: string | undefined): Promise<HttpResult> => {
    try {
      const split = target.indexOf("?");
      const path = (split < 0 ? target : target.slice(0, split)).replace(/\/$/, "") || "/";
      const query = parse(split < 0 ? "" : target.slice(split + 1));
      return await ({
        "GET /users": () => get(this.users, query, ["id", "username"]),
        "POST /users": () => post(this.users, read<User>(body)),
        "PUT /users": () => put(this.users, query, body, (incoming, current) => ({ ...incoming, id: current.id })),
        "DELETE /users": () => remove(this.users, query),
        "GET /categories": () => this.getCategories(query),
        "POST /categories": () => post(this.categories, read<Category>(body)),
        "PUT /categories": () => put(this.categories, query, body, (incoming, current) => ({ ...incoming, id: current.id })),
        "DELETE /categories": () => remove(this.categories, query),
        "GET /games": () => get(this.games, query, ["id", "name"]),
        "POST /games": () => post(this.games, read<Game>(body)),
        "PUT /games": () => put(this.games, query, body, (incoming, current) => ({ ...incoming, id: current.id })),
        "DELETE /games": () => remove(this.games, query),
        "GET /reviews": () => get(this.reviews, query, ["id", "gameId", "userId"]),
        "POST /reviews": () => post(this.reviews, read<Review>(body)),
        "PUT /reviews": () => put(this.reviews, query, body, (incoming, current) => ({ ...incoming, id: current.id })),
        "DELETE /reviews": () => remove(this.reviews, query),
        "GET /game_states": () => get(this.states, query, ["id", "gameId", "userId"]),
        "POST /game_states": () => post(this.states, read<GameState>(body)),
        "PUT /game_states": () => put(this.states, query, body, (incoming, current) => ({ ...incoming, id: current.id })),
        "DELETE /game_states": () => remove(this.states, query),
        "GET /contacts": () => get(this.contacts, query, ["id", "userId"]),
        "POST /contacts": () => post(this.contacts, read<Contact>(body)),
        "PUT /contacts": () => put(this.contacts, query, body, (incoming, current) => ({ ...incoming, id: current.id })),
        "DELETE /contacts": () => remove(this.contacts, query),
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

  private getCategories = async (query: Record<string, string>): Promise<HttpResult> => {
    const raw = query.categoryId ?? query.category_id ?? "";
    if (raw.length > 0 && Number.isNaN(Number(raw))) throw new TypeError("categoryId is required");
    const filter = raw.length === 0
      ? new Where(new Operator.IsNull(new Identifier("categoryId")))
      : new Where(new Operator.Equals("categoryId", Number(raw)));
    return { status: 200, body: jsonObject.toString(await list(this.categories, filter)) };
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

const get = async <T>(repo: ICRUDRepository<T>, query: Record<string, string>, keys: string[]): Promise<HttpResult> =>
  ({ status: 200, body: jsonObject.toString(await list(repo, whereOf(query, keys))) });

const post = async <T>(repo: ICRUDRepository<T>, row: T): Promise<HttpResult> =>
  ({ status: 201, body: jsonObject.toString(await repo.create(row)) });

const put = async <T>(
  repo: ICRUDRepository<T>,
  query: Record<string, string>,
  body: string | undefined,
  withId: (incoming: T, current: T) => T,
): Promise<HttpResult> => {
  const id = requiredNumber(query, "id");
  const by = new Where(new Operator.Equals("id", id));
  const current = await repo.readSingle(by);
  if (current == null) return { status: 404, body: "" };
  await repo.update(withId(read<T>(body), current));
  return { status: 200, body: jsonObject.toString(await repo.readSingle(by)) };
};

const remove = async <T>(repo: ICRUDRepository<T>, query: Record<string, string>): Promise<HttpResult> => {
  const id = requiredNumber(query, "id");
  const row = await repo.readSingle(new Where(new Operator.Equals("id", id)));
  if (row == null) return { status: 404, body: "" };
  await repo.delete(row);
  return { status: 204, body: "" };
};

const list = async <T>(repo: { read: (query?: IWhere) => AsyncIterable<T> }, query: IWhere | undefined): Promise<T[]> => {
  const rows: T[] = [];
  for await (const row of query == null ? repo.read() : repo.read(query)) rows.push(row);
  return rows;
};

const whereOf = (query: Record<string, string>, keys: string[]): IWhere | undefined => {
  const parts = keys
    .filter((key) => (query[key] ?? "").length > 0)
    .map((key) => new Operator.Equals(key, numberKey(key) ? Number(query[key]) : query[key]));
  return parts.length === 0 ? undefined : new Where(parts.reduce((left, right) => new Operator.And(left, right)));
};

const numberKey = (key: string): boolean => key === "id" || key === "userId" || key === "gameId" || key === "categoryId";

const read = <T>(body: string | undefined): T => jsonObject.fromString<T>(body);

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
