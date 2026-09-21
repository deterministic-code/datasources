namespace Datasources.Postgres;

public sealed record PostgresColumn(string Name, string Type, bool Id = false);
