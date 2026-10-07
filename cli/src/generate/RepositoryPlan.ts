import { createCasingStrategy, type ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { createDatasourceNaming } from "@deterministic-code/generators-common/datasource-naming";
import { identityColumns, isCollectionField } from "@deterministic-code/generators-common/spec-types";
import type { DatasourceTable, IDeterministic, Type, TypeField } from "@deterministic-code/deterministic-specifications-typescript/parser";
import type { Dialect } from "./GenerateRequest.ts";
import type { PlannedEnrichment, RepositoryModel } from "./RepositoryModel.ts";

const referenceOf = (references: TypeField["references"]): { type: string; field: string } | undefined => {
  if (references === undefined) return undefined;
  const [type, field] = [references].flat().join(".").split(".");
  return type !== undefined && field !== undefined && type.length > 0 && field.length > 0 ? { type, field } : undefined;
};

const childNames = (model: RepositoryModel): string[] => [
  ...new Set(model.enrichments.flatMap((item) => (item.link === undefined ? [item.via] : [item.via, item.link]))),
];

const guardCycles = (models: RepositoryModel[]): void => {
  const byName = new Map(models.map((model) => [model.specName, model]));
  const walk = (model: RepositoryModel, stack: readonly string[]): void => {
    if (stack.includes(model.specName)) throw new Error(`repository cycle: ${[...stack, model.specName].join(" -> ")}`);
    for (const child of childNames(model)) {
      const next = byName.get(child);
      if (next) walk(next, [...stack, model.specName]);
    }
  };
  for (const model of models) walk(model, []);
};

export const planRepositories = (
  spec: IDeterministic,
  dialect: Dialect,
  settings: Record<string, string>,
): RepositoryModel[] => {
  const casing = createCasingStrategy(dialect, settings);
  const naming = createDatasourceNaming(settings);
  const types = new Map(spec.types.map((type) => [type.name, type]));
  const expanded = new Map(spec.expandedTypes.map((type) => [type.name, type]));
  const referenced = new Set<string>();
  for (const type of spec.types) {
    for (const field of type.fields) {
      const ref = referenceOf(field.references);
      if (ref) referenced.add(ref.type);
    }
  }
  const viewByTable = new Map<string, Type>();
  for (const type of spec.types) {
    if (type.tags.includes("view_type") && type.inherits !== undefined) viewByTable.set(type.inherits, type);
  }
  const abstractParent = (table: DatasourceTable): boolean => {
    const inherited = spec.types.some((type) => type.inherits === table.name && type.tags.includes("datasource_type"));
    return inherited && table.mapping === undefined && !referenced.has(table.name);
  };
  const repoName = (tableName: string): string => viewByTable.get(tableName)?.name ?? tableName;
  const property = (name: string): string => casing.convertFields(name);
  const identity = (name: string): string => property(identityColumns(expanded.get(name), spec.datasource.find((table) => table.name === name))[0] ?? "id");
  const models = spec.datasource.flatMap((table): RepositoryModel[] => {
    if (abstractParent(table)) return [];
    const authored = viewByTable.get(table.name) ?? (types.get(table.name) as Type);
    const full = expanded.get(authored.name) as Type;
    const overlays = new Map(table.fields.map((field) => [field.name, field]));
    const projected = new Set(Object.values(authored.mapping ?? {}));
    const columns = full.fields.filter((field) => !isCollectionField(field) && !projected.has(field.name));
    return [
      {
        specName: authored.name,
        typeName: casing.convertTypes(authored.name),
        typeHome: authored.tags.includes("view_type") ? "view" : "datasource",
        entity: casing.convertTypes(authored.name),
        table: naming.resolveTable(table.name, table.mapping),
        propertyMappings: columns.flatMap((field) => {
          const from = property(field.name);
          const to = naming.resolveColumn(field.name, overlays.get(field.name)?.mapping);
          return from === to ? [] : [{ from, to }];
        }),
        ids: identityColumns(full, table).map(property),
        readProperties: columns.map((field) => property(field.name)),
        enrichments: relations(authored, full, casing, types, expanded, repoName, identity),
      },
    ];
  });
  guardCycles(models);
  return models.sort((left, right) => left.specName.localeCompare(right.specName));
};

const relations = (
  authored: Type,
  full: Type,
  casing: ICasingStrategy,
  types: Map<string, Type>,
  expanded: Map<string, Type>,
  repoName: (tableName: string) => string,
  identity: (name: string) => string,
): PlannedEnrichment[] => {
  const property = (name: string): string => casing.convertFields(name);
  const named = (tableName: string): { via: string; viaType: string } => {
    const via = repoName(tableName);
    return { via, viaType: casing.convertTypes(via) };
  };
  const planned: PlannedEnrichment[] = [];
  for (const field of authored.fields) {
    if (!isCollectionField(field)) continue;
    const ref = referenceOf(field.references);
    if (!ref) continue;
    const target = types.get(ref.type);
    const other = target?.fields.find((item) => item.name !== ref.field);
    const otherRef = other === undefined ? undefined : referenceOf(other.references);
    const source = identity(authored.inherits ?? authored.name);
    if (target?.tags.includes("many_to_many") === true && other !== undefined && otherRef !== undefined) {
      const via = named(otherRef.type);
      const link = named(ref.type);
      planned.push({
        destination: property(field.name),
        source,
        relatedKey: identity(otherRef.type),
        kind: "linked",
        via: via.via,
        viaType: via.viaType,
        link: link.via,
        linkType: link.viaType,
        parentFk: property(ref.field),
        relatedFk: property(other.name),
      });
    } else {
      const via = named(ref.type);
      planned.push({
        destination: property(field.name),
        source,
        relatedKey: property(ref.field),
        kind: "list",
        via: via.via,
        viaType: via.viaType,
      });
    }
  }
  for (const [key, dest] of Object.entries(authored.mapping ?? {})) {
    const dotted = key.includes(".") ? key.split(".") : undefined;
    const memberName =
      dotted?.[0] ?? (authored.union ?? []).find((name) => (expanded.get(name) as Type).fields.some((field) => field.name === key));
    const fk = memberName === undefined ? undefined : full.fields.find((field) => referenceOf(field.references)?.type === memberName);
    if (memberName === undefined || fk === undefined) continue;
    const via = named(memberName);
    planned.push({
      destination: property(dest),
      source: property(fk.name),
      relatedKey: identity(memberName),
      relatedProperty: property(dotted?.[1] ?? key),
      kind: "object",
      via: via.via,
      viaType: via.viaType,
    });
  }
  return planned;
};
