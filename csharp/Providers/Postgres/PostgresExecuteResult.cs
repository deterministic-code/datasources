using Datasources;

namespace Datasources.Postgres;

internal sealed record PostgresExecuteResult(
    int RowsAffected,
    Value Result,
    IReadOnlyList<Parameter>? Parameters = null);
