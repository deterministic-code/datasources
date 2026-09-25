import type { IConnection } from "./IConnection.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { Cells } from "./converters/Cell.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import type { IWriteEnrichmentStrategy } from "./IWriteEnrichmentStrategy.ts";
import type { Response } from "./Response.ts";
import type { IWhere } from "./statements/IWhere.ts";

const createdId = (item: object): Scalar | undefined => {
  const record = item as Record<string, Scalar | object>;
  const value = record.id ?? record.Id;
  return value === undefined || value === null || typeof value === "object" ? undefined : value;
};

const fold = async <T>(
  items: T[],
  write: (item: T) => Promise<Response>,
): Promise<Response> => {
  let result: Response["result"] = null;
  let rowsAffected = 0;
  for (const item of items) {
    const response = await write(item);
    result = response.result;
    rowsAffected += response.rowsAffected;
  }
  return { result, rowsAffected };
};

export class BulkCRUDRepository<T extends object = Cells> extends CRUDRepository<T> {
  constructor(type: Constructable<T>, connection: IConnection);
  constructor(connection: IConnection, options: CRUDRepositoryOptions<T>);
  constructor(built: BuiltRepository<T>);
  constructor(write?: IWriteEnrichmentStrategy<T>);
  constructor(
    typeOrConnectionOrBuiltOrWrite?:
      | Constructable<T>
      | IConnection
      | BuiltRepository<T>
      | IWriteEnrichmentStrategy<T>,
    connectionOrOptions?: IConnection | CRUDRepositoryOptions<T>,
  ) {
    super(
      typeOrConnectionOrBuiltOrWrite as Constructable<T> & IConnection & BuiltRepository<T> & IWriteEnrichmentStrategy<T>,
      connectionOrOptions as IConnection & CRUDRepositoryOptions<T>,
    );
  }

  createMany = async (items: T[]): Promise<Response> => {
    let result: Response["result"] = null;
    for (const item of items) {
      const created = await this.create(item);
      const id = createdId(created as object);
      if (id !== undefined) result = id;
    }
    return { result, rowsAffected: items.length };
  };

  updateMany = (items: T[], byWhere?: IWhere): Promise<Response> =>
    fold(items, (item) => this.update(item, byWhere));

  deleteMany = (items: T[], byWhere?: IWhere): Promise<Response> =>
    fold(items, (item) => this.delete(item, byWhere));

  createBulk = (items: T[]): Promise<Response> => this.createMany(items);

  updateBulk = (items: T[], byWhere?: IWhere): Promise<Response> =>
    this.updateMany(items, byWhere);

  deleteBulk = (items: T[], byWhere?: IWhere): Promise<Response> =>
    this.deleteMany(items, byWhere);

  bulkCreate = (items: T[]): Promise<Response> => this.createMany(items);

  bulkUpdate = (items: T[], byWhere?: IWhere): Promise<Response> =>
    this.updateMany(items, byWhere);

  bulkDelete = (items: T[], byWhere?: IWhere): Promise<Response> =>
    this.deleteMany(items, byWhere);
}
