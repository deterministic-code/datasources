using Datasources;

namespace Datasources.Postgres;

internal sealed class PostgresStatementRenderer(StatementTextRenderer text) : StatementRenderer(text)
{
    public override string Visit(SelectInsertedIdStatement inserted) =>
        new PostgresSelectInsertedIdStatementTextRenderer().Render(inserted);

    public override string Visit(CreateTableStatement create) =>
        new PostgresCreateTableStatementRenderer().Render(create);
}
