import {
  DeleteStatement,
  InsertStatement,
  SelectStatement,
  StatementRenderer,
  UpdateStatement,
  type SelectInsertedIdStatement,
} from "@deterministic-code/datasources";

export class MemoryStatementRenderer extends StatementRenderer {
  protected override visitInsert(statement: InsertStatement): string {
    return statement.table ? super.visitInsert(statement) : "insert";
  }

  protected override visitUpdate(statement: UpdateStatement): string {
    if (statement.table) return super.visitUpdate(statement);
    const where = statement.where ? this.text.render(statement.where) : "";
    if (!where.trim()) return "update";
    return where.trimStart().startsWith("{")
      ? `update ${where}`
      : super.visitUpdate(new UpdateStatement("mem", statement.fields, statement.where));
  }

  protected override visitDelete(statement: DeleteStatement): string {
    if (statement.table) return super.visitDelete(statement);
    const where = statement.where ? this.text.render(statement.where) : "";
    if (!where.trim()) return "delete";
    return where.trimStart().startsWith("{")
      ? `delete ${where}`
      : super.visitDelete(new DeleteStatement("mem", statement.where));
  }

  protected override visitSelect(statement: SelectStatement): string {
    const where = statement.where ? this.text.render(statement.where) : "";
    if (where.trimStart().startsWith("{")) return where;
    return statement.table ? super.visitSelect(statement) : "";
  }

  protected override visitSelectInsertedId(statement: SelectInsertedIdStatement): string {
    return JSON.stringify({ __inserted: statement.table, __column: statement.column });
  }
}
