import { ChangeCasingMapper, NestedSelectBuilder, PluralizerMapper } from "@deterministic-code/datasources";
import type { NestedSelectRelation, NestedSelectType } from "@deterministic-code/datasources";
import type { Type, TypeField } from "@deterministic-code/deterministic-generators-common/parser";

const pascal = new ChangeCasingMapper("snakeCase", "pascalCase");
const plural = new PluralizerMapper();

export const nestedSelectFromSpec = (types: readonly Type[], root: string): NestedSelectType => {
  const tagged = types.some((item) => item.tags?.includes("datasource_type"));
  const catalog = new Map(
    types
      .filter((item) => !tagged || item.tags?.includes("datasource_type"))
      .map((item) => [item.name, item]),
  );
  const name = catalog.has(root)
    ? root
    : [...catalog.keys()].find((item) => entity(item) === root);
  if (name === undefined) throw new Error(`Unknown nested select type "${root}"`);
  return build(catalog.get(name) as Type, catalog, new Set());
};

export { NestedSelectBuilder };

const build = (
  type: Type,
  catalog: Map<string, Type>,
  ancestors: Set<string>,
): NestedSelectType => {
  const next = new Set(ancestors);
  next.add(type.name);
  const key = entity(type.fields.find((field) => field.isId)?.name ?? type.ids?.[0] ?? "id");
  const properties = type.fields
    .filter((field) => field.references === undefined)
    .map((field) => entity(field.name));
  const objects = type.fields.flatMap((field) => {
    const [table, column] = target(field);
    if (!table || !catalog.has(table) || next.has(table)) return [];
    return [{
      name: relationName(field.name, table),
      kind: "object" as const,
      from: entity(field.name),
      to: entity(column),
      target: build(catalog.get(table) as Type, catalog, next),
    } satisfies NestedSelectRelation];
  });
  const lists = [...catalog.values()].flatMap((child) => {
    if (next.has(child.name)) return [];
    return child.fields.flatMap((field) => {
      const [table, column] = target(field);
      if (table !== type.name || !column) return [];
      return [{
        name: plural.map(entity(child.name)),
        kind: "list" as const,
        from: key,
        to: entity(field.name),
        target: build(child, catalog, next),
      } satisfies NestedSelectRelation];
    });
  });
  return { name: entity(type.name), key, properties, relations: [...objects, ...lists] };
};

const target = (field: TypeField): [string, string] => {
  if (Array.isArray(field.references) && field.references.length === 2)
    return [field.references[0] ?? "", field.references[1] ?? ""];
  const [table, column] = String(field.references ?? "").split(".");
  return [table ?? "", column ?? ""];
};

const relationName = (field: string, table: string): string => {
  const name = entity(field);
  return name.endsWith("Id") && name.length > 2 ? name.slice(0, -2) : entity(table);
};

const entity = (name: string): string => pascal.map(name);
