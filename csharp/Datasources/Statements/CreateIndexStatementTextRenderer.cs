namespace Datasources;

public class CreateIndexStatementTextRenderer
{
    public virtual string Render(CreateIndexStatement statement) =>
        $"CREATE {(statement.Unique ? "UNIQUE " : "")}INDEX {Quote(statement.Name)} ON {Quote(statement.Table)} ({string.Join(", ", statement.Columns.Select(Quote))})";

    protected virtual string Quote(string name) => name;
}
