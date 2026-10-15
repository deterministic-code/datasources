import {
  CreateRoutineStatementTextRenderer,
  type CreateRoutineStatement,
  type RoutineParameter,
} from "@datasources/core";

const params = (list: readonly RoutineParameter[]): string =>
  list.map((p) => `  @${p.name} ${p.type}`).join(",\n");

export class SqlServerCreateRoutineStatementRenderer extends CreateRoutineStatementTextRenderer {
  override render(statement: CreateRoutineStatement): string {
    const cols = statement.columns.join(", ");
    return {
      create: () => `CREATE OR ALTER PROCEDURE ${statement.name}
${params(statement.parameters)}
AS
BEGIN
  INSERT INTO ${statement.table} (${statement.parameters.map((p) => p.name).join(", ")})
  VALUES (${statement.parameters.map((p) => `@${p.name}`).join(", ")});
  SELECT CAST(SCOPE_IDENTITY() AS BIGINT) AS id;
END;
GO`,
      findOne: () => `CREATE OR ALTER PROCEDURE ${statement.name} @${statement.key} ${statement.keyType}
AS
BEGIN
  SELECT ${cols} FROM ${statement.table} WHERE ${statement.key} = @${statement.key};
END;
GO`,
      findAll: () => `CREATE OR ALTER PROCEDURE ${statement.name}
AS
BEGIN
  SELECT ${cols} FROM ${statement.table} ORDER BY ${statement.key};
END;
GO`,
      findBy: () => {
        const by = statement.byField ?? statement.key;
        const type = statement.parameters[0]?.type ?? statement.keyType;
        return `CREATE OR ALTER PROCEDURE ${statement.name} @${by} ${type}
AS
BEGIN
  SELECT ${cols} FROM ${statement.table} WHERE ${by} = @${by};
END;
GO`;
      },
      update: () => {
        const sets = statement.writable.map((c) => `    ${c} = @${c}`);
        if (statement.auditUpdated !== undefined) {
          const auditParam = statement.parameters.find((p) => p.name.includes("new_updated"));
          if (auditParam) sets.push(`    ${statement.auditUpdated} = @${auditParam.name}`);
        }
        const by = statement.byField ?? statement.key;
        return `CREATE OR ALTER PROCEDURE ${statement.name}
${params(statement.parameters)}
AS
BEGIN
  UPDATE ${statement.table}
  SET
${sets.join(",\n")}
  WHERE ${by} = @${by};
  SELECT @@ROWCOUNT AS affected;
END;
GO`;
      },
      delete: () => `CREATE OR ALTER PROCEDURE ${statement.name} @${statement.key} ${statement.keyType}
AS
BEGIN
  DELETE FROM ${statement.table} WHERE ${statement.key} = @${statement.key};
  SELECT @@ROWCOUNT AS affected;
END;
GO`,
    }[statement.kind]();
  }
}
