import type {
  CreateBulkCreateStoredProcedureStatement,
  CreateBulkDeleteStoredProcedureStatement,
  CreateBulkUpdateStoredProcedureStatement,
  CreateCreateStoredProcedureStatement,
  CreateDeleteStoredProcedureStatement,
  CreateReadStoredProcedureStatement,
  CreateUpdateStoredProcedureStatement,
  RoutineParameter,
} from "@datasources/core";

const params = (list: readonly RoutineParameter[]): string =>
  list.map((p) => `  ${p.name} ${p.type}`).join(",\n");

const recordset = (columns: readonly RoutineParameter[]): string =>
  columns.map((c) => `${c.name} ${c.type}`).join(", ");

export class PostgresStoredProcedureStatementRenderer {
  renderCreate(statement: CreateCreateStoredProcedureStatement): string {
    return `CREATE OR REPLACE FUNCTION ${statement.name}(
${params(statement.parameters)}
) RETURNS ${statement.keyType} LANGUAGE plpgsql AS $$
DECLARE
  new_id ${statement.keyType};
BEGIN
  INSERT INTO ${statement.table} (${statement.parameters.map((p) => p.name).join(", ")})
  VALUES (${statement.parameters.map((p) => p.name).join(", ")})
  RETURNING ${statement.key} INTO new_id;
  RETURN new_id;
END;
$$`;
  }

  renderRead(statement: CreateReadStoredProcedureStatement): string {
    const aliased = statement.columns.map((c) => `t.${c}`).join(", ");
    if (statement.byProperties.length === 0) {
      return `CREATE OR REPLACE FUNCTION ${statement.name}() RETURNS SETOF ${statement.table}
LANGUAGE sql AS $$ SELECT ${aliased} FROM ${statement.table} t ORDER BY t.${statement.key}; $$`;
    }
    const signature = statement.byProperties
      .map((c) => `${c} ${statement.parameters.find((p) => p.name === c)?.type ?? statement.keyType}`)
      .join(", ");
    const pred = statement.byProperties
      .map((c) => `t.${c} = ${statement.name}.${c}`)
      .join(" AND ");
    return `CREATE OR REPLACE FUNCTION ${statement.name}(${signature}) RETURNS SETOF ${statement.table}
LANGUAGE sql AS $$ SELECT ${aliased} FROM ${statement.table} t WHERE ${pred}; $$`;
  }

  renderUpdate(statement: CreateUpdateStoredProcedureStatement): string {
    const sets = statement.writable.map((c) => `    ${c} = ${statement.name}.${c}`);
    if (statement.auditUpdated !== undefined) {
      const auditParam = statement.parameters.find((p) => p.name.includes("new_updated"));
      if (auditParam) sets.push(`    ${statement.auditUpdated} = ${statement.name}.${auditParam.name}`);
    }
    const by = statement.byProperties.length > 0 ? statement.byProperties : [statement.key];
    const pred = by.map((c) => `t.${c} = ${statement.name}.${c}`).join(" AND ");
    return `CREATE OR REPLACE FUNCTION ${statement.name}(
${params(statement.parameters)}
) RETURNS INT LANGUAGE plpgsql AS $$
DECLARE
  affected INT;
BEGIN
  UPDATE ${statement.table} t
  SET
${sets.join(",\n")}
  WHERE ${pred};
  GET DIAGNOSTICS affected = ROW_COUNT;
  RETURN affected;
END;
$$`;
  }

  renderDelete(statement: CreateDeleteStoredProcedureStatement): string {
    const by = statement.byProperties.length > 0 ? statement.byProperties : [statement.key];
    const signature = by
      .map((c) => `${c} ${statement.parameters.find((p) => p.name === c)?.type ?? statement.keyType}`)
      .join(", ");
    const pred = by.map((c) => `t.${c} = ${statement.name}.${c}`).join(" AND ");
    return `CREATE OR REPLACE FUNCTION ${statement.name}(${signature}) RETURNS INT LANGUAGE plpgsql AS $$
DECLARE
  affected INT;
BEGIN
  DELETE FROM ${statement.table} t WHERE ${pred};
  GET DIAGNOSTICS affected = ROW_COUNT;
  RETURN affected;
END;
$$`;
  }

  renderBulkCreate(statement: CreateBulkCreateStoredProcedureStatement): string {
    const cols = statement.columns.map((c) => c.name).join(", ");
    return `CREATE OR REPLACE PROCEDURE ${statement.name}(${statement.payload.name} jsonb, INOUT ids jsonb DEFAULT NULL)
LANGUAGE plpgsql AS $$
BEGIN
  WITH inserted AS (
    INSERT INTO ${statement.table} (${cols})
    SELECT ${cols}
    FROM jsonb_to_recordset(${statement.payload.name}) AS x(${recordset(statement.columns)})
    RETURNING ${statement.key}
  )
  SELECT COALESCE(jsonb_agg(${statement.key}), '[]'::jsonb) INTO ids FROM inserted;
END;
$$`;
  }

  renderBulkUpdate(statement: CreateBulkUpdateStoredProcedureStatement): string {
    const sets = statement.columns
      .filter((c) => c.name !== statement.key)
      .map((c) => `${c.name} = x.${c.name}`)
      .join(", ");
    return `CREATE OR REPLACE PROCEDURE ${statement.name}(${statement.payload.name} jsonb)
LANGUAGE plpgsql AS $$
BEGIN
  UPDATE ${statement.table} t SET
    ${sets}
  FROM jsonb_to_recordset(${statement.payload.name}) AS x(${recordset(statement.columns)})
  WHERE t.${statement.key} = x.${statement.key};
END;
$$`;
  }

  renderBulkDelete(statement: CreateBulkDeleteStoredProcedureStatement): string {
    return `CREATE OR REPLACE PROCEDURE ${statement.name}(${statement.payload.name} jsonb)
LANGUAGE plpgsql AS $$
BEGIN
  DELETE FROM ${statement.table} WHERE ${statement.key} IN (SELECT ${statement.key} FROM jsonb_to_recordset(${statement.payload.name}) AS x(${statement.key} ${statement.keyType}));
END;
$$`;
  }
}
