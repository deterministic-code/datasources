namespace Datasources;

public class ForeignKeyStatementTextRenderer
{
    public virtual string Render(ForeignKeyStatement statement) =>
        throw new InvalidOperationException("ForeignKeyStatement is rendered by the SQL driver.");
}
