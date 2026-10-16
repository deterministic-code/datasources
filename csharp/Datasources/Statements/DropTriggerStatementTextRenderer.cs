namespace Datasources;

public class DropTriggerStatementTextRenderer
{
    public virtual string Render(DropTriggerStatement statement) =>
        $"DROP TRIGGER IF EXISTS {Quote(statement.Name)}";

    protected virtual string Quote(string name) => name;
}
