import { CreateTriggerStatementTextRenderer, type CreateTriggerStatement } from "@deterministic-code/datasources";
import { PostgresDriver } from "./PostgresDriver.ts";

export class PostgresCreateTriggerStatementRenderer extends CreateTriggerStatementTextRenderer {
  override render(statement: CreateTriggerStatement): string {
    const updated = PostgresDriver.quote(statement.updatedColumn);
    return `-- Shared updated_at function
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS $$
BEGIN
  NEW.${updated} = ${PostgresDriver.utcNow()};
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER ${PostgresDriver.quote(statement.name)} BEFORE UPDATE ON ${PostgresDriver.quote(statement.table)} FOR EACH ROW EXECUTE FUNCTION set_updated_at()`;
  }
}
