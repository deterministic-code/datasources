using Datasources;

namespace Datasources.Sqlite;

internal sealed record SqliteExecuteResult(
    int RowsAffected,
    Value Result,
    IReadOnlyList<Parameter>? Parameters = null);
