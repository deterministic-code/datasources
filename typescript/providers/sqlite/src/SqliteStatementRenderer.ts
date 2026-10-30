import {
  AlterTableStatement,
  CreateIndexStatement,
  CreateTableStatement,
  CreateTriggerStatement,
  DropIndexStatement,
  DropTableStatement,
  DropTriggerStatement,
  ForeignKeyStatement,
  SelectInsertedIdStatement,
  SelectInsertedIdStatementTextRenderer,
  SqlSeedStatement,
  StatementRenderer,
  type StatementTextRenderer,
} from "@deterministic-code/datasources";
import { SqliteAlterTableStatementRenderer } from "./SqliteAlterTableStatementRenderer.ts";
import { SqliteCreateTableStatementRenderer } from "./SqliteCreateTableStatementRenderer.ts";
import { SqliteCreateTriggerStatementRenderer } from "./SqliteCreateTriggerStatementRenderer.ts";
import {
  SqliteCreateIndexStatementRenderer,
  SqliteDropIndexStatementRenderer,
  SqliteDropTableStatementRenderer,
  SqliteDropTriggerStatementRenderer,
  SqliteForeignKeyStatementRenderer,
} from "./SqliteQuotedRenderers.ts";
import { SqliteSqlSeedStatementRenderer } from "./SqliteSqlSeedStatementRenderer.ts";

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

  protected override visitSqlSeed(seed: SqlSeedStatement): string {
    return new SqliteSqlSeedStatementRenderer().render(seed);
  }

  protected override visitDropTable(drop: DropTableStatement): string {
    return new SqliteDropTableStatementRenderer().render(drop);
  }

  protected override visitCreateIndex(index: CreateIndexStatement): string {
    return new SqliteCreateIndexStatementRenderer().render(index);
  }

  protected override visitDropIndex(index: DropIndexStatement): string {
    return new SqliteDropIndexStatementRenderer().render(index);
  }

  protected override visitDropTrigger(trigger: DropTriggerStatement): string {
    return new SqliteDropTriggerStatementRenderer().render(trigger);
  }

  protected override visitForeignKey(key: ForeignKeyStatement): string {
    return new SqliteForeignKeyStatementRenderer().render(key);
  }
}
