namespace Datasources.Postgres;

public sealed record PostgresExecuteResult(int RowsAffected, object? Result);
