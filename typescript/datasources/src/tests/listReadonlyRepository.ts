import type { Constructable } from "../converters/Constructable.ts";
import type { Parameter } from "../Parameter.ts";
import type { IStatement } from "../statements/IStatement.ts";
import type { ISelectStatement } from "../statements/ISelectStatement.ts";
import type { Scalar } from "../builders/Scalar.ts";
import { ReadonlyRepository, untyped, type Sourced } from "../ReadonlyRepository.ts";

type Props = Record<string, Scalar | object>;

export class ListReadonlyRepository<T extends object> extends ReadonlyRepository<T> {
  constructor(
    private readonly items: T[] = [],
    type: Constructable<T> = untyped<T>(),
  ) {
    super(type);
  }

  protected async *readItems(query?: IStatement, parameters?: Parameter[]): AsyncIterable<Sourced<T>> {
    for (const item of this.items)
      if (matches(item, query, parameters)) yield { item };
  }
}

const fieldOf = (name: string): string => {
  const split = name.lastIndexOf("_");
  return split > 0 && /^\d+$/.test(name.slice(split + 1))
    ? name.slice(0, split)
    : name;
};

const matches = <T extends object>(item: T, query?: IStatement, parameters?: Parameter[]): boolean => {
  if (query && "getText" in query && query.getText() === "1 = 0") return false;
  const groups = new Map<string, { name: string; value: Scalar }[]>();
  for (const parameter of parameters ?? []) {
    const field = fieldOf(parameter.name);
    const list = groups.get(field) ?? [];
    list.push(parameter);
    groups.set(field, list);
  }
  return [...groups.entries()].every(([field, values]) =>
    values.some(({ value }) => String(props(item)[field]) === String(value)),
  );
};

const props = (item: object): Props => item as Props;
