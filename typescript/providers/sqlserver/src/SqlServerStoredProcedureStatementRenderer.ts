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

const params = (list: readonly RoutineParameter[]): string =>
  list.map((p) => `  @${p.name} ${p.type}`).join(",\n");

const openjson = (columns: readonly RoutineParameter[]): string =>
  columns.map((c) => `${c.name} ${c.type} '$.${c.name}'`).join(", ");

export class SqlServerStoredProcedureStatementRenderer {
  renderCreate(statement: CreateCreateStoredProcedureStatement): string {
    return `CREATE OR ALTER PROCEDURE ${statement.name}
${params(statement.parameters)}
AS
BEGIN
  INSERT INTO ${statement.table} (${statement.parameters.map((p) => p.name).join(", ")})
  VALUES (${statement.parameters.map((p) => `@${p.name}`).join(", ")});
  SELECT CAST(SCOPE_IDENTITY() AS BIGINT) AS ${statement.key};
END;
GO`;
  }

  renderNestedRead(statement: CreateNestedReadStoredProcedureStatement): string {
    const select = DefaultStatementTextRenderer.instance.render(statement.select);
    const signature = statement.arguments
      .map((argument) => `  @${argument.parameter.name} ${argument.parameter.type}`)
      .join(",\n");
    return `CREATE OR ALTER PROCEDURE ${statement.name}
${signature}
AS
BEGIN
  ${select};
END;
GO`;
  }

  renderRead(statement: CreateReadStoredProcedureStatement): string {
    const cols = statement.columns.join(", ");
    if (statement.byProperties.length === 0) {
      return `CREATE OR ALTER PROCEDURE ${statement.name}
AS
BEGIN
  SELECT ${cols} FROM ${statement.table} ORDER BY ${statement.key};
END;
GO`;
    }
    const signature = statement.byProperties
      .map((c) => `@${c} ${statement.parameters.find((p) => p.name === c)?.type ?? statement.keyType}`)
      .join(", ");
    const pred = statement.byProperties.map((c) => `${c} = @${c}`).join(" AND ");
    return `CREATE OR ALTER PROCEDURE ${statement.name} ${signature}
AS
BEGIN
  SELECT ${cols} FROM ${statement.table} WHERE ${pred};
END;
GO`;
  }

  renderUpdate(statement: CreateUpdateStoredProcedureStatement): string {
    const sets = statement.writable.map((c) => `    ${c} = @${c}`);
    if (statement.auditUpdated !== undefined) {
      const auditParam = statement.parameters.find((p) => p.name.includes("new_updated"));
      if (auditParam) sets.push(`    ${statement.auditUpdated} = @${auditParam.name}`);
    }
    const by = statement.byProperties.length > 0 ? statement.byProperties : [statement.key];
    const pred = by.map((c) => `${c} = @${c}`).join(" AND ");
    return `CREATE OR ALTER PROCEDURE ${statement.name}
${params(statement.parameters)}
AS
BEGIN
  UPDATE ${statement.table}
  SET
${sets.join(",\n")}
  WHERE ${pred};
  SELECT @@ROWCOUNT AS affected;
END;
GO`;
  }

  renderDelete(statement: CreateDeleteStoredProcedureStatement): string {
    const by = statement.byProperties.length > 0 ? statement.byProperties : [statement.key];
    const signature = by
      .map((c) => `@${c} ${statement.parameters.find((p) => p.name === c)?.type ?? statement.keyType}`)
      .join(", ");
    const pred = by.map((c) => `${c} = @${c}`).join(" AND ");
    return `CREATE OR ALTER PROCEDURE ${statement.name} ${signature}
AS
BEGIN
  DELETE FROM ${statement.table} WHERE ${pred};
  SELECT @@ROWCOUNT AS affected;
END;
GO`;
  }

  renderBulkCreate(statement: CreateBulkCreateStoredProcedureStatement): string {
    const cols = statement.columns.map((c) => c.name).join(", ");
    return `CREATE OR ALTER PROCEDURE ${statement.name}
  @${statement.payload.name} NVARCHAR(MAX),
  @ids NVARCHAR(MAX) OUTPUT
AS
BEGIN
  INSERT INTO ${statement.table} (${cols})
  SELECT ${cols}
  FROM OPENJSON(@${statement.payload.name}) WITH (${openjson(statement.columns)});
  SET @ids = (SELECT ${statement.key} FROM ${statement.table} WHERE ${statement.key} >= SCOPE_IDENTITY() FOR JSON PATH);
END;
GO`;
  }

  renderBulkUpdate(statement: CreateBulkUpdateStoredProcedureStatement): string {
    const sets = statement.columns
      .filter((c) => c.name !== statement.key)
      .map((c) => `t.${c.name} = payload_rows.${c.name}`)
      .join(", ");
    return `CREATE OR ALTER PROCEDURE ${statement.name}
  @${statement.payload.name} NVARCHAR(MAX)
AS
BEGIN
  UPDATE t SET ${sets}
  FROM ${statement.table} t
  JOIN OPENJSON(@${statement.payload.name}) WITH (${openjson(statement.columns)}) AS payload_rows
    ON t.${statement.key} = payload_rows.${statement.key};
END;
GO`;
  }

  renderBulkDelete(statement: CreateBulkDeleteStoredProcedureStatement): string {
    return `CREATE OR ALTER PROCEDURE ${statement.name}
  @${statement.payload.name} NVARCHAR(MAX)
AS
BEGIN
  DELETE t FROM ${statement.table} t
  JOIN OPENJSON(@${statement.payload.name}) WITH (${statement.key} ${statement.keyType} '$.${statement.key}') AS payload_rows
    ON t.${statement.key} = payload_rows.${statement.key};
END;
GO`;
  }
}
