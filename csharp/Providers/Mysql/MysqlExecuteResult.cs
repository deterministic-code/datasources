using Datasources;

namespace Datasources.Mysql;

public sealed record MysqlExecuteResult(
    int RowsAffected,
    Value Result,
    IReadOnlyList<Parameter>? Parameters = null);
