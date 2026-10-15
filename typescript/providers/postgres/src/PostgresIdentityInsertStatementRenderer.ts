import {
  IdentityInsertStatementTextRenderer,
  type IdentityInsertStatement,
} from "@datasources/core";

export class PostgresIdentityInsertStatementRenderer extends IdentityInsertStatementTextRenderer {
  override render(statement: IdentityInsertStatement): string {
    const inserts = super.render(statement);
    if (!statement.identity || statement.inserts.length === 0) return inserts;
    const quoted = statement.inserts[0]!.table;
    return `${inserts}
SELECT setval(pg_get_serial_sequence('${statement.table}', '${statement.idColumn}'), (SELECT MAX("${statement.idColumn}") FROM ${quoted}))`;
  }
}
