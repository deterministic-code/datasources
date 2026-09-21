namespace Datasources.Sqlite;

public sealed record SqliteColumn(string Name, string Type, bool Id = false);
