using Datasources;

namespace Datasources.Sqlite;

public sealed record SqliteExecuteResult(int RowsAffected, Value Result);
