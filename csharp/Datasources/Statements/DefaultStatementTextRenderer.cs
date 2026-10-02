namespace Datasources;

public sealed class DefaultStatementTextRenderer : StatementTextRenderer
{
    public static DefaultStatementTextRenderer Instance { get; } = new();

    public DefaultStatementTextRenderer() : base() { }
}
