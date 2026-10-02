using Datasources;

namespace Datasources.Postgres;

internal sealed class PostgresSelectInsertedIdStatementTextRenderer : SelectInsertedIdStatementTextRenderer
{
    public override string Render(SelectInsertedIdStatement statement) =>
        $"SELECT lastval() AS {statement.Column}";
}
