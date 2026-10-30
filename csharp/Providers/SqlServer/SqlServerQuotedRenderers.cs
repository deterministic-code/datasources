using Datasources;

namespace Datasources.SqlServer;

internal sealed class SqlServerDropTableStatementRenderer : DropTableStatementTextRenderer
{
    protected override string Quote(string name) => SqlServerDriver.Quote(name);
}

internal sealed class SqlServerCreateIndexStatementRenderer : CreateIndexStatementTextRenderer
{
    protected override string Quote(string name) => SqlServerDriver.Quote(name);
}

internal sealed class SqlServerDropTriggerStatementRenderer : DropTriggerStatementTextRenderer
{
    protected override string Quote(string name) => SqlServerDriver.Quote(name);
}
