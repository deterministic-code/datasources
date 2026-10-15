import {
  BulkUpdateStatement,
  CreateRoutineStatement,
  CreateTableStatement,
  CreateTriggerStatement,
  DropRoutineStatement,
  SelectInsertedIdStatement,
  SelectInsertedIdStatementTextRenderer,
  StatementRenderer,
  type StatementTextRenderer,
} from "@datasources/core";
import { MysqlBulkUpdateStatementTextRenderer } from "./MysqlBulkUpdateStatementTextRenderer.ts";
import { MysqlCreateRoutineStatementRenderer } from "./MysqlCreateRoutineStatementRenderer.ts";
import { MysqlCreateTableStatementRenderer } from "./MysqlCreateTableStatementRenderer.ts";
import { MysqlCreateTriggerStatementRenderer } from "./MysqlCreateTriggerStatementRenderer.ts";
import { MysqlDropRoutineStatementRenderer } from "./MysqlDropRoutineStatementRenderer.ts";

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

  protected override visitBulkUpdate(update: BulkUpdateStatement): string {
    return new MysqlBulkUpdateStatementTextRenderer().render(update);
  }

  protected override visitCreateTrigger(trigger: CreateTriggerStatement): string {
    return new MysqlCreateTriggerStatementRenderer().render(trigger);
  }

  protected override visitCreateRoutine(routine: CreateRoutineStatement): string {
    return new MysqlCreateRoutineStatementRenderer().render(routine);
  }

  protected override visitDropRoutine(routine: DropRoutineStatement): string {
    return new MysqlDropRoutineStatementRenderer().render(routine);
  }
}
