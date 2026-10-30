using Datasources;

namespace Datasources.Postgres;

internal sealed class PostgresDropTableStatementRenderer : DropTableStatementTextRenderer
{
    protected override string Quote(string name) => PostgresDriver.Quote(name);
}

internal sealed class PostgresCreateIndexStatementRenderer : CreateIndexStatementTextRenderer
{
    protected override string Quote(string name) => PostgresDriver.Quote(name);
}

internal sealed class PostgresDropIndexStatementRenderer : DropIndexStatementTextRenderer
{
    protected override string Quote(string name) => PostgresDriver.Quote(name);
}
