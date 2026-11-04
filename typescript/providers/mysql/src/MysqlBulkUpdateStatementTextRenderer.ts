import { BulkUpdateStatementTextRenderer, type BulkUpdateStatement } from "@deterministic-code/datasources";
import { isKeyword, isVerbatim } from "./applyKeywords.ts";

const quote = (name: string): string =>
  isKeyword(name) || isVerbatim(name) ? `\`${name}\`` : name;
const parameter = (name: string): string => `:${name}`;

export class MysqlBulkUpdateStatementTextRenderer extends BulkUpdateStatementTextRenderer {
  override render = (statement: BulkUpdateStatement): string => {
    const assigns = statement.set
      .map((column) => `${this.quote(statement.table)}.${this.quote(column)} = data.${this.quote(column)}`)
      .join(", ");
    return `UPDATE ${this.quote(statement.table)} JOIN ${this.source(statement)} ON ${this.on(statement)} SET ${assigns}`;
  };

  protected override quote(name: string): string {
    return quote(name);
  }

  protected override parameter(name: string): string {
    return parameter(name);
  }
}
