namespace Datasources.Sqlite;

public sealed class SqliteConnectionOptions
{
    public string? Filename { get; set; }
    public ISqliteDriver? Driver { get; set; }
}
