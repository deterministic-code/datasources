using System.Globalization;

namespace Datasources;

public class IdentityInsertStatementTextRenderer
{
    public virtual string Render(IdentityInsertStatement statement) =>
        string.Join("\n", statement.Inserts.Select(Insert));

    protected virtual string Insert(InsertStatement statement) =>
        $"INSERT INTO {statement.Table} ({string.Join(", ", statement.Fields.Select(field => field.Name))}) VALUES ({string.Join(", ", statement.Fields.Select(Sql))});";

    static string Sql(Parameter field) => field.Value switch
    {
        Value.Null => "NULL",
        Value.Text(var text) => text,
        Value.Int(var number) => number.ToString(CultureInfo.InvariantCulture),
        Value.Bool(true) => "TRUE",
        Value.Bool(false) => "FALSE",
        Value.Float(var number) => number.ToString(CultureInfo.InvariantCulture),
        _ => field.Value.ToObject()?.ToString() ?? "NULL",
    };
}
