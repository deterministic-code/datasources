using Datasources;

namespace Datasources.Mysql;

internal sealed record MysqlExecuteResult(
    int RowsAffected,
    Value Result,
    IReadOnlyList<Parameter>? Parameters = null);
