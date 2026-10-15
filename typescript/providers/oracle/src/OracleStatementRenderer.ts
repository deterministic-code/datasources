import {
  CreateTableStatement,
  CreateTriggerStatement,
  DropTableStatement,
  StatementRenderer,
  type StatementTextRenderer,
} from "@datasources/core";
import { OracleCreateTableStatementRenderer } from "./OracleCreateTableStatementRenderer.ts";
import { OracleCreateTriggerStatementRenderer } from "./OracleCreateTriggerStatementRenderer.ts";
import { OracleDropTableStatementRenderer } from "./OracleDropTableStatementRenderer.ts";

export class OracleStatementRenderer extends StatementRenderer {
  constructor(text: StatementTextRenderer) {
    super(text);
  }

  protected override visitCreateTable(create: CreateTableStatement): string {
    return new OracleCreateTableStatementRenderer().render(create);
  }

  protected override visitCreateTrigger(trigger: CreateTriggerStatement): string {
    return new OracleCreateTriggerStatementRenderer().render(trigger);
  }

  protected override visitDropTable(drop: DropTableStatement): string {
    return new OracleDropTableStatementRenderer().render(drop);
  }
}
