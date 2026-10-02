import {
  CreateTableStatement,
  SelectInsertedIdStatement,
  SelectInsertedIdStatementTextRenderer,
  StatementRenderer,
  type StatementTextRenderer,
} from "@datasources/core";
import { MysqlCreateTableStatementRenderer } from "./MysqlCreateTableStatementRenderer.ts";

class MysqlSelectInsertedIdStatementTextRenderer extends SelectInsertedIdStatementTextRenderer {
  override render(statement: SelectInsertedIdStatement): string {
    return `SELECT LAST_INSERT_ID() AS ${statement.column}`;
  }
}

export class MysqlStatementRenderer extends StatementRenderer {
  constructor(text: StatementTextRenderer) {
    super(text);
  }

  protected override visitSelectInsertedId(inserted: SelectInsertedIdStatement): string {
    return new MysqlSelectInsertedIdStatementTextRenderer().render(inserted);
  }

  protected override visitCreateTable(create: CreateTableStatement): string {
    return new MysqlCreateTableStatementRenderer().render(create);
  }
}
