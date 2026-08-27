using Datasources;

namespace Datasources.Postgres;

public sealed class PostgresConnection : StubConnection
{
    public PostgresConnection() : base("postgres") { }
}
