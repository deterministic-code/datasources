using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlInsertStatementTextRenderer : InsertStatementTextRenderer
{
    protected override string Quote(string name) => MysqlDriver.Quote(name);
    protected override string Parameter(string name) => MysqlDriver.Parameter(name);
}

internal sealed class MysqlUpdateStatementTextRenderer(StatementTextRenderer text) : UpdateStatementTextRenderer(text)
{
    protected override string Quote(string name) => MysqlDriver.Quote(name);
    protected override string Parameter(string name) => MysqlDriver.Parameter(name);
}

internal sealed class MysqlDeleteStatementTextRenderer(StatementTextRenderer text) : DeleteStatementTextRenderer(text)
{
    protected override string Quote(string name) => MysqlDriver.Quote(name);
}

internal sealed class MysqlSelectStatementTextRenderer(StatementTextRenderer text) : SelectStatementTextRenderer(text)
{
    protected override string Quote(string name) => MysqlDriver.Quote(name);
}

internal sealed class MysqlBulkInsertStatementTextRenderer : BulkInsertStatementTextRenderer
{
    protected override string Quote(string name) => MysqlDriver.Quote(name);
    protected override string Parameter(string name) => MysqlDriver.Parameter(name);
}

internal sealed class MysqlBulkDeleteStatementTextRenderer : BulkDeleteStatementTextRenderer
{
    protected override string Quote(string name) => MysqlDriver.Quote(name);
    protected override string Parameter(string name) => MysqlDriver.Parameter(name);
}

internal sealed class MysqlExpressionRenderer(StatementTextRenderer text) : ExpressionRenderer(text)
{
    protected override string Quote(string name) => MysqlDriver.Quote(name);
    protected override string Parameter(string name) => MysqlDriver.Parameter(name);
}
