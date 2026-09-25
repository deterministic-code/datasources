import type { Constructable } from "../converters/Constructable.ts";
import type { Parameter } from "../Parameter.ts";
import type { IStatement } from "../statements/IStatement.ts";
import { BulkCRUDRepository } from "../BulkCRUDRepository.ts";
import type { Response } from "../Response.ts";
import type { IWhere } from "../statements/IWhere.ts";
import type { Scalar } from "../builders/Scalar.ts";
import { defaultId, fieldNamesOf } from "../createRepositoryOptions.ts";
import type { IWriteEnrichmentStrategy } from "../IWriteEnrichmentStrategy.ts";
import type { IRepositoryTree } from "../IRepositoryTree.ts";
import { untyped, type Sourced } from "../ReadonlyRepository.ts";

type Props = Record<string, Scalar | object>;

const props = (item: object): Props => item as Props;

const matches = <T extends object>(item: T, parameters?: Parameter[]): boolean =>
  (parameters ?? []).every(({ name, value }) => String(props(item)[name]) === String(value));

const idOf = (item: object): Scalar | object | undefined => {
  const value = props(item).id ?? props(item).Id;
  return value === undefined ? undefined : value;
};

const hasId = (id: Scalar | object | undefined): boolean =>
  id !== undefined && id !== null && id !== 0 && id !== 0n && id !== "";

const idKey = (id: Scalar | object | undefined): string => String(id ?? "");

const setId = (item: object, id: number): void => {
  const name = "id" in item ? "id" : "Id" in item ? "Id" : "id";
  Object.assign(item, { [name]: id });
};

export class ListCRUDRepository<T extends object> extends BulkCRUDRepository<T> {
  readonly items: T[];
  private nextId: number;

  constructor(items?: T[], type?: Constructable<T>);
  constructor(write: IWriteEnrichmentStrategy<T> | undefined, items?: T[], type?: Constructable<T>);
  constructor(
    itemsOrWrite: T[] | IWriteEnrichmentStrategy<T> | undefined = [],
    typeOrItems: Constructable<T> | T[] = untyped<T>(),
    typeArg: Constructable<T> = untyped<T>(),
  ) {
    const write =
      itemsOrWrite !== undefined &&
      !Array.isArray(itemsOrWrite) &&
      typeof itemsOrWrite === "object" &&
      "persist" in itemsOrWrite
        ? itemsOrWrite
        : undefined;
    const items = Array.isArray(itemsOrWrite)
      ? itemsOrWrite
      : Array.isArray(typeOrItems)
        ? typeOrItems
        : [];
    const constructable =
      typeof typeOrItems === "function" ? typeOrItems : typeArg;
    super(write);
    (this as { type: Constructable<T> }).type = constructable;
    (this as { ids: readonly string[] }).ids =
      constructable === Object ? ["id"] : defaultId(fieldNamesOf(constructable));
    this.items = [...items];
    this.nextId =
      items.reduce((max, item) => {
        const id = idOf(item);
        return typeof id === "number" && id > max ? id : max;
      }, 0) + 1;
  }

  enrichmentTree = (): IRepositoryTree => this.tree;

  protected async *readItems(_query?: IStatement, parameters?: Parameter[]): AsyncIterable<Sourced<T>> {
    for (const item of this.items)
      if (matches(item, parameters)) yield { item };
  }

  protected inserted = async (item: T): Promise<T> => {
    if (!hasId(idOf(item))) setId(item, this.nextId++);
    this.items.push(item);
    return item;
  };

  protected save = async (item: T, _byWhere?: IWhere): Promise<Response> => {
    const key = idKey(idOf(item));
    const index = this.items.findIndex((row) => idKey(idOf(row)) === key);
    if (index >= 0) this.items[index] = item;
    return { result: null, rowsAffected: 1 };
  };

  protected remove = async (item: T, byWhere?: IWhere): Promise<Response> => {
    if (byWhere) {
      for (let index = this.items.length - 1; index >= 0; index--)
        if (matches(this.items[index]!, byWhere.parameters)) this.items.splice(index, 1);
      return { result: null, rowsAffected: 1 };
    }
    const key = idKey(idOf(item));
    const index = this.items.findIndex((row) => idKey(idOf(row)) === key);
    if (index >= 0) this.items.splice(index, 1);
    return { result: null, rowsAffected: 1 };
  };
}
