namespace Datasources.Sqlite;

internal interface ISqliteConnectionSession
{
    Task Open(string filename);
    Task Close();
}
