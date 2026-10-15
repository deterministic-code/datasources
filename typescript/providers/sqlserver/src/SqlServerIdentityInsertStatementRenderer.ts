import {
  IdentityInsertStatementTextRenderer,
  type IdentityInsertStatement,
} from "@datasources/core";

export class SqlServerIdentityInsertStatementRenderer extends IdentityInsertStatementTextRenderer {
  override render(statement: IdentityInsertStatement): string {
    const inserts = super.render(statement);
    if (!statement.identity || statement.inserts.length === 0) return inserts;
    const quoted = statement.inserts[0]!.table;
    return `SET IDENTITY_INSERT ${quoted} ON;
${inserts}
SET IDENTITY_INSERT ${quoted} OFF`;
  }
}
