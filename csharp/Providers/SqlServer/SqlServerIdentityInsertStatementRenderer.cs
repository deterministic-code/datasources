using Datasources;

namespace Datasources.SqlServer;

internal sealed class SqlServerIdentityInsertStatementRenderer : IdentityInsertStatementTextRenderer
{
    public override string Render(IdentityInsertStatement statement)
    {
        var inserts = base.Render(statement);
        return !statement.Identity || statement.Inserts.Count == 0
            ? inserts
            : $"SET IDENTITY_INSERT {statement.Inserts[0].Table} ON;\n{inserts}\nSET IDENTITY_INSERT {statement.Inserts[0].Table} OFF";
    }
}
