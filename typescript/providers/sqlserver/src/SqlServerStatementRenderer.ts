import {
  AlterTableStatement,
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
  DropIndexStatement,
  DropRoutineStatement,
  IdentityInsertStatement,
  StatementRenderer,
  type StatementTextRenderer,
} from "@deterministic-code/datasources";
import { SqlServerAlterTableStatementRenderer } from "./SqlServerAlterTableStatementRenderer.ts";
import { SqlServerCreateTableStatementRenderer } from "./SqlServerCreateTableStatementRenderer.ts";
import { SqlServerCreateTriggerStatementRenderer } from "./SqlServerCreateTriggerStatementRenderer.ts";
import { SqlServerDropIndexStatementRenderer } from "./SqlServerDropIndexStatementRenderer.ts";
import { SqlServerDropRoutineStatementRenderer } from "./SqlServerDropRoutineStatementRenderer.ts";
import { SqlServerStoredProcedureStatementRenderer } from "./SqlServerStoredProcedureStatementRenderer.ts";
import { SqlServerIdentityInsertStatementRenderer } from "./SqlServerIdentityInsertStatementRenderer.ts";

export class SqlServerStatementRenderer extends StatementRenderer {
  constructor(text: StatementTextRenderer) {
    super(text);
  }

  protected override visitCreateTable(create: CreateTableStatement): string {
    return new SqlServerCreateTableStatementRenderer().render(create);
  }

  protected override visitAlterTable(alter: AlterTableStatement): string {
    return new SqlServerAlterTableStatementRenderer().renderAlter(alter);
  }

  protected override visitDropIndex(index: DropIndexStatement): string {
    return new SqlServerDropIndexStatementRenderer().render(index);
  }

  protected override visitCreateTrigger(trigger: CreateTriggerStatement): string {
    return new SqlServerCreateTriggerStatementRenderer().render(trigger);
  }

  protected override visitIdentityInsert(insert: IdentityInsertStatement): string {
    return new SqlServerIdentityInsertStatementRenderer().render(insert);
  }

  protected override visitDropRoutine(routine: DropRoutineStatement): string {
    return new SqlServerDropRoutineStatementRenderer().render(routine);
  }

  protected override visitCreateCreateStoredProcedureStatement(statement: CreateCreateStoredProcedureStatement): string {
    return new SqlServerStoredProcedureStatementRenderer().renderCreate(statement);
  }

  protected override visitCreateReadStoredProcedureStatement(statement: CreateReadStoredProcedureStatement): string {
    return new SqlServerStoredProcedureStatementRenderer().renderRead(statement);
  }

  protected override visitCreateNestedReadStoredProcedureStatement(statement: CreateNestedReadStoredProcedureStatement): string {
    return new SqlServerStoredProcedureStatementRenderer().renderNestedRead(statement);
  }

  protected override visitCreateUpdateStoredProcedureStatement(statement: CreateUpdateStoredProcedureStatement): string {
    return new SqlServerStoredProcedureStatementRenderer().renderUpdate(statement);
  }

  protected override visitCreateDeleteStoredProcedureStatement(statement: CreateDeleteStoredProcedureStatement): string {
    return new SqlServerStoredProcedureStatementRenderer().renderDelete(statement);
  }

  protected override visitCreateBulkCreateStoredProcedureStatement(statement: CreateBulkCreateStoredProcedureStatement): string {
    return new SqlServerStoredProcedureStatementRenderer().renderBulkCreate(statement);
  }

  protected override visitCreateBulkUpdateStoredProcedureStatement(statement: CreateBulkUpdateStoredProcedureStatement): string {
    return new SqlServerStoredProcedureStatementRenderer().renderBulkUpdate(statement);
  }

  protected override visitCreateBulkDeleteStoredProcedureStatement(statement: CreateBulkDeleteStoredProcedureStatement): string {
    return new SqlServerStoredProcedureStatementRenderer().renderBulkDelete(statement);
  }
}
