using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlBulkUpdateStatementTextRenderer : BulkUpdateStatementTextRenderer
{
    public override string Render(BulkUpdateStatement statement)
    {
        var assigns = string.Join(", ", statement.Set.Select(column => $"{statement.Table}.{column} = data.{column}"));
        return $"UPDATE {statement.Table} JOIN {Source(statement)} ON {On(statement)} SET {assigns}";
    }
}
