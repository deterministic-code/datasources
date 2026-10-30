using Datasources;

namespace Datasources.Postgres;

internal sealed class PostgresCreateTriggerStatementRenderer : CreateTriggerStatementTextRenderer
{
    public override string Render(CreateTriggerStatement statement)
    {
        var updated = PostgresDriver.Quote(statement.UpdatedColumn);
        return $"""
            -- Shared updated_at function
            CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS $$
            BEGIN
              NEW.{updated} = {PostgresDriver.UtcNow()};
              RETURN NEW;
            END;
            $$ LANGUAGE plpgsql;
            CREATE TRIGGER {PostgresDriver.Quote(statement.Name)} BEFORE UPDATE ON {PostgresDriver.Quote(statement.Table)} FOR EACH ROW EXECUTE FUNCTION set_updated_at()
            """;
    }
}
