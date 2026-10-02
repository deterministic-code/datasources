namespace Datasources;

public class DeleteStatementTextRenderer(StatementTextRenderer text)
{
    public virtual string Render(DeleteStatement statement) =>
        statement.Where is null
            ? $"DELETE FROM {statement.Table}"
            : $"DELETE FROM {statement.Table} WHERE {text.Render(statement.Where)}";
}
