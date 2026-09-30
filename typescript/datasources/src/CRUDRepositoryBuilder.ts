import type { IConnection } from "./IConnection.ts";
import { defaultId, fieldNamesOf } from "./createCRUDRepositoryOptions.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import { CRUDRepositoryStatements } from "./CRUDRepositoryStatements.ts";
import { ReadonlyRepositoryBuilder } from "./ReadonlyRepositoryBuilder.ts";
import type { ReadonlyRepositoryOptions } from "./ReadonlyRepositoryOptions.ts";
import type { ReadonlyRepositoryStatements } from "./ReadonlyRepositoryStatements.ts";

const isCrud = <T>(options: ReadonlyRepositoryOptions<T>): options is CRUDRepositoryOptions<T> =>
  "ids" in options || "write" in options;

export class CRUDRepositoryBuilder<T = object> extends ReadonlyRepositoryBuilder<T> {
  protected override ids = (options: ReadonlyRepositoryOptions<T>): string[] =>
    isCrud(options)
      ? (options.ids ?? (options.type ? defaultId(fieldNamesOf(options.type)) : ["id"]))
      : [];

  protected override statements = (table: string, columns: string[]): ReadonlyRepositoryStatements =>
    new CRUDRepositoryStatements(table, columns, this.entityMapper, this.propertyMapper);

  build = (connection: IConnection, options: CRUDRepositoryOptions<T>): CRUDRepository<T> =>
    new CRUDRepository(connection, options);
}
