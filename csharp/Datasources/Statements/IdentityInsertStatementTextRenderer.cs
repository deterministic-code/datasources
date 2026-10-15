namespace Datasources;

public class IdentityInsertStatementTextRenderer
{
    public virtual string Render(IdentityInsertStatement statement) =>
        string.Join("\n", statement.Inserts.Select(Insert));

    protected virtual string Insert(InsertStatement statement) =>
        $"INSERT INTO {statement.Table} ({string.Join(", ", statement.Fields.Select(field => field.Name))}) VALUES ({string.Join(", ", statement.Fields.Select(field => $"{field.Value}"))})";
}
