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
import { MysqlDriver } from "./MysqlDriver.ts";

const typeOf = (parameter: RoutineParameter): string => MysqlDriver.nativeParam(parameter);

const params = (list: readonly RoutineParameter[]): string =>
  list.map((p) => `  IN ${p.name} ${typeOf(p)}`).join(",\n");

const where = (columns: readonly string[]): string =>
  columns.map((c) => `t.${c} = ${c}`).join(" AND ");

const jsonColumns = (columns: readonly RoutineParameter[]): string =>
  columns.map((c) => `${c.name} ${typeOf(c)} PATH '$.${c.name}'`).join(", ");

const procedure = (name: string, signature: string, body: string): string =>
  `DROP PROCEDURE IF EXISTS ${name};
GO
CREATE PROCEDURE ${name}(${signature})
BEGIN
${body}
END;
GO`;

export class MysqlStoredProcedureStatementRenderer {
  renderCreate(statement: CreateCreateStoredProcedureStatement): string {
    const generated = statement.generatedKey;
    const insertCols = generated ? statement.columns : statement.parameters.map((p) => p.name);
    const vals = generated
      ? statement.columns.map((c) => (c === statement.key ? "new_id" : c))
      : statement.parameters.map((p) => p.name);
    const body = generated
      ? `  DECLARE new_id VARCHAR(36);
  SET new_id = UUID();
  INSERT INTO ${MysqlDriver.quote(statement.table)} (${insertCols.join(", ")})
  VALUES (${vals.join(", ")});
  SELECT new_id AS ${statement.key};`
      : `  INSERT INTO ${MysqlDriver.quote(statement.table)} (${insertCols.join(", ")})
  VALUES (${vals.join(", ")});
  SELECT LAST_INSERT_ID() AS ${statement.key};`;
    return procedure(statement.name, `\n${params(statement.parameters)}\n`, body);
  }

  renderNestedRead(statement: CreateNestedReadStoredProcedureStatement): string {
    const select = DefaultStatementTextRenderer.instance.render(statement.select);
    const signature = statement.arguments
      .map((argument) => `IN ${argument.parameter.name} ${typeOf(argument.parameter)}`)
      .join(", ");
    return procedure(statement.name, signature, `  ${select};`);
  }

  renderRead(statement: CreateReadStoredProcedureStatement): string {
    const aliased = statement.columns.map((c) => `t.${c}`).join(", ");
    if (statement.byProperties.length === 0) {
      return procedure(
        statement.name,
        "",
        `  SELECT ${aliased} FROM ${MysqlDriver.quote(statement.table)} t ORDER BY t.${statement.key};`,
      );
    }
    const signature = statement.byProperties
      .map((c) => {
        const type = typeOf(statement.parameters.find((p) => p.name === c) ?? { name: c, type: statement.keyType });
        return `IN ${c} ${type}`;
      })
      .join(", ");
    return procedure(
      statement.name,
      signature,
      `  SELECT ${aliased} FROM ${MysqlDriver.quote(statement.table)} t WHERE ${where(statement.byProperties)};`,
    );
  }

  renderUpdate(statement: CreateUpdateStoredProcedureStatement): string {
    const sets = statement.writable.map((c) => `    t.${c}     = ${c}`);
    if (statement.auditUpdated !== undefined) {
      const auditParam = statement.parameters.find((p) => p.name.includes("new_updated"));
      if (auditParam) sets.push(`    t.${statement.auditUpdated} = ${auditParam.name}`);
    }
    const by = statement.byProperties.length > 0 ? statement.byProperties : [statement.key];
    return procedure(
      statement.name,
      `\n${params(statement.parameters)}\n`,
      `  UPDATE ${MysqlDriver.quote(statement.table)} t
  SET
${sets.join(",\n")}
  WHERE ${where(by)};
  SELECT ROW_COUNT() AS affected;`,
    );
  }

  renderDelete(statement: CreateDeleteStoredProcedureStatement): string {
    const by = statement.byProperties.length > 0 ? statement.byProperties : [statement.key];
    const signature = by
      .map((c) => {
        const type = typeOf(statement.parameters.find((p) => p.name === c) ?? { name: c, type: statement.keyType });
        return `IN ${c} ${type}`;
      })
      .join(", ");
    return procedure(
      statement.name,
      signature,
      `  DELETE t FROM ${MysqlDriver.quote(statement.table)} t WHERE ${where(by)};
  SELECT ROW_COUNT() AS affected;`,
    );
  }

  renderBulkCreate(statement: CreateBulkCreateStoredProcedureStatement): string {
    const cols = statement.columns.map((c) => c.name).join(", ");
    return procedure(
      statement.name,
      `IN ${statement.payload.name} ${typeOf(statement.payload)}, OUT ids JSON`,
      `  INSERT INTO ${MysqlDriver.quote(statement.table)} (${cols})
  SELECT ${cols}
  FROM JSON_TABLE(${statement.payload.name}, '$[*]' COLUMNS (
    ${jsonColumns(statement.columns)}
  )) AS payload_rows;
  GET DIAGNOSTICS @n = ROW_COUNT;
  SET @first = LAST_INSERT_ID();
  SET ids = JSON_ARRAY();
  SET @i = 0;
  WHILE @i < @n DO
    SET ids = JSON_ARRAY_APPEND(ids, '$', @first + @i);
    SET @i = @i + 1;
  END WHILE;`,
    );
  }

  renderBulkUpdate(statement: CreateBulkUpdateStoredProcedureStatement): string {
    const sets = statement.columns
      .filter((c) => c.name !== statement.key)
      .map((c) => `c.${c.name} = payload_rows.${c.name}`)
      .join(", ");
    return procedure(
      statement.name,
      `IN ${statement.payload.name} ${typeOf(statement.payload)}`,
      `  UPDATE ${MysqlDriver.quote(statement.table)} c
  JOIN JSON_TABLE(${statement.payload.name}, '$[*]' COLUMNS (
    ${jsonColumns(statement.columns)}
  )) AS payload_rows ON c.${statement.key} = payload_rows.${statement.key}
  SET ${sets};`,
    );
  }

  renderBulkDelete(statement: CreateBulkDeleteStoredProcedureStatement): string {
    const key = statement.columns.find((c) => c.name === statement.key) ?? {
      name: statement.key,
      type: statement.keyType,
    };
    return procedure(
      statement.name,
      `IN ${statement.payload.name} ${typeOf(statement.payload)}`,
      `  DELETE c FROM ${MysqlDriver.quote(statement.table)} c JOIN JSON_TABLE(${statement.payload.name}, '$[*]' COLUMNS (${key.name} ${typeOf(key)} PATH '$.${key.name}')) AS payload_rows ON c.${statement.key} = payload_rows.${statement.key};`,
    );
  }
}
