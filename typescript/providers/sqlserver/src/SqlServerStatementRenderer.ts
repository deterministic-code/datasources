import {
  AlterTableStatement,
  CreateRoutineStatement,
  CreateTableStatement,
  CreateTriggerStatement,
  DropIndexStatement,
  DropRoutineStatement,
  IdentityInsertStatement,
  StatementRenderer,
  type StatementTextRenderer,
} from "@datasources/core";
import { SqlServerAlterTableStatementRenderer } from "./SqlServerAlterTableStatementRenderer.ts";
import { SqlServerCreateRoutineStatementRenderer } from "./SqlServerCreateRoutineStatementRenderer.ts";
import { SqlServerCreateTableStatementRenderer } from "./SqlServerCreateTableStatementRenderer.ts";
import { SqlServerCreateTriggerStatementRenderer } from "./SqlServerCreateTriggerStatementRenderer.ts";
import { SqlServerDropIndexStatementRenderer } from "./SqlServerDropIndexStatementRenderer.ts";
import { SqlServerDropRoutineStatementRenderer } from "./SqlServerDropRoutineStatementRenderer.ts";
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

  protected override visitCreateRoutine(routine: CreateRoutineStatement): string {
    return new SqlServerCreateRoutineStatementRenderer().render(routine);
  }

  protected override visitDropRoutine(routine: DropRoutineStatement): string {
    return new SqlServerDropRoutineStatementRenderer().render(routine);
  }
}
