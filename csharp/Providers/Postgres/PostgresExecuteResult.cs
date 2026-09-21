using Datasources;

namespace Datasources.Postgres;

public sealed record PostgresExecuteResult(
    int RowsAffected,
    Value Result,
    IReadOnlyList<Parameter>? Parameters = null);
