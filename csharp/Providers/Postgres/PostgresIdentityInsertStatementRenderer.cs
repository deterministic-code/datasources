using Datasources;

namespace Datasources.Postgres;

internal sealed class PostgresIdentityInsertStatementRenderer : IdentityInsertStatementTextRenderer
{
    public override string Render(IdentityInsertStatement statement)
    {
        var inserts = base.Render(statement);
        return !statement.Identity || statement.Inserts.Count == 0
            ? inserts
            : $"{inserts}\nSELECT setval(pg_get_serial_sequence('{statement.Table}', '{statement.IdColumn}'), (SELECT MAX(\"{statement.IdColumn}\") FROM {statement.Inserts[0].Table}))";
    }
}
