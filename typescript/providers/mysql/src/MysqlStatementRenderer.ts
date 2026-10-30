import {
  AlterTableStatement,
  BulkUpdateStatement,
  CreateBulkCreateStoredProcedureStatement,
  CreateBulkDeleteStoredProcedureStatement,
  CreateBulkUpdateStoredProcedureStatement,
  CreateCreateStoredProcedureStatement,
  CreateDeleteStoredProcedureStatement,
  CreateNestedReadStoredProcedureStatement,
  CreateReadStoredProcedureStatement,
  CreateTableStatement,
  CreateTriggerStatement,
  CreateUpdateStoredProcedureStatement,
  CreateIndexStatement,
  DropIndexStatement,
  DropRoutineStatement,
  DropTableStatement,
  ForeignKeyStatement,
  SelectInsertedIdStatement,
  SelectInsertedIdStatementTextRenderer,
  SqlSeedStatement,
  StatementRenderer,
  type StatementTextRenderer,
} from "@deterministic-code/datasources";
import { MysqlAlterTableStatementRenderer } from "./MysqlAlterTableStatementRenderer.ts";
import { MysqlBulkUpdateStatementTextRenderer } from "./MysqlBulkUpdateStatementTextRenderer.ts";
import { MysqlCreateTableStatementRenderer } from "./MysqlCreateTableStatementRenderer.ts";
import { MysqlCreateTriggerStatementRenderer } from "./MysqlCreateTriggerStatementRenderer.ts";
import { MysqlDropIndexStatementRenderer } from "./MysqlDropIndexStatementRenderer.ts";
import { MysqlDropRoutineStatementRenderer } from "./MysqlDropRoutineStatementRenderer.ts";
import { MysqlStoredProcedureStatementRenderer } from "./MysqlStoredProcedureStatementRenderer.ts";
import { MysqlSqlSeedStatementRenderer } from "./MysqlSqlSeedStatementRenderer.ts";
import {
  MysqlCreateIndexStatementRenderer,
  MysqlDropTableStatementRenderer,
  MysqlForeignKeyStatementRenderer,
} from "./MysqlQuotedRenderers.ts";

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

  protected override visitAlterTable(alter: AlterTableStatement): string {
    return new MysqlAlterTableStatementRenderer().renderAlter(alter);
  }

  protected override visitDropIndex(index: DropIndexStatement): string {
    return new MysqlDropIndexStatementRenderer().render(index);
  }

  protected override visitBulkUpdate(update: BulkUpdateStatement): string {
    return new MysqlBulkUpdateStatementTextRenderer().render(update);
  }

  protected override visitCreateTrigger(trigger: CreateTriggerStatement): string {
    return new MysqlCreateTriggerStatementRenderer().render(trigger);
  }

  protected override visitDropRoutine(routine: DropRoutineStatement): string {
    return new MysqlDropRoutineStatementRenderer().render(routine);
  }

  protected override visitCreateCreateStoredProcedureStatement(statement: CreateCreateStoredProcedureStatement): string {
    return new MysqlStoredProcedureStatementRenderer().renderCreate(statement);
  }

  protected override visitCreateReadStoredProcedureStatement(statement: CreateReadStoredProcedureStatement): string {
    return new MysqlStoredProcedureStatementRenderer().renderRead(statement);
  }

  protected override visitCreateNestedReadStoredProcedureStatement(statement: CreateNestedReadStoredProcedureStatement): string {
    return new MysqlStoredProcedureStatementRenderer().renderNestedRead(statement);
  }

  protected override visitCreateUpdateStoredProcedureStatement(statement: CreateUpdateStoredProcedureStatement): string {
    return new MysqlStoredProcedureStatementRenderer().renderUpdate(statement);
  }

  protected override visitCreateDeleteStoredProcedureStatement(statement: CreateDeleteStoredProcedureStatement): string {
    return new MysqlStoredProcedureStatementRenderer().renderDelete(statement);
  }

  protected override visitCreateBulkCreateStoredProcedureStatement(statement: CreateBulkCreateStoredProcedureStatement): string {
    return new MysqlStoredProcedureStatementRenderer().renderBulkCreate(statement);
  }

  protected override visitCreateBulkUpdateStoredProcedureStatement(statement: CreateBulkUpdateStoredProcedureStatement): string {
    return new MysqlStoredProcedureStatementRenderer().renderBulkUpdate(statement);
  }

  protected override visitCreateBulkDeleteStoredProcedureStatement(statement: CreateBulkDeleteStoredProcedureStatement): string {
    return new MysqlStoredProcedureStatementRenderer().renderBulkDelete(statement);
  }

  protected override visitSqlSeed(seed: SqlSeedStatement): string {
    return new MysqlSqlSeedStatementRenderer().render(seed);
  }

  protected override visitDropTable(drop: DropTableStatement): string {
    return new MysqlDropTableStatementRenderer().render(drop);
  }

  protected override visitCreateIndex(index: CreateIndexStatement): string {
    return new MysqlCreateIndexStatementRenderer().render(index);
  }

  protected override visitForeignKey(key: ForeignKeyStatement): string {
    return new MysqlForeignKeyStatementRenderer().render(key);
  }
}
