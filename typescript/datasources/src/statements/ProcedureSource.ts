import type { RoutineParameter } from "./RoutineParameter.ts";

export type ProcedureByGroup = {
  logical: readonly string[]
  columns: readonly string[]
  parameters: readonly RoutineParameter[]
  updateParameters: readonly RoutineParameter[]
};

export type ProcedureSource = {
  plural: string
  table: string
  columns: readonly string[]
  createParameters: readonly RoutineParameter[]
  writable: readonly string[]
  createColumns: readonly RoutineParameter[]
  updateColumns: readonly RoutineParameter[]
  typedColumns: readonly RoutineParameter[]
  byGroups: readonly ProcedureByGroup[]
  key: string
  keyType: string
  auditUpdated?: string
  generatedKey: boolean
  routineName: (stem: string) => string
};
