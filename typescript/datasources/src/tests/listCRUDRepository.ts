import type { Constructable } from "../converters/Constructable.ts";
import type { IExecuteStoredProcedureStatement } from "../statements/IExecuteStoredProcedureStatement.ts";
import type { IQuery } from "../statements/IQuery.ts";
import type { ICRUDRepository } from "../ICRUDRepository.ts";
import { ReadonlyRepository, untyped, withoutSource } from "../ReadonlyRepository.ts";
import type { Sourced } from "../IReadonlyRepository.ts";
import type { Response } from "../Response.ts";
import type { IWhere } from "../statements/IWhere.ts";
import type { Cells } from "../converters/Cell.ts";
import type { Scalar } from "../builders/Scalar.ts";

type Props = Record<string, Scalar | object>;

const props = (item: object): Props => item as Props;

const matches = <T extends object>(item: T, query?: IQuery | IWhere | IExecuteStoredProcedureStatement): boolean =>
  parametersOf(query).every(
    ({ name, value }) => String(props(item)[name]) === String(value),
  );

const parametersOf = (
  query?: IQuery | IWhere | IExecuteStoredProcedureStatement,
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

export class ListCRUDRepository<T extends object>
  extends ReadonlyRepository<T>
  implements ICRUDRepository<T>
{
  readonly items: T[];
  private nextId: number;

  constructor(
    items: T[] = [],
    type: Constructable<T> = untyped<T>(),
  ) {
    super(type);
    this.items = [...items];
    this.nextId =
      items.reduce((max, item) => {
        const id = idOf(item);
        return typeof id === "number" && id > max ? id : max;
      }, 0) + 1;
  }

  async *read(query?: IQuery | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T> {
    for (const item of this.items) if (matches(item, query)) yield item;
  }

  async *readWithSource(
    query?: IQuery | IWhere | IExecuteStoredProcedureStatement,
  ): AsyncIterable<Sourced<T>> {
    yield* withoutSource(this.read(query));
  }

  create = async (item: T): Promise<Response> => {
    if (!hasId(idOf(item))) setId(item, this.nextId++);
    this.items.push(item);
    return { result: idOf(item), rowsAffected: 1 };
  };

  update = async (item: T, _byWhere?: IWhere): Promise<Response> => {
    const key = idKey(idOf(item));
    const index = this.items.findIndex((row) => idKey(idOf(row)) === key);
    if (index >= 0) this.items[index] = item;
    return { result: null, rowsAffected: 1 };
  };

  delete = async (item: T, _byWhere?: IWhere): Promise<Response> => {
    const key = idKey(idOf(item));
    const index = this.items.findIndex((row) => idKey(idOf(row)) === key);
    if (index >= 0) this.items.splice(index, 1);
    return { result: null, rowsAffected: 1 };
  };

  readSingleById = async (id: Scalar | Cells): Promise<T | undefined> =>
    this.items.find((item) =>
      id !== null && typeof id === "object"
        ? Object.entries(id).every(([name, value]) => String(props(item)[name]) === String(value))
        : String(idOf(item)) === String(id),
    );
}
