import type { Value } from "../converters/Value.ts";
import type { Parameter } from "../Parameter.ts";
import type { IBulkInsertStatement } from "./IBulkInsertStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";

const lookup = (fields: readonly Parameter[]): Map<string, Value> => {
  const map = new Map<string, Value>();
  for (const field of fields) map.set(field.name, (field.value ?? null) as Value);
  return map;
};

export const flattenBulkParameters = (
  columns: readonly string[],
  rows: readonly Parameter[][],
): Parameter[] => {
  const parameters: Parameter[] = [];
  rows.forEach((row, index) => {
    const byName = lookup(row);
    for (const column of columns)
      parameters.push({ name: `${column}__${index}`, value: (byName.get(column) ?? null) as Parameter["value"] });
  });
  return parameters;
};

export class BulkInsertStatement implements IBulkInsertStatement {
  constructor(
    readonly table: string,
    readonly columns: readonly string[],
    readonly rows: readonly Parameter[][],
  ) {}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);

  get parameters(): Parameter[] {
    return flattenBulkParameters(this.columns, this.rows);
  }
}
