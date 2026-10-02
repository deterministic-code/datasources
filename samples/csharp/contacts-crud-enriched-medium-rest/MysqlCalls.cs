using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public static class MysqlCalls
{
    public static IMapper<IInsertStatement, IStatement> Insert(string procedure) =>
        new TransformMapper<IInsertStatement, IStatement>(
            statement => new ExecuteStoredProcedureStatement(procedure, Fields(statement)),
            static statement => (IInsertStatement)statement);

    public static IMapper<IUpdateStatement, IStatement> Update(string procedure) =>
        new TransformMapper<IUpdateStatement, IStatement>(
            statement => new ExecuteStoredProcedureStatement(procedure, Fields(statement)),
            static statement => (IUpdateStatement)statement);

    public static IMapper<IDeleteStatement, IStatement> Delete(string procedure) =>
        new TransformMapper<IDeleteStatement, IStatement>(
            statement => new ExecuteStoredProcedureStatement(
                procedure,
                statement.Where?.Parameters ?? []),
            static statement => (IDeleteStatement)statement);

    static IReadOnlyList<Parameter> Fields(IInsertStatement statement) =>
        statement is InsertStatement insert ? insert.Fields : [];

    static IReadOnlyList<Parameter> Fields(IUpdateStatement statement) =>
        statement is UpdateStatement update
            ? UpdateStatement.ParameterList(update.Fields, update.Where)
            : statement.Where?.Parameters ?? [];
}
