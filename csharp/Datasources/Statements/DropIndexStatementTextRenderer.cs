namespace Datasources;

public class DropIndexStatementTextRenderer
{
    public virtual string Render(DropIndexStatement statement) =>
        $"DROP INDEX IF EXISTS {Quote(statement.Name)}";

    protected virtual string Quote(string name) => name;
}
