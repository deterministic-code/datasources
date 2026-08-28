namespace Datasources.Mysql;

public sealed record MysqlExecuteResult(int RowsAffected, object? Result);
