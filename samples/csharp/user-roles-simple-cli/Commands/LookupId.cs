using Datasources;

namespace UserRolesSimpleCli;

public static class LookupId
{
    public static async Task<int> Of(ICommand command, string table, string column, string value)
    {
        var where = new Where(new Datasources.Operator.Equals(column, value));
        var reader = await command.ExecuteReader(new QueryStatement(new() { Table = table, Fields = ["id"], Where = where }), where.Parameters);
        return await reader.Next() && reader.Current is { } row
            ? row["id"] switch
            {
                Value.Int(var number) => (int)number,
                _ => throw new InvalidOperationException($"{table} '{value}' not found")
            }
            : throw new InvalidOperationException($"{table} '{value}' not found");
    }
}
