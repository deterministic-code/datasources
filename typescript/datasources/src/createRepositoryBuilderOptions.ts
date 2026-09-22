import type { IConnection } from "./IConnection.ts";
import type { Cell } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { RepositoryBuilderOptions } from "./RepositoryBuilderOptions.ts";

type FieldSource = Record<string, string | number | boolean | bigint | null | object>;

export const fieldNamesOf = <T>(type: Constructable<T>): string[] => {
  const instance = new type() as FieldSource;
  return Object.keys(instance).filter((key) => !Array.isArray(instance[key]));
};

export const defaultId = (fields: string[]): string[] =>
  fields.includes("id") ? ["id"] : fields.includes("Id") ? ["Id"] : [];

export const createRepositoryBuilderOptions = <T>(
  type: Constructable<T>,
  connection: IConnection,
  types?: Record<string, TypeConverter<Cell, Cell>>,
): RepositoryBuilderOptions<T> => ({
  type,
  ids: defaultId(fieldNamesOf(type)),
  connection,
  types,
});
