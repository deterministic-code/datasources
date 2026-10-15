using Datasources;

namespace Datasources.Postgres;

internal sealed class PostgresCreateTriggerStatementRenderer : CreateTriggerStatementTextRenderer
{
    public override string Render(CreateTriggerStatement statement) =>
        $"""
        -- Shared updated_at function
        CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS $$
        BEGIN
          NEW.{statement.UpdatedColumn} = NOW() AT TIME ZONE 'UTC';
          RETURN NEW;
        END;
        $$ LANGUAGE plpgsql;
        CREATE TRIGGER {statement.Name} BEFORE UPDATE ON {statement.Table} FOR EACH ROW EXECUTE FUNCTION set_updated_at()
        """;
}
