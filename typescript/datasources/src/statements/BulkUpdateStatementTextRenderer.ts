import type { Cell } from "../converters/Cell.ts";
import type { Parameter } from "../Parameter.ts";
import type { BulkUpdateStatement } from "./BulkUpdateStatement.ts";

export class BulkUpdateStatementTextRenderer {
  render = (statement: BulkUpdateStatement): string => {
    const assigns = statement.set.map((column) => `${this.quote(column)} = data.${this.quote(column)}`).join(", ");
    return `UPDATE ${this.quote(statement.table)} SET ${assigns} FROM ${this.source(statement)} WHERE ${this.on(statement)}`;
  };

  protected quote(name: string): string {
    return name;
  }

  protected parameter(name: string): string {
    return `$${name}`;
  }

  protected on = (statement: BulkUpdateStatement): string =>
    statement.ids.map((column) => `${this.quote(statement.table)}.${this.quote(column)} = data.${this.quote(column)}`).join(" AND ");

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
    `${this.parameter(`${column}__${index}`)} AS ${this.quote(column)}`;
}
