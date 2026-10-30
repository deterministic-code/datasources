import { CreateTableStatement } from "@deterministic-code/datasources";
import { createCasing } from "@deterministic-code/datasources-code-generators/sql/common/default-casing";
import { inferReferenceFieldTypes } from "@deterministic-code/datasources-code-generators/sql/common/infer-reference-types";
import { createCasingStrategy, type ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { identityColumns, isCollectionField, persistedColumnFields } from "@deterministic-code/generators-common/spec-types";
import { writeInclusionOf } from "@deterministic-code/datasources-code-generators/sql/common/write-inclusion";
import type { DatasourceTable, IDeterministic, Type, TypeField } from "@deterministic-code/deterministic-generators-common/parser";
import type { Dialect } from "./GenerateRequest.ts";
import type { PlannedEnrichment, RepositoryModel } from "./RepositoryModel.ts";

const createZeroKind = (field: TypeField): "string" | "boolean" | undefined =>
  field.isId === true ||
  field.references !== undefined ||
  field.isNullable === true ||
  (field.defaultValue !== undefined && field.defaultValue !== null)
    ? undefined
    : field.type === "boolean"
      ? "boolean"
      : field.type === "string"
        ? "string"
        : undefined;

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
  const naming = createCasing(settings);
  spec = {
    ...spec,
    types: inferReferenceFieldTypes(spec.types),
    expandedTypes: inferReferenceFieldTypes(spec.expandedTypes),
  };
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
  const included = new Set(spec.datasource.map((table) => table.name));
  const models = spec.datasource.flatMap((table): RepositoryModel[] => {
    if (abstractParent(table)) return [];
    const view = types.get(table.name);
    if (view?.tags.includes("view_type") === true && view.inherits !== undefined && included.has(view.inherits))
      return [];
    const authored = viewByTable.get(table.name) ?? types.get(table.name);
    if (authored === undefined) throw new Error(`${table.name}: no matching type in types.yaml`);
    if (authored.inherits === "dictionary") return [];
    const removed = authored.removeFields ?? [];
    const full = expanded.get(authored.name) as Type;
    const tableType = (expanded.get(table.name) ?? types.get(table.name) ?? full) as Type;
    const overlays = new Map(table.fields.map((field) => [field.name, field]));
    const projected = new Set(Object.values(authored.mapping ?? {}));
    const columns = persistedColumnFields(tableType, expanded)
      .filter((field) => !projected.has(field.name));
    return [
      {
        specName: authored.name,
        typeName: casing.convertTypes(authored.name),
        typeHome: authored.tags.includes("view_type") ? "view" : "datasource",
        entity: casing.convertTypes(authored.name),
        table: CreateTableStatement.tableName(table.mapping ?? table.name, naming.naming),
        propertyMappings: columns.flatMap((field) => {
          const from = property(field.name);
          const to = CreateTableStatement.columnName(overlays.get(field.name)?.mapping ?? field.name, naming.naming);
          return from === to ? [] : [{ from, to }];
        }),
        ids: identityColumns(tableType, table).map(property),
        readProperties: columns.map((field) => property(field.name)),
        createProperties: columns
          .filter((field) =>
            writeInclusionOf(field, identityColumns(tableType, table), overlays.get(field.name)).includeInCreate,
          )
          .map((field) => property(field.name)),
        createZeros: columns.flatMap((field) => {
          if (!writeInclusionOf(field, identityColumns(tableType, table), overlays.get(field.name)).includeInCreate)
            return [];
          const kind = createZeroKind(field);
          return kind === undefined ? [] : [{ name: property(field.name), kind }];
        }),
        updateProperties: columns
          .filter((field) =>
            writeInclusionOf(field, identityColumns(tableType, table), overlays.get(field.name)).includeInUpdate,
          )
          .map((field) => property(field.name)),
        optimisticConcurrency: concurrencyToken(table, tableType, property),
        enrichments: relations(authored, full, casing, types, expanded, repoName, identity),
        removeFields: removed.map(property),
      },
    ];
  });
  guardCycles(models);
  return models.sort((left, right) => left.specName.localeCompare(right.specName));
};

const concurrencyToken = (
  table: DatasourceTable,
  tableType: Type,
  property: (name: string) => string,
): { property: string; increment: boolean } | undefined => {
  if (table.useOptimisticConcurrency === false) return undefined;
  const token = table.fields.find((field) => field.isOptimisticConcurrency === true);
  if (token === undefined) return undefined;
  return {
    property: property(token.name),
    increment: tableType.fields.some((field) =>
      field.name === token.name && (field as { type?: string }).type === "integer",
    ),
  };
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
  const source = identity(authored.inherits ?? authored.name);
  for (const field of authored.fields) {
    if (!isCollectionField(field)) continue;
    if (field.isMap === true) {
      const dict = expanded.get(field.base) ?? types.get(field.base);
      const owner = referenceOf(field.references);
      if (dict === undefined || owner === undefined) continue;
      const via = named(owner.type);
      const value = dict.fields.find((item) => item.name === "value");
      const nested =
        value !== undefined && value.kind === "type" && !value.isArray && value.isMap !== true
          ? expanded.get(value.base) ?? types.get(value.base)
          : undefined;
      const roleColumn = (role: string): string => {
        const item = dict.fields.find((entry) => entry.name === role);
        const ref = referenceOf(item?.references);
        return ref?.type === owner.type ? ref.field : role;
      };
      planned.push({
        destination: property(field.name),
        source,
        relatedKey: property(owner.field),
        relatedProperty: nested === undefined ? property(roleColumn("value")) : undefined,
        kind: "dictionary",
        via: via.via,
        viaType: via.viaType,
      });
      continue;
    }
    const ref = referenceOf(field.references);
    if (!ref) continue;
    const target = types.get(ref.type);
    const other = target?.fields.find((item) => item.name !== ref.field);
    const otherRef = other === undefined ? undefined : referenceOf(other.references);
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
    const parent = types.get(authored.inherits ?? "");
    const fk =
      memberName === undefined
        ? undefined
        : full.fields.find((field) => referenceOf(field.references)?.type === memberName)
          ?? parent?.fields.find((field) => referenceOf(field.references)?.type === memberName);
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
