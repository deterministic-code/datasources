/** Unique fields + unique indexes → `ByProperties` groups including the primary key. */

import { pkName } from "@deterministic-code/generators-common/spec-types";
import { overlayOf, type SqlTable } from "./sql-schema.ts";

const uniqueLookupFields = (type: SqlTable): string[] => {
  const overlay = overlayOf(type);
  const names: string[] = [];
  const add = (name: string): void => {
    if (!names.includes(name)) names.push(name);
  };
  for (const field of overlay.fields) if (field.isUnique === true) add(field.name);
  for (const name of overlay.uniqueIndexFields) add(name);
  return names;
};

const same = (left: string[], right: string[]): boolean =>
  left.length === right.length && left.every((name, index) => name === right[index]);

export const byGroupsFromDatasource = (types: SqlTable[]): Map<string, string[][]> =>
  new Map(
    types.map((type) => {
      const overlay = overlayOf(type);
      const pk = pkName(type, overlay);
      const groups: string[][] = pk === undefined ? [] : [[pk]];
      const add = (props: string[]): void => {
        if (props.length === 0) return;
        if (groups.some((group) => same(group, props))) return;
        groups.push(props);
      };
      for (const name of uniqueLookupFields(type)) {
        if (name !== pk) add([name]);
      }
      for (const index of type.indexes.filter((item) => item.isUnique)) {
        add(index.fields);
      }
      return [type.name, groups] as const;
    }),
  );
