import {
  BulkUpdateStatement,
  CreateTableStatement,
  SelectInsertedIdStatement,
  SelectInsertedIdStatementTextRenderer,
  StatementRenderer,
  type StatementTextRenderer,
} from "@datasources/core";
import { MysqlBulkUpdateStatementTextRenderer } from "./MysqlBulkUpdateStatementTextRenderer.ts";
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

  protected override visitBulkUpdate(update: BulkUpdateStatement): string {
    return new MysqlBulkUpdateStatementTextRenderer().render(update);
  }
}
