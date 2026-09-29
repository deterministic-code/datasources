import type { ICommand } from "../ICommand.ts";
import type { IConnection } from "../IConnection.ts";
import type { Cells } from "../converters/Cell.ts";
import type { IReader } from "../IReader.ts";
import type { Response } from "../Response.ts";
import { CommandMiddlewareHost } from "../CommandMiddlewareHost.ts";
import type { Parameter } from "../Parameter.ts";
import type { Row } from "../Row.ts";
import type { IStatement } from "../statements/IStatement.ts";
import { SelectInsertedIdStatement } from "../statements/SelectInsertedIdStatement.ts";
import { MockReader } from "./mock-reader.ts";

export class MockCommand extends CommandMiddlewareHost implements ICommand {
  executeCalls: { text: string; parameters?: Parameter[] }[] = [];
  executeReaderCalls: { text: string; parameters?: Parameter[] }[] = [];
  executeError?: unknown;
  executeReaderError?: unknown;
  response: Response = { result: undefined, rowsAffected: 0 };
  reader: IReader<Cells> = new MockReader();

  constructor(readonly connection: IConnection) {
    super();
  }

  executeReaderStatement = (
    statement: IStatement,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> =>
    this.executeReader(
      statement instanceof SelectInsertedIdStatement
        ? `${statement.table}.${statement.column}`
        : statement.getText(),
      parameters,
    );

  async executeReader(
    text: string,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> {
    return this.run(text, parameters, async (query, params) => {
      this.executeReaderCalls.push({ text: query, parameters: params });
      if (this.executeReaderError) {
        throw this.executeReaderError;
      }
      const rows =
        this.reader instanceof MockReader
          ? (this.reader.rows as readonly Row[])
          : [];
      return { value: this.reader, rows };
    });
  }

  async execute(
    text: string,
    parameters?: Parameter[],
  ): Promise<Response> {
    return this.run(text, parameters, async (query, params) => {
      this.executeCalls.push({ text: query, parameters: params });
      if (this.executeError) {
        throw this.executeError;
      }
      return { value: this.response, rows: [] };
    });
  }
}
