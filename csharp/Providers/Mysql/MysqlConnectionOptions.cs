namespace Datasources.Mysql;

internal sealed class MysqlConnectionOptions
{
    public string? Url { get; set; }
    public IMysqlDriver? Driver { get; set; }
}
