import type { IQuery } from "../IQuery.ts";
import type { IRepository } from "../IRepository.ts";
import type { IResponse } from "../IResponse.ts";
import type { IWhere } from "../IWhere.ts";
import type { Scalar } from "../builders/Scalar.ts";

type Props = Record<string, Scalar | object>;

const props = (item: object): Props => item as Props;

const matches = <T extends object>(item: T, query?: IQuery | IWhere): boolean =>
  parametersOf(query).every(
    ({ name, value }) => String(props(item)[name]) === String(value),
  );

const parametersOf = (
  query?: IQuery | IWhere,
): { name: string; value: Scalar }[] => {
  if (!query) return [];
  const where = "where" in query ? query.where : query;
  return (where?.parameters ?? query.parameters ?? []).map((parameter) => ({
    name: parameter.name,
    value: parameter.value as Scalar,
  }));
};

const idOf = (item: object): Scalar | undefined => {
  const value = props(item).id ?? props(item).Id;
  return value === undefined || typeof value === "object" ? undefined : value;
};

const hasId = (id: Scalar | undefined): boolean =>
  id !== undefined && id !== null && id !== 0 && id !== 0n && id !== "";

const idKey = (id: Scalar | undefined): string => String(id ?? "");

const setId = (item: object, id: number): void => {
  const name = "id" in item ? "id" : "Id" in item ? "Id" : "id";
  Object.assign(item, { [name]: id });
};

export class ListCRUDRepository<T extends object> implements IRepository<T> {
  readonly items: T[];
  private nextId: number;

  constructor(items: T[] = []) {
    this.items = [...items];
    this.nextId =
      items.reduce((max, item) => {
        const id = idOf(item);
        return typeof id === "number" && id > max ? id : max;
      }, 0) + 1;
  }

  getSingle = async (query?: IQuery | IWhere): Promise<T | undefined> => {
    for await (const item of this.get(query)) return item;
    return undefined;
  };

  async *get(query?: IQuery | IWhere): AsyncIterable<T> {
    for (const item of this.items) if (matches(item, query)) yield item;
  }

  add = async (item: T): Promise<IResponse> => {
    if (!hasId(idOf(item))) setId(item, this.nextId++);
    this.items.push(item);
    return { result: idOf(item), rowsAffected: 1 };
  };

  update = async (item: T, _byWhere?: IWhere): Promise<IResponse> => {
    const key = idKey(idOf(item));
    const index = this.items.findIndex((row) => idKey(idOf(row)) === key);
    if (index >= 0) this.items[index] = item;
    return { result: null, rowsAffected: 1 };
  };

  delete = async (item: T, _byWhere?: IWhere): Promise<IResponse> => {
    const key = idKey(idOf(item));
    const index = this.items.findIndex((row) => idKey(idOf(row)) === key);
    if (index >= 0) this.items.splice(index, 1);
    return { result: null, rowsAffected: 1 };
  };
}
