namespace Datasources;

public sealed record BuiltRepository<T>(
    IConnection Connection,
    TypeConverter<T, Row> RowConverter,
    StatementMapper<T> StatementMapper,
    RepositoryStatements Statements,
    IReadOnlyList<string> Ids,
    bool AssignIdentityOnAdd,
    IReadOnlyList<string> WriteColumns) where T : new();
