import {
  ArrayReader,
  CommandMiddlewareHost,
  Native,
  StatementTextRenderer,
  type Cells,
  type ICommand,
  type IReader,
  type IStatement,
  type Parameter,
  Response,
} from "@deterministic-code/datasources";
import type { MemoryConnection } from "./connection.ts";
import { MemoryStatementRenderer } from "./MemoryStatementRenderer.ts";

export class MemoryCommand extends CommandMiddlewareHost implements ICommand {
  private readonly statements: StatementTextRenderer;

  constructor(readonly connection: MemoryConnection, statements?: StatementTextRenderer) {
    super();
    this.statements = statements ?? new StatementTextRenderer((t) => new MemoryStatementRenderer(t));
  }

  executeReaderNative(text: Native, parameters?: Parameter[]): Promise<IReader<Cells>> {
    return this.executeReader(text.text, parameters);
  }

  executeNative(text: Native, parameters?: Parameter[]): Promise<Response> {
    return this.execute(text.text, parameters);
  }

  async executeReader(text: string, parameters?: Parameter[]): Promise<IReader<Cells>> {
    return this.run(text, parameters, async (query, params) => {
      this.connection.ensureOpen();
      this.connection.engine.execute(query, params);
      const rows = this.connection.engine.records as Cells[];
      return { value: new ArrayReader(rows), rows };
    });
  }

  executeReaderStatement = (statement: IStatement, parameters?: Parameter[]): Promise<IReader<Cells>> =>
    this.executeReader(this.statements.render(statement), parameters);

  executeStatement = (statement: IStatement, parameters?: Parameter[]): Promise<Response> =>
    this.execute(this.statements.render(statement), parameters);

  async execute(text: string, parameters?: Parameter[]): Promise<Response> {
    if (!text.trim()) throw new Error("empty command");
    return this.run(text, parameters, async (query, params) => {
      this.connection.ensureOpen();
      return { value: this.connection.engine.execute(query, params), rows: [] };
    });
  }
}
