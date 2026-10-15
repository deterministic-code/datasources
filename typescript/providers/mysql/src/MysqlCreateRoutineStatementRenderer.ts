import {
  CreateRoutineStatementTextRenderer,
  type CreateRoutineStatement,
  type RoutineParameter,
} from "@datasources/core";

const params = (list: readonly RoutineParameter[]): string =>
  list.map((p) => `  IN ${p.name} ${p.type}`).join(",\n");

export class MysqlCreateRoutineStatementRenderer extends CreateRoutineStatementTextRenderer {
  override render(statement: CreateRoutineStatement): string {
    const aliased = statement.columns.map((c) => `t.${c}`).join(", ");
    const cols = statement.parameters.map((p) => p.name).join(", ");
    return {
      create: () => {
        const generated = statement.generatedKey;
        const lastInsertId = !generated;
        const insertCols = generated ? statement.columns : statement.parameters.map((p) => p.name);
        const vals = generated
          ? statement.columns.map((c) => (c === statement.key ? "new_id" : c))
          : statement.parameters.map((p) => p.name);
        return `DROP PROCEDURE IF EXISTS ${statement.name};
GO
CREATE PROCEDURE ${statement.name}(
${params(statement.parameters)}
)
BEGIN
${generated ? `  DECLARE new_id VARCHAR(36);
  SET new_id = UUID();
  INSERT INTO ${statement.table} (${insertCols.join(", ")})
  VALUES (${vals.join(", ")});
  SELECT new_id AS ${statement.key};` : `  INSERT INTO ${statement.table} (${cols})
  VALUES (${vals.join(", ")});
${lastInsertId ? `  SELECT LAST_INSERT_ID() AS ${statement.key};` : `  SELECT t.${statement.key} AS ${statement.key} FROM ${statement.table} t WHERE t.uuid = uuid;`}`}
END;
GO`;
      },
      findOne: () => `DROP PROCEDURE IF EXISTS ${statement.name};
GO
CREATE PROCEDURE ${statement.name}(IN ${statement.key} ${statement.keyType})
BEGIN
  SELECT ${aliased} FROM ${statement.table} t WHERE t.${statement.key} = ${statement.key};
END;
GO`,
      findAll: () => `DROP PROCEDURE IF EXISTS ${statement.name};
GO
CREATE PROCEDURE ${statement.name}()
BEGIN
  SELECT ${aliased} FROM ${statement.table} t ORDER BY t.${statement.key};
END;
GO`,
      findBy: () => {
        const by = statement.byField ?? statement.key;
        const type = statement.parameters[0]?.type ?? statement.keyType;
        return `DROP PROCEDURE IF EXISTS ${statement.name};
GO
CREATE PROCEDURE ${statement.name}(IN ${by} ${type})
BEGIN
  SELECT ${aliased} FROM ${statement.table} t WHERE t.${by} = ${by};
END;
GO`;
      },
      update: () => {
        const sets = statement.writable.map((c) => `    t.${c}     = ${c}`);
        if (statement.auditUpdated !== undefined) {
          const auditParam = statement.parameters.find((p) => p.name.includes("new_updated"));
          if (auditParam) sets.push(`    t.${statement.auditUpdated} = ${auditParam.name}`);
        }
        const by = statement.byField ?? statement.key;
        return `DROP PROCEDURE IF EXISTS ${statement.name};
GO
CREATE PROCEDURE ${statement.name}(
${params(statement.parameters)}
)
BEGIN
  UPDATE ${statement.table} t
  SET
${sets.join(",\n")}
  WHERE t.${by} = ${by};
  SELECT ROW_COUNT() AS affected;
END;
GO`;
      },
      delete: () => `DROP PROCEDURE IF EXISTS ${statement.name};
GO
CREATE PROCEDURE ${statement.name}(IN ${statement.key} ${statement.keyType})
BEGIN
  DELETE t FROM ${statement.table} t WHERE t.${statement.key} = ${statement.key};
  SELECT ROW_COUNT() AS affected;
END;
GO`,
    }[statement.kind]();
  }
}
