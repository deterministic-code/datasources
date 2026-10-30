import {
  AlterTableStatement,
  CreateBulkCreateStoredProcedureStatement,
  CreateBulkDeleteStoredProcedureStatement,
  CreateBulkUpdateStoredProcedureStatement,
  CreateCreateStoredProcedureStatement,
  CreateDeleteStoredProcedureStatement,
  CreateNestedReadStoredProcedureStatement,
  CreateReadStoredProcedureStatement,
  CreateIndexStatement,
  CreateTableStatement,
  CreateTriggerStatement,
  CreateUpdateStoredProcedureStatement,
  DropIndexStatement,
  DropTableStatement,
  DropTriggerStatement,
  ForeignKeyStatement,
  SqlSeedStatement,
  StatementRenderer,
  type StatementTextRenderer,
} from "@deterministic-code/datasources";
import { OracleAlterTableStatementRenderer } from "./OracleAlterTableStatementRenderer.ts";
import { OracleCreateTableStatementRenderer } from "./OracleCreateTableStatementRenderer.ts";
import { OracleCreateTriggerStatementRenderer } from "./OracleCreateTriggerStatementRenderer.ts";
import { OracleDropIndexStatementRenderer } from "./OracleDropIndexStatementRenderer.ts";
import { OracleDropTableStatementRenderer } from "./OracleDropTableStatementRenderer.ts";
import { OracleDropTriggerStatementRenderer } from "./OracleDropTriggerStatementRenderer.ts";
import { OracleStoredProcedureStatementRenderer } from "./OracleStoredProcedureStatementRenderer.ts";
import { OracleSqlSeedStatementRenderer } from "./OracleSqlSeedStatementRenderer.ts";
import { OracleCreateIndexStatementRenderer, OracleForeignKeyStatementRenderer } from "./OracleQuotedRenderers.ts";

export class OracleStatementRenderer extends StatementRenderer {
  constructor(text: StatementTextRenderer) {
    super(text);
  }

  protected override visitCreateTable(create: CreateTableStatement): string {
    return new OracleCreateTableStatementRenderer().render(create);
  }

  protected override visitAlterTable(alter: AlterTableStatement): string {
    return new OracleAlterTableStatementRenderer().renderAlter(alter);
  }

  protected override visitDropIndex(index: DropIndexStatement): string {
    return new OracleDropIndexStatementRenderer().render(index);
  }

  protected override visitDropTrigger(trigger: DropTriggerStatement): string {
    return new OracleDropTriggerStatementRenderer().render(trigger);
  }

  protected override visitCreateTrigger(trigger: CreateTriggerStatement): string {
    return new OracleCreateTriggerStatementRenderer().render(trigger);
  }

  protected override visitDropTable(drop: DropTableStatement): string {
    return new OracleDropTableStatementRenderer().render(drop);
  }

  protected override visitCreateCreateStoredProcedureStatement(statement: CreateCreateStoredProcedureStatement): string {
    return new OracleStoredProcedureStatementRenderer().renderCreate(statement);
  }

  protected override visitCreateReadStoredProcedureStatement(statement: CreateReadStoredProcedureStatement): string {
    return new OracleStoredProcedureStatementRenderer().renderRead(statement);
  }

  protected override visitCreateNestedReadStoredProcedureStatement(statement: CreateNestedReadStoredProcedureStatement): string {
    return new OracleStoredProcedureStatementRenderer().renderNestedRead(statement);
  }

  protected override visitCreateUpdateStoredProcedureStatement(statement: CreateUpdateStoredProcedureStatement): string {
    return new OracleStoredProcedureStatementRenderer().renderUpdate(statement);
  }

  protected override visitCreateDeleteStoredProcedureStatement(statement: CreateDeleteStoredProcedureStatement): string {
    return new OracleStoredProcedureStatementRenderer().renderDelete(statement);
  }

  protected override visitCreateBulkCreateStoredProcedureStatement(statement: CreateBulkCreateStoredProcedureStatement): string {
    return new OracleStoredProcedureStatementRenderer().renderBulkCreate(statement);
  }

  protected override visitCreateBulkUpdateStoredProcedureStatement(statement: CreateBulkUpdateStoredProcedureStatement): string {
    return new OracleStoredProcedureStatementRenderer().renderBulkUpdate(statement);
  }

  protected override visitCreateBulkDeleteStoredProcedureStatement(statement: CreateBulkDeleteStoredProcedureStatement): string {
    return new OracleStoredProcedureStatementRenderer().renderBulkDelete(statement);
  }

  protected override visitSqlSeed(seed: SqlSeedStatement): string {
    return new OracleSqlSeedStatementRenderer().render(seed);
  }

  protected override visitCreateIndex(index: CreateIndexStatement): string {
    return new OracleCreateIndexStatementRenderer().render(index);
  }

  protected override visitForeignKey(key: ForeignKeyStatement): string {
    return new OracleForeignKeyStatementRenderer().render(key);
  }
}
