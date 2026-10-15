import {
  BulkUpdateStatement,
  CreateRoutineStatement,
  CreateTableStatement,
  CreateTriggerStatement,
  DropRoutineStatement,
  IdentityInsertStatement,
  SelectInsertedIdStatement,
  SelectInsertedIdStatementTextRenderer,
  StatementRenderer,
  type StatementTextRenderer,
} from "@datasources/core";
import { PostgresBulkUpdateStatementTextRenderer } from "./PostgresBulkUpdateStatementTextRenderer.ts";
import { PostgresCreateRoutineStatementRenderer } from "./PostgresCreateRoutineStatementRenderer.ts";
import { PostgresCreateTableStatementRenderer } from "./PostgresCreateTableStatementRenderer.ts";
import { PostgresCreateTriggerStatementRenderer } from "./PostgresCreateTriggerStatementRenderer.ts";
import { PostgresDropRoutineStatementRenderer } from "./PostgresDropRoutineStatementRenderer.ts";
import { PostgresIdentityInsertStatementRenderer } from "./PostgresIdentityInsertStatementRenderer.ts";

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

  protected override visitCreateTrigger(trigger: CreateTriggerStatement): string {
    return new PostgresCreateTriggerStatementRenderer().render(trigger);
  }

  protected override visitIdentityInsert(insert: IdentityInsertStatement): string {
    return new PostgresIdentityInsertStatementRenderer().render(insert);
  }

  protected override visitCreateRoutine(routine: CreateRoutineStatement): string {
    return new PostgresCreateRoutineStatementRenderer().render(routine);
  }

  protected override visitDropRoutine(routine: DropRoutineStatement): string {
    return new PostgresDropRoutineStatementRenderer().render(routine);
  }
}
