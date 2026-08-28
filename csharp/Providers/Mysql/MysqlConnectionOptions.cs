namespace Datasources.Mysql;

public sealed class MysqlConnectionOptions
{
    public string? Url { get; set; }
    public IMysqlDriver? Driver { get; set; }
}
