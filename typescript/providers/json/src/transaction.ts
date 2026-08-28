import {
  IsolationLevel,
  MiddlewareHost,
  type ITransaction,
  type IsolationLevelName,
} from "@datasources/core";
import type { JsonConnection } from "./connection.ts";

export class JsonTransaction extends MiddlewareHost implements ITransaction {
  constructor(
    readonly connection: JsonConnection,
    readonly isolation: IsolationLevelName = IsolationLevel.Serializable,
  ) {
    super();
  }

  async commit(): Promise<void> {
    return this.run("commit", async () => {
      this.connection.finishCommit();
      if (this.connection.path) {
        await this.connection.fileStore.write(
          this.connection.path,
          JSON.stringify(this.connection.data),
        );
      }
    });
  }

  async abort(): Promise<void> {
    return this.run("abort", async () => {
      this.connection.finishAbort();
    });
  }
}
