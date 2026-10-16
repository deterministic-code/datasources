import {
  AlterTableStatement,
  CreateTableStatement,
  CreateTriggerStatement,
  SelectInsertedIdStatement,
  SelectInsertedIdStatementTextRenderer,
  StatementRenderer,
  type StatementTextRenderer,
} from "@datasources/core";
import { SqliteAlterTableStatementRenderer } from "./SqliteAlterTableStatementRenderer.ts";
import { SqliteCreateTableStatementRenderer } from "./SqliteCreateTableStatementRenderer.ts";
import { SqliteCreateTriggerStatementRenderer } from "./SqliteCreateTriggerStatementRenderer.ts";

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

  protected override visitAlterTable(alter: AlterTableStatement): string {
    return new SqliteAlterTableStatementRenderer().renderAlter(alter);
  }

  protected override visitCreateTrigger(trigger: CreateTriggerStatement): string {
    return new SqliteCreateTriggerStatementRenderer().render(trigger);
  }
}
