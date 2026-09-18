namespace Datasources;

public sealed record BuiltRepository<T>(
    IConnection Connection,
    TypeConverter<T, Row> RowConverter,
    StatementMapper<T> StatementMapper,
    RepositoryStatements Statements,
    IReadOnlyList<string> Ids,
    bool AssignIdentityOnCreate) where T : new();
