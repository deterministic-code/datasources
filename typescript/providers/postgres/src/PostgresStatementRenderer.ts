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
  DropRoutineStatement,
  DropTriggerStatement,
  IdentityInsertStatement,
  SelectInsertedIdStatement,
  SelectInsertedIdStatementTextRenderer,
  StatementRenderer,
  type StatementTextRenderer,
} from "@deterministic-code/datasources";
import { PostgresAlterTableStatementRenderer } from "./PostgresAlterTableStatementRenderer.ts";
import { PostgresBulkUpdateStatementTextRenderer } from "./PostgresBulkUpdateStatementTextRenderer.ts";
import { PostgresCreateTableStatementRenderer } from "./PostgresCreateTableStatementRenderer.ts";
import { PostgresCreateTriggerStatementRenderer } from "./PostgresCreateTriggerStatementRenderer.ts";
import { PostgresDropRoutineStatementRenderer } from "./PostgresDropRoutineStatementRenderer.ts";
import { PostgresStoredProcedureStatementRenderer } from "./PostgresStoredProcedureStatementRenderer.ts";
import { PostgresDropTriggerStatementRenderer } from "./PostgresDropTriggerStatementRenderer.ts";
import { PostgresIdentityInsertStatementRenderer } from "./PostgresIdentityInsertStatementRenderer.ts";

class PostgresSelectInsertedIdStatementTextRenderer extends SelectInsertedIdStatementTextRenderer {
  override render(statement: SelectInsertedIdStatement): string {
    const { table, column } = statement;
    return `SELECT setval(pg_get_serial_sequence('${table}', '${column}'), COALESCE((SELECT MAX("${column}") FROM ${table}), 1)) AS ${column}`;
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

  protected override visitAlterTable(alter: AlterTableStatement): string {
    return new PostgresAlterTableStatementRenderer().renderAlter(alter);
  }

  protected override visitDropTrigger(trigger: DropTriggerStatement): string {
    return new PostgresDropTriggerStatementRenderer().render(trigger);
  }

  protected override visitBulkUpdate(update: BulkUpdateStatement): string {
    return new PostgresBulkUpdateStatementTextRenderer().render(update);
  }

  protected override visitCreateTrigger(trigger: CreateTriggerStatement): string {
    return new PostgresCreateTriggerStatementRenderer().render(trigger);
  }

  protected override visitIdentityInsert(insert: IdentityInsertStatement): string {
    return new PostgresIdentityInsertStatementRenderer().render(insert);
  }

  protected override visitDropRoutine(routine: DropRoutineStatement): string {
    return new PostgresDropRoutineStatementRenderer().render(routine);
  }

  protected override visitCreateCreateStoredProcedureStatement(statement: CreateCreateStoredProcedureStatement): string {
    return new PostgresStoredProcedureStatementRenderer().renderCreate(statement);
  }

  protected override visitCreateReadStoredProcedureStatement(statement: CreateReadStoredProcedureStatement): string {
    return new PostgresStoredProcedureStatementRenderer().renderRead(statement);
  }

  protected override visitCreateNestedReadStoredProcedureStatement(statement: CreateNestedReadStoredProcedureStatement): string {
    return new PostgresStoredProcedureStatementRenderer().renderNestedRead(statement);
  }

  protected override visitCreateUpdateStoredProcedureStatement(statement: CreateUpdateStoredProcedureStatement): string {
    return new PostgresStoredProcedureStatementRenderer().renderUpdate(statement);
  }

  protected override visitCreateDeleteStoredProcedureStatement(statement: CreateDeleteStoredProcedureStatement): string {
    return new PostgresStoredProcedureStatementRenderer().renderDelete(statement);
  }

  protected override visitCreateBulkCreateStoredProcedureStatement(statement: CreateBulkCreateStoredProcedureStatement): string {
    return new PostgresStoredProcedureStatementRenderer().renderBulkCreate(statement);
  }

  protected override visitCreateBulkUpdateStoredProcedureStatement(statement: CreateBulkUpdateStoredProcedureStatement): string {
    return new PostgresStoredProcedureStatementRenderer().renderBulkUpdate(statement);
  }

  protected override visitCreateBulkDeleteStoredProcedureStatement(statement: CreateBulkDeleteStoredProcedureStatement): string {
    return new PostgresStoredProcedureStatementRenderer().renderBulkDelete(statement);
  }
}
