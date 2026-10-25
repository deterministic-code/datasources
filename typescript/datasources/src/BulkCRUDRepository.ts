import type { IConnection } from "./IConnection.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { Cells } from "./converters/Cell.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import { createCRUDRepositoryOptions } from "./createCRUDRepositoryOptions.ts";
import type { Response } from "./Response.ts";
import type { IWhere } from "./statements/IWhere.ts";
import type { IBulkWriteStrategy } from "./IBulkWriteStrategy.ts";
import { SingleCommandIBulkWriteStrategy } from "./SingleCommandIBulkWriteStrategy.ts";
import { UnitOfWorkTransaction } from "./UnitOfWorkTransaction.ts";

const isBulkStrategy = <T extends object>(value: object): value is IBulkWriteStrategy<T> =>
  "create" in value &&
  "update" in value &&
  "delete" in value &&
  !("persist" in value) &&
  !("createCommand" in value) &&
  !("statements" in value);

const transactionOf = <T extends object>(
  options: IConnection | CRUDRepositoryOptions<T> | IBulkWriteStrategy<T> | undefined,
): boolean =>
  options !== undefined && "writeEnrichment" in options
    ? options.writeEnrichment?.useTransaction ?? false
    : false;

const inTransaction = async <TResult>(
  transactional: boolean,
  connection: IConnection,
  write: () => Promise<TResult>,
): Promise<TResult> => {
  if (!transactional) return write();
  const work = new UnitOfWorkTransaction(connection);
  await work.begin();
  try {
    const result = await write();
    await work.commit();
    return result;
  } catch (error) {
    await work.abort();
    throw error;
  }
};

export class BulkCRUDRepository<T extends object = Cells> extends CRUDRepository<T> {
  private readonly strategy: IBulkWriteStrategy<T>;
  private readonly transactional: boolean;

  constructor(type: Constructable<T>, connection: IConnection, bulk?: IBulkWriteStrategy<T>);
  constructor(connection: IConnection, options: CRUDRepositoryOptions<T>, bulk?: IBulkWriteStrategy<T>);
  constructor(
    typeOrConnection: Constructable<T> | IConnection,
    connectionOrOptions?: IConnection | CRUDRepositoryOptions<T> | IBulkWriteStrategy<T>,
    bulk?: IBulkWriteStrategy<T>,
  ) {
    const strategy = bulk ?? (isBulkStrategy<T>(connectionOrOptions ?? {}) ? connectionOrOptions : undefined);
    const transactional = transactionOf(connectionOrOptions);
    const parent = (
      isBulkStrategy(connectionOrOptions ?? {}) ? undefined : connectionOrOptions
    ) as IConnection & CRUDRepositoryOptions<T>;
    super(typeOrConnection as Constructable<T> & IConnection, parent);
    this.strategy = strategy ?? new SingleCommandIBulkWriteStrategy<T>();
    this.transactional = transactional;
    this.strategy.repo = this;
    this.strategy.attach(
      this.command(),
      typeof typeOrConnection === "function"
        ? createCRUDRepositoryOptions(typeOrConnection)
        : parent,
    );
  }

  createBulk = async (items: T[]): Promise<T[]> => {
    await inTransaction(this.transactional || items.length > 0, this.command().connection, () =>
      this.strategy.create(items),
    );
    return items;
  };

  updateBulk = (items: T[], byWhere?: IWhere): Promise<Response> =>
    inTransaction(this.transactional, this.command().connection, () => this.strategy.update(items, byWhere));

  deleteBulk = (items: T[], byWhere?: IWhere): Promise<Response> =>
    inTransaction(this.transactional, this.command().connection, () => this.strategy.delete(items, byWhere));
}
