import type { IConnection } from "./IConnection.ts";

export class UnitOfWork {
  constructor(private readonly connection: IConnection) {}

  inTransaction = async <TResult>(work: () => Promise<TResult>): Promise<TResult> => {
    const txn = await this.connection.startTransaction();
    try {
      const result = await work();
      await txn.commit();
      return result;
    } catch (error) {
      await txn.abort();
      throw error;
    }
  };
}
