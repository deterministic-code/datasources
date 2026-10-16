import {
  AlterTableStatement,
  CreateTableStatement,
  CreateTriggerStatement,
  DropIndexStatement,
  DropTableStatement,
  DropTriggerStatement,
  StatementRenderer,
  type StatementTextRenderer,
} from "@datasources/core";
import { OracleAlterTableStatementRenderer } from "./OracleAlterTableStatementRenderer.ts";
import { OracleCreateTableStatementRenderer } from "./OracleCreateTableStatementRenderer.ts";
import { OracleCreateTriggerStatementRenderer } from "./OracleCreateTriggerStatementRenderer.ts";
import { OracleDropIndexStatementRenderer } from "./OracleDropIndexStatementRenderer.ts";
import { OracleDropTableStatementRenderer } from "./OracleDropTableStatementRenderer.ts";
import { OracleDropTriggerStatementRenderer } from "./OracleDropTriggerStatementRenderer.ts";

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
}
