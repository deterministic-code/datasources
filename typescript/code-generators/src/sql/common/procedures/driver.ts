import type { PackCasing } from "../default-casing.ts";

type ProcOp =
  | "create"
  | "findOne"
  | "findAll"
  | "findBy"
  | "update"
  | "updateBy"
  | "delete";

type ProcSpec = {
  op: ProcOp;
  name: string;
  byField?: string;
};

/** Ordered CRUD ops + routine names for one entity (CREATE bodies and DROP names). */
export const procedureSpecs = (
  entityName: string,
  byFields: string[],
  casing: PackCasing,
): ProcSpec[] => {
  const name = (stem: string): string => casing.routineName(stem);
  return [
    { op: "create", name: name(`create_${entityName}`) },
    { op: "findOne", name: name(`find_${entityName}`) },
    {
      op: "findAll",
      name: name(`find_${casing.pluralTableName(entityName)}`),
    },
    ...byFields.map((bf) => ({
      op: "findBy" as const,
      name: name(`find_${entityName}_by_${bf}`),
      byField: bf,
    })),
    { op: "update", name: name(`update_${entityName}`) },
    ...byFields.map((bf) => ({
      op: "updateBy" as const,
      name: name(`update_${entityName}_by_${bf}`),
      byField: bf,
    })),
    { op: "delete", name: name(`delete_${entityName}`) },
  ];
};
