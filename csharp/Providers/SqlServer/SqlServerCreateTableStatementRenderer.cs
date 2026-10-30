using Datasources;

namespace Datasources.SqlServer;

internal sealed class SqlServerCreateTableStatementRenderer : CreateTableStatementTextRenderer
{
    protected override string Quote(string name) => SqlServerDriver.Quote(name);

    protected override string Type(Column column) => SqlServerDriver.NativeType(column);

    protected override string Default(Column column) =>
        column.Default is null
            ? ""
            : column.Default.Name is null
                ? $" DEFAULT {SqlServerDriver.NativeDefault(column)}"
                : $" CONSTRAINT {Quote(column.Default.Name)} DEFAULT {SqlServerDriver.NativeDefault(column)}";

    protected override string Identity(Column column) => column.Identity ? " IDENTITY(1,1)" : "";
}
