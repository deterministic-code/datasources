using Datasources;

namespace Datasources.Postgres;

public sealed class PostgresDatasource : IDatasourceFactory
{
    public static readonly PostgresDatasource Factory = new();
    public string Provider => "postgres";
    public IDatasource Create(object? options = null) => new StubDatasource("postgres");
}
