import {
  CreateRoutineStatement,
  CreateTableStatement,
  CreateTriggerStatement,
  DropRoutineStatement,
  IdentityInsertStatement,
  StatementRenderer,
  type StatementTextRenderer,
} from "@datasources/core";
import { SqlServerCreateRoutineStatementRenderer } from "./SqlServerCreateRoutineStatementRenderer.ts";
import { SqlServerCreateTableStatementRenderer } from "./SqlServerCreateTableStatementRenderer.ts";
import { SqlServerCreateTriggerStatementRenderer } from "./SqlServerCreateTriggerStatementRenderer.ts";
import { SqlServerDropRoutineStatementRenderer } from "./SqlServerDropRoutineStatementRenderer.ts";
import { SqlServerIdentityInsertStatementRenderer } from "./SqlServerIdentityInsertStatementRenderer.ts";

export class SqlServerStatementRenderer extends StatementRenderer {
  constructor(text: StatementTextRenderer) {
    super(text);
  }

  protected override visitCreateTable(create: CreateTableStatement): string {
    return new SqlServerCreateTableStatementRenderer().render(create);
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
