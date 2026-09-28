import { DatasourceError } from "./DatasourceError.ts";
import type { IConnection } from "./IConnection.ts";
import type { ITransaction } from "./ITransaction.ts";
import type { IUnitOfWork } from "./IUnitOfWork.ts";

const open = new WeakSet<IConnection>();

export class UnitOfWorkTransaction implements IUnitOfWork {
  private begun = false;
  private transaction: ITransaction | undefined;

  constructor(private readonly connection: IConnection) {}

  begin = async (): Promise<void> => {
    if (this.begun) throw new DatasourceError("Unit of work already begun");
    this.begun = true;
    if (open.has(this.connection)) return;
    open.add(this.connection);
    try {
      this.transaction = await this.connection.startTransaction();
    } catch (error) {
      open.delete(this.connection);
      throw error;
    }
  };

  commit = async (): Promise<void> => {
    if (!this.begun) throw new DatasourceError("Unit of work has not begun");
    const txn = this.transaction;
    if (!txn) return;
    await txn.commit();
    this.release();
  };

  abort = async (): Promise<void> => {
    if (!this.begun) throw new DatasourceError("Unit of work has not begun");
    const txn = this.transaction;
    if (!txn) return;
    try {
      await txn.abort();
    } finally {
      this.release();
    }
  };

  private release(): void {
    open.delete(this.connection);
    this.transaction = undefined;
  }
}
