import {
  CreateRoutineStatementTextRenderer,
  type CreateRoutineStatement,
  type RoutineParameter,
} from "@datasources/core";

const params = (list: readonly RoutineParameter[]): string =>
  list.map((p) => `  ${p.name} ${p.type}`).join(",\n");

export class PostgresCreateRoutineStatementRenderer extends CreateRoutineStatementTextRenderer {
  override render(statement: CreateRoutineStatement): string {
    const cols = statement.columns.join(", ");
    const aliased = statement.columns.map((c) => `t.${c}`).join(", ");
    return {
      create: () => `CREATE OR REPLACE FUNCTION ${statement.name}(
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
$$`,
      findOne: () =>
        `CREATE OR REPLACE FUNCTION ${statement.name}(${statement.key} ${statement.keyType}) RETURNS SETOF ${statement.table}
LANGUAGE sql AS $$ SELECT ${aliased} FROM ${statement.table} t WHERE t.${statement.key} = ${statement.name}.${statement.key}; $$`,
      findAll: () =>
        `CREATE OR REPLACE FUNCTION ${statement.name}() RETURNS SETOF ${statement.table}
LANGUAGE sql AS $$ SELECT ${aliased} FROM ${statement.table} t ORDER BY t.${statement.key}; $$`,
      findBy: () => {
        const by = statement.byField ?? statement.key;
        const type = statement.parameters[0]?.type ?? statement.keyType;
        return `CREATE OR REPLACE FUNCTION ${statement.name}(${by} ${type}) RETURNS SETOF ${statement.table}
LANGUAGE sql AS $$ SELECT ${aliased} FROM ${statement.table} t WHERE t.${by} = ${statement.name}.${by}; $$`;
      },
      update: () => {
        const sets = statement.writable.map((c) => `    ${c} = ${statement.name}.${c}`);
        const audit = statement.auditUpdated;
        const auditParam = statement.parameters.find((p) => p.name.includes("new_updated"));
        if (audit !== undefined && auditParam !== undefined) {
          sets.push(`    ${audit} = ${statement.name}.${auditParam.name}`);
        }
        const by = statement.byField ?? statement.key;
        return `CREATE OR REPLACE FUNCTION ${statement.name}(
${params(statement.parameters)}
) RETURNS INT LANGUAGE plpgsql AS $$
DECLARE
  affected INT;
BEGIN
  UPDATE ${statement.table} t
  SET
${sets.join(",\n")}
  WHERE t.${by} = ${statement.name}.${by};
  GET DIAGNOSTICS affected = ROW_COUNT;
  RETURN affected;
END;
$$`;
      },
      delete: () =>
        `CREATE OR REPLACE FUNCTION ${statement.name}(${statement.key} ${statement.keyType}) RETURNS INT LANGUAGE plpgsql AS $$
DECLARE
  affected INT;
BEGIN
  DELETE FROM ${statement.table} t WHERE t.${statement.key} = ${statement.name}.${statement.key};
  GET DIAGNOSTICS affected = ROW_COUNT;
  RETURN affected;
END;
$$`,
    }[statement.kind]();
  }
}
