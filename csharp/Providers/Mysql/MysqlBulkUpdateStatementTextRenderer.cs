using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlBulkUpdateStatementTextRenderer : BulkUpdateStatementTextRenderer
{
    public override string Render(BulkUpdateStatement statement)
    {
        var assigns = string.Join(", ", statement.Set.Select(column => $"{Quote(statement.Table)}.{Quote(column)} = data.{Quote(column)}"));
        return $"UPDATE {Quote(statement.Table)} JOIN {Source(statement)} ON {On(statement)} SET {assigns}";
    }

    protected override string Quote(string name) => MysqlDriver.Quote(name);

    protected override string Parameter(string name) => MysqlDriver.Parameter(name);
}
