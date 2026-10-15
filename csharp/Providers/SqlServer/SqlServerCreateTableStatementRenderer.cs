using Datasources;

namespace Datasources.SqlServer;

internal sealed class SqlServerCreateTableStatementRenderer : CreateTableStatementTextRenderer
{
    protected override string Identity(Column column) => column.Identity ? " IDENTITY(1,1)" : "";
}
