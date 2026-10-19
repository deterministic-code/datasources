import { BulkUpdateStatementTextRenderer, type BulkUpdateStatement } from "@deterministic-code/datasources";

export class MysqlBulkUpdateStatementTextRenderer extends BulkUpdateStatementTextRenderer {
  override render = (statement: BulkUpdateStatement): string => {
    const assigns = statement.set
      .map((column) => `${statement.table}.${column} = data.${column}`)
      .join(", ");
    return `UPDATE ${statement.table} JOIN ${this.source(statement)} ON ${this.on(statement)} SET ${assigns}`;
  };
}
