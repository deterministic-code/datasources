namespace Datasources;

public class DropTableStatementTextRenderer
{
    public virtual string Render(DropTableStatement statement) =>
        $"DROP TABLE IF EXISTS {Quote(statement.Table)}";

    protected virtual string Quote(string name) => name;
}
