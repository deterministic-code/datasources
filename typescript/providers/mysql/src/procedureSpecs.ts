export type ProcOp =
  | "create"
  | "read"
  | "update"
  | "delete"
  | "bulkCreate"
  | "bulkUpdate"
  | "bulkDelete";

export type ProcSpec = {
  op: ProcOp;
  name: string;
  byProperties: string[];
};

export const procedureSpecs = (
  entityName: string,
  byGroups: string[][],
  casing: { routineName: (stem: string) => string; pluralStem: (entity: string) => string },
): ProcSpec[] => {
  const name = (stem: string): string => casing.routineName(stem);
  const plural = casing.pluralStem(entityName);
  const bySuffix = (props: string[]): string =>
    props.length === 0 ? "" : `_by_${props.join("_")}`;
  return [
    { op: "create", name: name(`create_${plural}`), byProperties: [] },
    { op: "read", name: name(`read_${plural}`), byProperties: [] },
    ...byGroups.flatMap((props) => [
      { op: "read" as const, name: name(`read_${plural}${bySuffix(props)}`), byProperties: props },
      { op: "update" as const, name: name(`update_${plural}${bySuffix(props)}`), byProperties: props },
      { op: "delete" as const, name: name(`delete_${plural}${bySuffix(props)}`), byProperties: props },
    ]),
    { op: "bulkCreate", name: name(`bulk_create_${plural}`), byProperties: [] },
    { op: "bulkUpdate", name: name(`bulk_update_${plural}`), byProperties: [] },
    { op: "bulkDelete", name: name(`bulk_delete_${plural}`), byProperties: [] },
  ];
};
