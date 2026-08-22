import type { ICommand } from "../ICommand.ts";
import type { IConnection } from "../IConnection.ts";
import type { IReader } from "../IReader.ts";
import type { IResponse } from "../IResponse.ts";
import { MiddlewareHost } from "../MiddlewareHost.ts";
import type { Parameter } from "../Parameter.ts";
import { MockReader } from "./mock-reader.ts";

export class MockCommand extends MiddlewareHost implements ICommand {
  executeCalls: { text?: string | null; parameters?: Parameter[] }[] = [];
  executeReaderCalls: { text?: string | null; parameters?: Parameter[] }[] = [];
  executeError?: unknown;
  executeReaderError?: unknown;
  response: IResponse = { result: undefined, rowsAffected: 0 };
  reader: IReader = new MockReader();

  constructor(readonly connection: IConnection) {
    super();
  }

  async executeReader(
    text?: string | null,
    parameters?: Parameter[],
  ): Promise<IReader> {
    return this.run(
      "executeReader",
      async () => {
        this.executeReaderCalls.push({ text, parameters });
        if (this.executeReaderError) {
          throw this.executeReaderError;
        }
        return this.reader;
      },
      { text, parameters },
    );
  }

  async execute(
    text?: string | null,
    parameters?: Parameter[],
  ): Promise<IResponse> {
    return this.run(
      "execute",
      async () => {
        this.executeCalls.push({ text, parameters });
        if (this.executeError) {
          throw this.executeError;
        }
        return this.response;
      },
      { text, parameters },
    );
  }
}
