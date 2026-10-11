import type { Cell } from "../converters/Cell.ts";
import type { Parameter } from "../Parameter.ts";
import type { BulkUpdateStatement } from "./BulkUpdateStatement.ts";

export class BulkUpdateStatementTextRenderer {
  render = (statement: BulkUpdateStatement): string => {
    const assigns = statement.set.map((column) => `${column} = data.${column}`).join(", ");
    return `UPDATE ${statement.table} SET ${assigns} FROM ${this.source(statement)} WHERE ${this.on(statement)}`;
  };

  protected on = (statement: BulkUpdateStatement): string =>
    statement.ids.map((column) => `${statement.table}.${column} = data.${column}`).join(" AND ");

  protected source = (statement: BulkUpdateStatement): string => {
    const columns = [...statement.set, ...statement.ids];
    const selects = statement.rows.map((row, index) => {
      const byName = new Map<string, Cell>();
      for (const field of [...row.fields, ...row.ids] as Parameter[])
        byName.set(field.name, (field.value ?? null) as Cell);
      return `SELECT ${columns
        .map((column) => this.cell(column, index, byName.get(column) ?? null))
        .join(", ")}`;
    });
    return `(${selects.join(" UNION ALL ")}) AS data`;
  };

  protected cell = (column: string, index: number, _value: Cell): string =>
    `$${column}__${index} AS ${column}`;
}
