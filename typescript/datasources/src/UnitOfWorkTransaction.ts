import type { IConnection } from "./IConnection.ts";

const open = new WeakSet<IConnection>();

export class UnitOfWorkTransaction {
  constructor(private readonly connection: IConnection) {}

  doWork = async <TResult>(work: () => Promise<TResult>): Promise<TResult> => {
    if (open.has(this.connection)) return work();
    open.add(this.connection);
    try {
      const txn = await this.connection.startTransaction();
      try {
        const result = await work();
        await txn.commit();
        return result;
      } catch (error) {
        await txn.abort();
        throw error;
      }
    } finally {
      open.delete(this.connection);
    }
  };
}
