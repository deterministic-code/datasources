import type { IConnection } from "./IConnection.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { RepositoryBuilderOptions } from "./RepositoryBuilderOptions.ts";

type FieldSource = Record<string, string | number | boolean | bigint | null | object>;

export const fieldNamesOf = <T>(type: Constructable<T>): string[] => {
  const instance = new type() as FieldSource;
  return Object.keys(instance).filter((key) => !Array.isArray(instance[key]));
};

const defaultId = (fields: string[]): string[] =>
  fields.includes("id") ? ["id"] : fields.includes("Id") ? ["Id"] : [];

export const createRepositoryBuilderOptions = <T>(
  type: Constructable<T>,
  connection: IConnection,
): RepositoryBuilderOptions<T> => ({
  type,
  ids: defaultId(fieldNamesOf(type)),
  connection,
});
