namespace Datasources.Sqlite;

public sealed record SqliteExecuteResult(int RowsAffected, object? Result);
