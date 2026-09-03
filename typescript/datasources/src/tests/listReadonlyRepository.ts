import type { IQuery } from "../IQuery.ts";
import type { IReadonlyRepository } from "../IReadonlyRepository.ts";
import type { IWhere } from "../IWhere.ts";
import type { Scalar } from "../builders/Scalar.ts";

type Props = Record<string, Scalar | object>;

export class ListReadonlyRepository<T extends object>
  implements IReadonlyRepository<T>
{
  constructor(private readonly items: T[]) {}

  getSingle = async (query?: IQuery | IWhere): Promise<T | undefined> => {
    for await (const item of this.get(query)) return item;
    return undefined;
  };

  async *get(query?: IQuery | IWhere): AsyncIterable<T> {
    for (const item of this.items) if (matches(item, query)) yield item;
  }
}

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

const props = (item: object): Props => item as Props;
