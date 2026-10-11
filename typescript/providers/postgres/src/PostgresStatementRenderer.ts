import {
  BulkUpdateStatement,
  CreateTableStatement,
  SelectInsertedIdStatement,
  SelectInsertedIdStatementTextRenderer,
  StatementRenderer,
  type StatementTextRenderer,
} from "@datasources/core";
import { PostgresBulkUpdateStatementTextRenderer } from "./PostgresBulkUpdateStatementTextRenderer.ts";
import { PostgresCreateTableStatementRenderer } from "./PostgresCreateTableStatementRenderer.ts";

class PostgresSelectInsertedIdStatementTextRenderer extends SelectInsertedIdStatementTextRenderer {
  override render(statement: SelectInsertedIdStatement): string {
    return `SELECT lastval() AS ${statement.column}`;
  }
}

export class PostgresStatementRenderer extends StatementRenderer {
  constructor(text: StatementTextRenderer) {
    super(text);
  }

  protected override visitSelectInsertedId(inserted: SelectInsertedIdStatement): string {
    return new PostgresSelectInsertedIdStatementTextRenderer().render(inserted);
  }

  protected override visitCreateTable(create: CreateTableStatement): string {
    return new PostgresCreateTableStatementRenderer().render(create);
  }

  protected override visitBulkUpdate(update: BulkUpdateStatement): string {
    return new PostgresBulkUpdateStatementTextRenderer().render(update);
  }
}
