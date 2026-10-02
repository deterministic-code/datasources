import {
  CreateTableStatement,
  SelectInsertedIdStatement,
  SelectInsertedIdStatementTextRenderer,
  StatementRenderer,
  type StatementTextRenderer,
} from "@datasources/core";
import { SqliteCreateTableStatementRenderer } from "./SqliteCreateTableStatementRenderer.ts";

class SqliteSelectInsertedIdStatementTextRenderer extends SelectInsertedIdStatementTextRenderer {
  override render(statement: SelectInsertedIdStatement): string {
    return `SELECT last_insert_rowid() AS ${statement.column}`;
  }
}

export class SqliteStatementRenderer extends StatementRenderer {
  constructor(text: StatementTextRenderer) {
    super(text);
  }

  protected override visitSelectInsertedId(inserted: SelectInsertedIdStatement): string {
    return new SqliteSelectInsertedIdStatementTextRenderer().render(inserted);
  }

  protected override visitCreateTable(create: CreateTableStatement): string {
    return new SqliteCreateTableStatementRenderer().render(create);
  }
}
