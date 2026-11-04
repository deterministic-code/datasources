namespace Datasources;

public class DeleteStatementTextRenderer(StatementTextRenderer text)
{
    public virtual string Render(DeleteStatement statement) =>
        statement.Where is null
            ? $"DELETE FROM {Quote(statement.Table)}"
            : $"DELETE FROM {Quote(statement.Table)} WHERE {text.Render(statement.Where)}";

    protected virtual string Quote(string name) => name;
}
