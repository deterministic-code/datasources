import type { IStatementVisitor } from "./IStatementVisitor.ts";
import { BulkDeleteStatement } from "./BulkDeleteStatement.ts";
import { BulkDeleteStatementTextRenderer } from "./BulkDeleteStatementTextRenderer.ts";
import { BulkInsertStatement } from "./BulkInsertStatement.ts";
import { BulkInsertStatementTextRenderer } from "./BulkInsertStatementTextRenderer.ts";
import { BulkUpdateStatement } from "./BulkUpdateStatement.ts";
import { BulkUpdateStatementTextRenderer } from "./BulkUpdateStatementTextRenderer.ts";
import { AlterTableStatement } from "./AlterTableStatement.ts";
import { AlterTableStatementTextRenderer } from "./AlterTableStatementTextRenderer.ts";
import { CreateIndexStatement } from "./CreateIndexStatement.ts";
import { CreateIndexStatementTextRenderer } from "./CreateIndexStatementTextRenderer.ts";
import { CreateRoutineStatement } from "./CreateRoutineStatement.ts";
import { CreateRoutineStatementTextRenderer } from "./CreateRoutineStatementTextRenderer.ts";
import { CreateTableStatement } from "./CreateTableStatement.ts";
import { CreateTableStatementTextRenderer } from "./CreateTableStatementTextRenderer.ts";
import { CreateTriggerStatement } from "./CreateTriggerStatement.ts";
import { CreateTriggerStatementTextRenderer } from "./CreateTriggerStatementTextRenderer.ts";
import { DeleteStatement } from "./DeleteStatement.ts";
import { DeleteStatementTextRenderer } from "./DeleteStatementTextRenderer.ts";
import { DropIndexStatement } from "./DropIndexStatement.ts";
import { DropIndexStatementTextRenderer } from "./DropIndexStatementTextRenderer.ts";
import { DropRoutineStatement } from "./DropRoutineStatement.ts";
import { DropRoutineStatementTextRenderer } from "./DropRoutineStatementTextRenderer.ts";
import { DropTableStatement } from "./DropTableStatement.ts";
import { DropTableStatementTextRenderer } from "./DropTableStatementTextRenderer.ts";
import { DropTriggerStatement } from "./DropTriggerStatement.ts";
import { DropTriggerStatementTextRenderer } from "./DropTriggerStatementTextRenderer.ts";
import { ExecuteStoredProcedureStatement } from "./ExecuteStoredProcedureStatement.ts";
import { ExecuteStoredProcedureStatementTextRenderer } from "./ExecuteStoredProcedureStatementTextRenderer.ts";
import { IdentityInsertStatement } from "./IdentityInsertStatement.ts";
import { IdentityInsertStatementTextRenderer } from "./IdentityInsertStatementTextRenderer.ts";
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
  | SelectInsertedIdStatement
  | BulkInsertStatement
  | BulkUpdateStatement
  | BulkDeleteStatement
  | DropTableStatement
  | AlterTableStatement
  | DropIndexStatement
  | DropTriggerStatement
  | CreateTriggerStatement
  | IdentityInsertStatement
  | CreateRoutineStatement
  | DropRoutineStatement;

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
  visit(statement: BulkInsertStatement): string;
  visit(statement: BulkUpdateStatement): string;
  visit(statement: BulkDeleteStatement): string;
  visit(statement: DropTableStatement): string;
  visit(statement: AlterTableStatement): string;
  visit(statement: DropIndexStatement): string;
  visit(statement: DropTriggerStatement): string;
  visit(statement: CreateTriggerStatement): string;
  visit(statement: IdentityInsertStatement): string;
  visit(statement: CreateRoutineStatement): string;
  visit(statement: DropRoutineStatement): string;
  visit(statement: AnyStatement): string {
    return ({
      SelectStatement: () => this.visitSelect(statement as SelectStatement),
      InsertStatement: () => this.visitInsert(statement as InsertStatement),
      UpdateStatement: () => this.visitUpdate(statement as UpdateStatement),
      DeleteStatement: () => this.visitDelete(statement as DeleteStatement),
      CreateTableStatement: () => this.visitCreateTable(statement as CreateTableStatement),
      CreateIndexStatement: () => this.visitCreateIndex(statement as CreateIndexStatement),
      ExecuteStoredProcedureStatement: () =>
        this.visitExecuteStoredProcedure(statement as ExecuteStoredProcedureStatement),
      ReturnStatement: () => this.visitReturn(statement as ReturnStatement),
      BulkInsertStatement: () => this.visitBulkInsert(statement as BulkInsertStatement),
      BulkUpdateStatement: () => this.visitBulkUpdate(statement as BulkUpdateStatement),
      BulkDeleteStatement: () => this.visitBulkDelete(statement as BulkDeleteStatement),
      DropTableStatement: () => this.visitDropTable(statement as DropTableStatement),
      AlterTableStatement: () => this.visitAlterTable(statement as AlterTableStatement),
      DropIndexStatement: () => this.visitDropIndex(statement as DropIndexStatement),
      DropTriggerStatement: () => this.visitDropTrigger(statement as DropTriggerStatement),
      CreateTriggerStatement: () => this.visitCreateTrigger(statement as CreateTriggerStatement),
      IdentityInsertStatement: () => this.visitIdentityInsert(statement as IdentityInsertStatement),
      CreateRoutineStatement: () => this.visitCreateRoutine(statement as CreateRoutineStatement),
      DropRoutineStatement: () => this.visitDropRoutine(statement as DropRoutineStatement),
      SelectInsertedIdStatement: () =>
        this.visitSelectInsertedId(statement as SelectInsertedIdStatement),
    }[statement.constructor.name] ?? (() => this.visitSelectInsertedId(statement as SelectInsertedIdStatement)))();
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

  protected visitBulkInsert(statement: BulkInsertStatement): string {
    return new BulkInsertStatementTextRenderer().render(statement);
  }

  protected visitBulkUpdate(statement: BulkUpdateStatement): string {
    return new BulkUpdateStatementTextRenderer().render(statement);
  }

  protected visitBulkDelete(statement: BulkDeleteStatement): string {
    return new BulkDeleteStatementTextRenderer().render(statement);
  }

  protected visitDropTable(statement: DropTableStatement): string {
    return new DropTableStatementTextRenderer().render(statement);
  }

  protected visitAlterTable(statement: AlterTableStatement): string {
    return new AlterTableStatementTextRenderer().renderAlter(statement);
  }

  protected visitDropIndex(statement: DropIndexStatement): string {
    return new DropIndexStatementTextRenderer().render(statement);
  }

  protected visitDropTrigger(statement: DropTriggerStatement): string {
    return new DropTriggerStatementTextRenderer().render(statement);
  }

  protected visitCreateTrigger(statement: CreateTriggerStatement): string {
    return new CreateTriggerStatementTextRenderer().render(statement);
  }

  protected visitIdentityInsert(statement: IdentityInsertStatement): string {
    return new IdentityInsertStatementTextRenderer().render(statement);
  }

  protected visitCreateRoutine(statement: CreateRoutineStatement): string {
    return new CreateRoutineStatementTextRenderer().render(statement);
  }

  protected visitDropRoutine(statement: DropRoutineStatement): string {
    return new DropRoutineStatementTextRenderer().render(statement);
  }
}
