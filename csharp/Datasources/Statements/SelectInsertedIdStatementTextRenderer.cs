namespace Datasources;

public class SelectInsertedIdStatementTextRenderer
{
    public virtual string Render(SelectInsertedIdStatement statement) =>
        throw new InvalidOperationException("provider must implement SelectInsertIdStatement");
}
