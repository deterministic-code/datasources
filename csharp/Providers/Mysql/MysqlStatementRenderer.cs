using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlStatementRenderer(StatementTextRenderer text) : StatementRenderer(text)
{
    public override string Visit(SelectInsertedIdStatement inserted) =>
        new MysqlSelectInsertedIdStatementTextRenderer().Render(inserted);

    public override string Visit(CreateTableStatement create) =>
        new MysqlCreateTableStatementRenderer().Render(create);

    public override string Visit(BulkUpdateStatement update) =>
        new MysqlBulkUpdateStatementTextRenderer().Render(update);
}
