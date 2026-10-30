import type {
  CreateBulkCreateStoredProcedureStatement,
  CreateBulkDeleteStoredProcedureStatement,
  CreateBulkUpdateStoredProcedureStatement,
  CreateCreateStoredProcedureStatement,
  CreateDeleteStoredProcedureStatement,
  CreateNestedReadStoredProcedureStatement,
  CreateReadStoredProcedureStatement,
  CreateUpdateStoredProcedureStatement,
  RoutineParameter,
} from "@deterministic-code/datasources";
import { DefaultStatementTextRenderer } from "@deterministic-code/datasources";
import { OracleDriver } from "./OracleDriver.ts";

const typeOf = (parameter: RoutineParameter): string => OracleDriver.nativeParam(parameter);

const params = (list: readonly RoutineParameter[]): string =>
  list.map((p) => `  ${p.name} IN ${typeOf(p)}`).join(",\n");

const jsonColumns = (columns: readonly RoutineParameter[]): string =>
  columns.map((c) => `${c.name} ${typeOf(c)} PATH '$.${c.name}'`).join(", ");

export class OracleStoredProcedureStatementRenderer {
  renderCreate(statement: CreateCreateStoredProcedureStatement): string {
    return `CREATE OR REPLACE PROCEDURE ${statement.name}(
${params(statement.parameters)}
) AS
BEGIN
  INSERT INTO ${statement.table} (${statement.parameters.map((p) => p.name).join(", ")})
  VALUES (${statement.parameters.map((p) => p.name).join(", ")});
END;`;
  }

  renderNestedRead(statement: CreateNestedReadStoredProcedureStatement): string {
    const select = DefaultStatementTextRenderer.instance.render(statement.select);
    const signature = statement.arguments
      .map((argument) => `${argument.parameter.name} IN ${typeOf(argument.parameter)}`)
      .join(", ");
    return `CREATE OR REPLACE PROCEDURE ${statement.name}(${signature}) AS
BEGIN
  ${select};
END;`;
  }

  renderRead(statement: CreateReadStoredProcedureStatement): string {
    const aliased = statement.columns.map((c) => `t.${c}`).join(", ");
    if (statement.byProperties.length === 0) {
      return `CREATE OR REPLACE PROCEDURE ${statement.name} AS
BEGIN
  SELECT ${aliased} FROM ${statement.table} t ORDER BY t.${statement.key};
END;`;
    }
    const signature = statement.byProperties
      .map((c) => `${c} IN ${typeOf(statement.parameters.find((p) => p.name === c) ?? { name: c, type: statement.keyType })}`)
      .join(", ");
    const pred = statement.byProperties.map((c) => `t.${c} = ${c}`).join(" AND ");
    return `CREATE OR REPLACE PROCEDURE ${statement.name}(${signature}) AS
BEGIN
  SELECT ${aliased} FROM ${statement.table} t WHERE ${pred};
END;`;
  }

  renderUpdate(statement: CreateUpdateStoredProcedureStatement): string {
    const sets = statement.writable.map((c) => `    t.${c} = ${c}`);
    if (statement.auditUpdated !== undefined) {
      const auditParam = statement.parameters.find((p) => p.name.includes("new_updated"));
      if (auditParam) sets.push(`    t.${statement.auditUpdated} = ${auditParam.name}`);
    }
    const by = statement.byProperties.length > 0 ? statement.byProperties : [statement.key];
    const pred = by.map((c) => `t.${c} = ${c}`).join(" AND ");
    return `CREATE OR REPLACE PROCEDURE ${statement.name}(
${params(statement.parameters)}
) AS
BEGIN
  UPDATE ${statement.table} t
  SET
${sets.join(",\n")}
  WHERE ${pred};
END;`;
  }

  renderDelete(statement: CreateDeleteStoredProcedureStatement): string {
    const by = statement.byProperties.length > 0 ? statement.byProperties : [statement.key];
    const signature = by
      .map((c) => `${c} IN ${typeOf(statement.parameters.find((p) => p.name === c) ?? { name: c, type: statement.keyType })}`)
      .join(", ");
    const pred = by.map((c) => `t.${c} = ${c}`).join(" AND ");
    return `CREATE OR REPLACE PROCEDURE ${statement.name}(${signature}) AS
BEGIN
  DELETE FROM ${statement.table} t WHERE ${pred};
END;`;
  }

  renderBulkCreate(statement: CreateBulkCreateStoredProcedureStatement): string {
    const cols = statement.columns.map((c) => c.name).join(", ");
    return `CREATE OR REPLACE PROCEDURE ${statement.name}(${statement.payload.name} IN CLOB) AS
BEGIN
  INSERT INTO ${statement.table} (${cols})
  SELECT ${cols}
  FROM JSON_TABLE(${statement.payload.name}, '$[*]' COLUMNS (${jsonColumns(statement.columns)}));
END;`;
  }

  renderBulkUpdate(statement: CreateBulkUpdateStoredProcedureStatement): string {
    const sets = statement.columns
      .filter((c) => c.name !== statement.key)
      .map((c) => `t.${c.name} = payload_rows.${c.name}`)
      .join(", ");
    return `CREATE OR REPLACE PROCEDURE ${statement.name}(${statement.payload.name} IN CLOB) AS
BEGIN
  MERGE INTO ${statement.table} t
  USING (
    SELECT * FROM JSON_TABLE(${statement.payload.name}, '$[*]' COLUMNS (${jsonColumns(statement.columns)}))
  ) payload_rows ON (t.${statement.key} = payload_rows.${statement.key})
  WHEN MATCHED THEN UPDATE SET ${sets};
END;`;
  }

  renderBulkDelete(statement: CreateBulkDeleteStoredProcedureStatement): string {
    return `CREATE OR REPLACE PROCEDURE ${statement.name}(${statement.payload.name} IN CLOB) AS
BEGIN
  DELETE FROM ${statement.table} t
  WHERE t.${statement.key} IN (
    SELECT ${statement.key} FROM JSON_TABLE(${statement.payload.name}, '$[*]' COLUMNS (${statement.key} ${OracleDriver.nativeParam({ name: statement.key, type: statement.keyType })} PATH '$.${statement.key}'))
  );
END;`;
  }
}
