import { CreateBulkCreateStoredProcedureStatement } from "./CreateBulkCreateStoredProcedureStatement.ts";
import { CreateBulkDeleteStoredProcedureStatement } from "./CreateBulkDeleteStoredProcedureStatement.ts";
import { CreateBulkUpdateStoredProcedureStatement } from "./CreateBulkUpdateStoredProcedureStatement.ts";
import { CreateCreateStoredProcedureStatement } from "./CreateCreateStoredProcedureStatement.ts";
import { CreateDeleteStoredProcedureStatement } from "./CreateDeleteStoredProcedureStatement.ts";
import { CreateReadStoredProcedureStatement } from "./CreateReadStoredProcedureStatement.ts";
import { CreateUpdateStoredProcedureStatement } from "./CreateUpdateStoredProcedureStatement.ts";
import type { IStatement } from "./IStatement.ts";
import type { ProcedureSource } from "./ProcedureSource.ts";
import type { RoutineParameter } from "./RoutineParameter.ts";

export const sqlProcedureStatementsOf = (source: ProcedureSource): IStatement[] => {
  const name = (stem: string): string => source.routineName(stem);
  const plural = source.plural;
  const payload: RoutineParameter = { name: "payload", type: "payload" };
  const statements: IStatement[] = [
    new CreateCreateStoredProcedureStatement(
      name(`create_${plural}`), source.table, source.columns, source.createParameters,
      source.key, source.keyType, source.writable, source.generatedKey, source.auditUpdated,
    ),
    new CreateReadStoredProcedureStatement(
      name(`read_${plural}`), source.table, source.columns, [],
      source.key, source.keyType, source.writable, [], source.generatedKey, source.auditUpdated,
    ),
  ];
  for (const group of source.byGroups) {
    const suffix = `_by_${group.logical.join("_")}`;
    statements.push(
      new CreateReadStoredProcedureStatement(
        name(`read_${plural}${suffix}`), source.table, source.columns, group.parameters,
        source.key, source.keyType, source.writable, group.columns, source.generatedKey, source.auditUpdated,
      ),
      new CreateUpdateStoredProcedureStatement(
        name(`update_${plural}${suffix}`), source.table, source.columns, group.updateParameters,
        source.key, source.keyType, source.writable, group.columns, source.generatedKey, source.auditUpdated,
      ),
      new CreateDeleteStoredProcedureStatement(
        name(`delete_${plural}${suffix}`), source.table, source.columns, group.parameters,
        source.key, source.keyType, source.writable, group.columns, source.generatedKey, source.auditUpdated,
      ),
    );
  }
  statements.push(
    new CreateBulkCreateStoredProcedureStatement(
      name(`bulk_create_${plural}`), source.table, source.createColumns, payload, source.key, source.keyType,
    ),
    new CreateBulkUpdateStoredProcedureStatement(
      name(`bulk_update_${plural}`), source.table, source.updateColumns, payload, source.key, source.keyType,
    ),
    new CreateBulkDeleteStoredProcedureStatement(
      name(`bulk_delete_${plural}`), source.table, source.typedColumns, payload, source.key, source.keyType,
    ),
  );
  return statements;
};

export const procedureNameOf = (statement: IStatement): string => {
  if (
    statement instanceof CreateCreateStoredProcedureStatement ||
    statement instanceof CreateReadStoredProcedureStatement ||
    statement instanceof CreateUpdateStoredProcedureStatement ||
    statement instanceof CreateDeleteStoredProcedureStatement ||
    statement instanceof CreateBulkCreateStoredProcedureStatement ||
    statement instanceof CreateBulkUpdateStoredProcedureStatement ||
    statement instanceof CreateBulkDeleteStoredProcedureStatement
  ) {
    return statement.name;
  }
  throw new Error("procedure statement has no name");
};
