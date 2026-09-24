import type { Constructable } from "../converters/Constructable.ts";
import type { IExecuteStoredProcedureStatement } from "../statements/IExecuteStoredProcedureStatement.ts";
import type { IQuery } from "../statements/IQuery.ts";
import type { IWhere } from "../statements/IWhere.ts";
import type { Scalar } from "../builders/Scalar.ts";
import { ReadonlyRepository, tracked, untyped, withoutSource, type Sourced } from "../ReadonlyRepository.ts";

type Props = Record<string, Scalar | object>;

export class ListReadonlyRepository<T extends object> extends ReadonlyRepository<T> {
  constructor(
    private readonly items: T[],
    type: Constructable<T> = untyped<T>(),
  ) {
    super(type);
  }

  async *read(query?: IQuery | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T> {
    for (const item of this.items) if (matches(item, query)) yield item;
  }

  async *[tracked](
    query?: IQuery | IWhere | IExecuteStoredProcedureStatement,
  ): AsyncIterable<Sourced<T>> {
    yield* withoutSource(this.read(query));
  }
}

const fieldOf = (name: string): string => {
  const split = name.lastIndexOf("_");
  return split > 0 && /^\d+$/.test(name.slice(split + 1))
    ? name.slice(0, split)
    : name;
};

const matches = <T extends object>(item: T, query?: IQuery | IWhere | IExecuteStoredProcedureStatement): boolean => {
  if (query && "getText" in query && query.getText() === "1 = 0") return false;
  const groups = new Map<string, { name: string; value: Scalar }[]>();
  for (const parameter of parametersOf(query)) {
    const field = fieldOf(parameter.name);
    const list = groups.get(field) ?? [];
    list.push(parameter);
    groups.set(field, list);
  }
  return [...groups.entries()].every(([field, values]) =>
    values.some(({ value }) => String(props(item)[field]) === String(value)),
  );
};

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

const props = (item: object): Props => item as Props;
