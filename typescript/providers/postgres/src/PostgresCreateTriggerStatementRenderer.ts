import { CreateTriggerStatementTextRenderer, type CreateTriggerStatement } from "@deterministic-code/datasources";

export class PostgresCreateTriggerStatementRenderer extends CreateTriggerStatementTextRenderer {
  override render(statement: CreateTriggerStatement): string {
    return `-- Shared updated_at function
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS $$
BEGIN
  NEW.${statement.updatedColumn} = NOW() AT TIME ZONE 'UTC';
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER ${statement.name} BEFORE UPDATE ON ${statement.table} FOR EACH ROW EXECUTE FUNCTION set_updated_at()`;
  }
}
