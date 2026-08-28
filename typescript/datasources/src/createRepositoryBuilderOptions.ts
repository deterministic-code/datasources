import type { IConnection } from "./IConnection.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { RepositoryBuilderOptions } from "./RepositoryBuilderOptions.ts";

const fieldNamesOf = <T>(type: Constructable<T>): string[] =>
  Object.keys(new type() as object);

const defaultId = (fields: string[]): string[] =>
  fields.includes("id") ? ["id"] : fields.includes("Id") ? ["Id"] : [];

export const createRepositoryBuilderOptions = <T extends Cells>(
  type: Constructable<T>,
  connection: IConnection,
): RepositoryBuilderOptions<T> => ({
  type,
  ids: defaultId(fieldNamesOf(type)),
  connection,
});
