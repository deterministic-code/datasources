using Datasources;

namespace Datasources.Postgres;

public sealed class PostgresCommand : StubCommand
{
    public PostgresCommand(IConnection connection) : base("postgres", connection) { }
}
