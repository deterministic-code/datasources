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
}
