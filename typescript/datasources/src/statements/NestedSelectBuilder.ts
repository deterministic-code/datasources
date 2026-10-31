import type { NestedSelectBuilderOptions } from "./NestedSelectBuilderOptions.ts";
import type { NestedSelectStatement } from "./NestedSelectStatement.ts";
import type { NestedSelectType } from "./NestedSelectType.ts";
import { selectFieldAliases } from "./SelectFieldAliases.ts";
import { Join } from "./Join.ts";
import { SelectStatement } from "./SelectStatement.ts";
import type { Join as JoinType } from "./Join.ts";
import type { TypeMapGroup } from "../mappers/TypeMapGroup.ts";

type FieldMap = Record<string, string | TypeMapGroup<Record<string, string>>>;

export class NestedSelectBuilder {
  build(root: NestedSelectType, options?: NestedSelectBuilderOptions): NestedSelectStatement {
    const stopAt = options?.stopAt ?? [];
    const fields: string[] = [];
    const joins: JoinType[] = [];
    walk(root, root.name, root.name, "", stopAt, false, fields, joins);
    const aliases = selectFieldAliases(root.name, fields);
    const byPath = Object.fromEntries(fields.map((field, i) => [field, aliases[i] as string]));
    return {
      select: new SelectStatement({
        table: root.name,
        fields,
        joins,
        where: options?.where,
      }),
      map: mapOf(root, root.name, "", stopAt, byPath),
    };
  }
}

const walk = (
  type: NestedSelectType,
  fieldPrefix: string,
  joinEntity: string,
  stopPrefix: string,
  stopAt: readonly string[],
  outer: boolean,
  fields: string[],
  joins: JoinType[],
): void => {
  for (const property of type.properties) fields.push(`${fieldPrefix}.${property}`);
  for (const relation of type.relations) {
    const path = stopPrefix.length === 0 ? relation.name : `${stopPrefix}.${relation.name}`;
    if (stopped(stopAt, path)) continue;
    const list = relation.kind === "list";
    joins.push(
      list || outer
        ? Join.left(relation.target.name, `${joinEntity}.${relation.from}`, `${relation.target.name}.${relation.to}`)
        : Join.inner(relation.target.name, `${joinEntity}.${relation.from}`, `${relation.target.name}.${relation.to}`),
    );
    walk(relation.target, `${fieldPrefix}.${relation.target.name}`, relation.target.name, path, stopAt, outer || list, fields, joins);
  }
};

const mapOf = (
  type: NestedSelectType,
  fieldPrefix: string,
  stopPrefix: string,
  stopAt: readonly string[],
  aliases: Record<string, string>,
): { key: string; fields: FieldMap } => ({
  key: aliases[`${fieldPrefix}.${type.key}`] as string,
  fields: fieldsOf(type, fieldPrefix, stopPrefix, stopAt, aliases),
});

const fieldsOf = (
  type: NestedSelectType,
  fieldPrefix: string,
  stopPrefix: string,
  stopAt: readonly string[],
  aliases: Record<string, string>,
): FieldMap => {
  const fields: FieldMap = {};
  for (const property of type.properties) {
    const alias = aliases[`${fieldPrefix}.${property}`] as string;
    fields[alias] = alias;
  }
  for (const relation of type.relations) {
    const path = stopPrefix.length === 0 ? relation.name : `${stopPrefix}.${relation.name}`;
    if (stopped(stopAt, path)) continue;
    const childPrefix = `${fieldPrefix}.${relation.target.name}`;
    if (relation.kind === "list") {
      fields[relation.name] = {
        key: aliases[`${childPrefix}.${relation.target.key}`] as string,
        fields: fieldsOf(relation.target, childPrefix, path, stopAt, aliases),
      };
      continue;
    }
    Object.assign(fields, fieldsOf(relation.target, childPrefix, path, stopAt, aliases));
  }
  return fields;
};

const stopped = (stopAt: readonly string[], path: string): boolean =>
  stopAt.some((item) => item === path || path.startsWith(`${item}.`));
