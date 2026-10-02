import type { IStatementVisitor } from "./IStatementVisitor.ts";
import { CreateIndexStatement } from "./CreateIndexStatement.ts";
import { CreateIndexStatementTextRenderer } from "./CreateIndexStatementTextRenderer.ts";
import { CreateTableStatement } from "./CreateTableStatement.ts";
import { CreateTableStatementTextRenderer } from "./CreateTableStatementTextRenderer.ts";
import { DeleteStatement } from "./DeleteStatement.ts";
import { DeleteStatementTextRenderer } from "./DeleteStatementTextRenderer.ts";
import { ExecuteStoredProcedureStatement } from "./ExecuteStoredProcedureStatement.ts";
import { ExecuteStoredProcedureStatementTextRenderer } from "./ExecuteStoredProcedureStatementTextRenderer.ts";
import { InsertStatement } from "./InsertStatement.ts";
import { InsertStatementTextRenderer } from "./InsertStatementTextRenderer.ts";
import { ReturnStatement } from "./ReturnStatement.ts";
import { ReturnStatementTextRenderer } from "./ReturnStatementTextRenderer.ts";
import { SelectInsertedIdStatement } from "./SelectInsertedIdStatement.ts";
import { SelectInsertedIdStatementTextRenderer } from "./SelectInsertedIdStatementTextRenderer.ts";
import { SelectStatement } from "./SelectStatement.ts";
import { SelectStatementTextRenderer } from "./SelectStatementTextRenderer.ts";
import type { StatementTextRenderer } from "./StatementTextRenderer.ts";
import { UpdateStatement } from "./UpdateStatement.ts";
import { UpdateStatementTextRenderer } from "./UpdateStatementTextRenderer.ts";

type AnyStatement =
  | SelectStatement
  | InsertStatement
  | UpdateStatement
  | DeleteStatement
  | CreateTableStatement
  | CreateIndexStatement
  | ExecuteStoredProcedureStatement
  | ReturnStatement
  | SelectInsertedIdStatement;

export class StatementRenderer implements IStatementVisitor {
  constructor(protected readonly text: StatementTextRenderer) {}

  visit(statement: SelectStatement): string;
  visit(statement: InsertStatement): string;
  visit(statement: UpdateStatement): string;
  visit(statement: DeleteStatement): string;
  visit(statement: CreateTableStatement): string;
  visit(statement: CreateIndexStatement): string;
  visit(statement: ExecuteStoredProcedureStatement): string;
  visit(statement: ReturnStatement): string;
  visit(statement: SelectInsertedIdStatement): string;
  visit(statement: AnyStatement): string {
    if (statement instanceof SelectStatement) return this.visitSelect(statement);
    if (statement instanceof InsertStatement) return this.visitInsert(statement);
    if (statement instanceof UpdateStatement) return this.visitUpdate(statement);
    if (statement instanceof DeleteStatement) return this.visitDelete(statement);
    if (statement instanceof CreateTableStatement) return this.visitCreateTable(statement);
    if (statement instanceof CreateIndexStatement) return this.visitCreateIndex(statement);
    if (statement instanceof ExecuteStoredProcedureStatement)
      return this.visitExecuteStoredProcedure(statement);
    if (statement instanceof ReturnStatement) return this.visitReturn(statement);
    return this.visitSelectInsertedId(statement as SelectInsertedIdStatement);
  }

  protected visitSelect(statement: SelectStatement): string {
    return new SelectStatementTextRenderer(this.text).render(statement);
  }

  protected visitInsert(statement: InsertStatement): string {
    return new InsertStatementTextRenderer().render(statement);
  }

  protected visitUpdate(statement: UpdateStatement): string {
    return new UpdateStatementTextRenderer(this.text).render(statement);
  }

  protected visitDelete(statement: DeleteStatement): string {
    return new DeleteStatementTextRenderer(this.text).render(statement);
  }

  protected visitCreateTable(statement: CreateTableStatement): string {
    return new CreateTableStatementTextRenderer().render(statement);
  }

  protected visitCreateIndex(statement: CreateIndexStatement): string {
    return new CreateIndexStatementTextRenderer().render(statement);
  }

  protected visitExecuteStoredProcedure(statement: ExecuteStoredProcedureStatement): string {
    return new ExecuteStoredProcedureStatementTextRenderer().render(statement);
  }

  protected visitReturn(statement: ReturnStatement): string {
    return new ReturnStatementTextRenderer().render(statement);
  }

  protected visitSelectInsertedId(statement: SelectInsertedIdStatement): string {
    return new SelectInsertedIdStatementTextRenderer().render(statement);
  }
}
