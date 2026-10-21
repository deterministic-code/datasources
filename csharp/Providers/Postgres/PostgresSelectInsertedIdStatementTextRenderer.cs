using Datasources;

namespace Datasources.Postgres;

internal sealed class PostgresSelectInsertedIdStatementTextRenderer : SelectInsertedIdStatementTextRenderer
{
    public override string Render(SelectInsertedIdStatement statement) =>
        $"SELECT setval(pg_get_serial_sequence('{statement.Table}', '{statement.Column}'), COALESCE((SELECT MAX(\"{statement.Column}\") FROM {statement.Table}), 1)) AS {statement.Column}";
}
