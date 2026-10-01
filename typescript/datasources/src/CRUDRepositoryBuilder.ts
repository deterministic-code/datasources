import type { IConnection } from "./IConnection.ts";
import { defaultId, fieldNamesOf, createCRUDRepositoryOptions } from "./createCRUDRepositoryOptions.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import { CRUDRepositoryStatements } from "./CRUDRepositoryStatements.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import type { Constructable } from "./converters/Constructable.ts";
import { ReadonlyRepositoryBuilder } from "./ReadonlyRepositoryBuilder.ts";
import { ReadonlyRepositoryStatements } from "./ReadonlyRepositoryStatements.ts";
import type { ReadonlyRepositoryOptions } from "./ReadonlyRepositoryOptions.ts";

const isCrud = <T>(options: ReadonlyRepositoryOptions<T>): options is CRUDRepositoryOptions<T> =>
  "ids" in options || "write" in options;

const withIds = <T>(options: CRUDRepositoryOptions<T>): CRUDRepositoryOptions<T> =>
  "ids" in options
    ? options
    : {
        ...options,
        ids: options.type ? defaultId(fieldNamesOf(options.type)) : ["id"],
      };

export const repositoryParts = <T>(
  typeOrConnection: Constructable<T> | IConnection,
  connectionOrOptions?: IConnection | CRUDRepositoryOptions<T>,
): { built: BuiltRepository<T>; statements: CRUDRepositoryStatements } => {
  const builder = new CRUDRepositoryBuilder<T>();
  const built =
    typeof typeOrConnection === "function"
      ? builder.parts(
          connectionOrOptions as IConnection,
          createCRUDRepositoryOptions(typeOrConnection),
        )
      : builder.parts(typeOrConnection, withIds(connectionOrOptions as CRUDRepositoryOptions<T>));
  return { built, statements: builder.crudStatements! };
};

export class CRUDRepositoryBuilder<T = object> extends ReadonlyRepositoryBuilder<T> {
  crudStatements?: CRUDRepositoryStatements;

  protected override ids = (options: ReadonlyRepositoryOptions<T>): string[] =>
    isCrud(options)
      ? (options.ids ?? (options.type ? defaultId(fieldNamesOf(options.type)) : ["id"]))
      : [];

  protected override statements = (table: string, columns: string[]): ReadonlyRepositoryStatements => {
    this.crudStatements = new CRUDRepositoryStatements(table, columns, this.entityMapper, this.propertyMapper);
    return new ReadonlyRepositoryStatements(table, columns);
  };

  build = (connection: IConnection, options: CRUDRepositoryOptions<T>): CRUDRepository<T> =>
    new CRUDRepository(connection, options);
}
