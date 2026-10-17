namespace Datasources;

public interface IStatementVisitor
{
    string Visit(SelectStatement statement);
    string Visit(InsertStatement statement);
    string Visit(UpdateStatement statement);
    string Visit(DeleteStatement statement);
    string Visit(CreateTableStatement statement);
    string Visit(CreateIndexStatement statement);
    string Visit(ExecuteStoredProcedureStatement statement);
    string Visit(ReturnStatement statement);
    string Visit(SelectInsertedIdStatement statement);
    string Visit(BulkInsertStatement statement);
    string Visit(BulkUpdateStatement statement);
    string Visit(BulkDeleteStatement statement);
    string Visit(DropTableStatement statement);
    string Visit(AlterTableStatement statement);
    string Visit(DropIndexStatement statement);
    string Visit(DropTriggerStatement statement);
    string Visit(CreateTriggerStatement statement);
    string Visit(IdentityInsertStatement statement);
    string Visit(DropRoutineStatement statement);
    string Visit(CreateCreateStoredProcedureStatement statement);
    string Visit(CreateReadStoredProcedureStatement statement);
    string Visit(CreateUpdateStoredProcedureStatement statement);
    string Visit(CreateDeleteStoredProcedureStatement statement);
    string Visit(CreateBulkCreateStoredProcedureStatement statement);
    string Visit(CreateBulkUpdateStoredProcedureStatement statement);
    string Visit(CreateBulkDeleteStoredProcedureStatement statement);
}
