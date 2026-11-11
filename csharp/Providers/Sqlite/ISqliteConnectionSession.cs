namespace Datasources.Sqlite;

internal interface ISqliteConnectionSession
{
    Task Open(string filename, SqliteOptions options);
    Task Close();
}
