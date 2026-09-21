namespace Datasources.Mysql;

public sealed record MysqlColumn(string Name, string Type, bool Id = false);
