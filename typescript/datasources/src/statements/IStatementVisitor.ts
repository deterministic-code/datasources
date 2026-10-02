import type { CreateIndexStatement } from "./CreateIndexStatement.ts";
import type { CreateTableStatement } from "./CreateTableStatement.ts";
import type { DeleteStatement } from "./DeleteStatement.ts";
import type { ExecuteStoredProcedureStatement } from "./ExecuteStoredProcedureStatement.ts";
import type { InsertStatement } from "./InsertStatement.ts";
import type { ReturnStatement } from "./ReturnStatement.ts";
import type { SelectInsertedIdStatement } from "./SelectInsertedIdStatement.ts";
import type { SelectStatement } from "./SelectStatement.ts";
import type { UpdateStatement } from "./UpdateStatement.ts";

export interface IStatementVisitor {
  visit(statement: SelectStatement): string;
  visit(statement: InsertStatement): string;
  visit(statement: UpdateStatement): string;
  visit(statement: DeleteStatement): string;
  visit(statement: CreateTableStatement): string;
  visit(statement: CreateIndexStatement): string;
  visit(statement: ExecuteStoredProcedureStatement): string;
  visit(statement: ReturnStatement): string;
  visit(statement: SelectInsertedIdStatement): string;
}
