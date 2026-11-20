import { CreateTableStatement } from "@deterministic-code/datasources";
import { createCasing } from "@deterministic-code/datasources-code-generators/sql/common/default-casing";
import { referencePairs } from "@deterministic-code/datasources-code-generators/sql/common/reference-target";
import { createCasingStrategy, type ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { Paths } from "@deterministic-code/generators-common/paths";
import {
  columnFields,
  identityColumns,
  isCollectionField,
  isReadonlyLookup,
  persistedColumnFields,
} from "@deterministic-code/generators-common/spec-types";
import { writeInclusionOf } from "@deterministic-code/datasources-code-generators/sql/common/write-inclusion";
import type { DatasourceTable, IDeterministic, Type, TypeField } from "@deterministic-code/deterministic-generators-common/parser";
import type { Dialect } from "./GenerateRequest.ts";
import type { PlannedEnrichment, RepositoryModel } from "./RepositoryModel.ts";

const referenceOf = (references: TypeField["references"]): { type: string; field: string } | undefined =>
  referencePairs(references)[0];

const compositeKeys = (
  authored: Type,
  field: TypeField,
  pairs: { type: string; field: string }[],
  expanded: Map<string, Type>,
  property: (name: string) => string,
): { source: string; relatedKey: string }[] => {
  const child = pairs[0].type;
  if (!pairs.every((pair) => pair.type === child)) {
    throw new Error(`${authored.name}.${field.name} composite references must name one type`);
  }
  const columns = new Set(expanded.get(authored.name)!.fields.map((item) => item.name));
  for (const pair of pairs) {
    if (!columns.has(pair.field))
      throw new Error(`${authored.name}.${field.name} composite reference ${pair.type}.${pair.field} has no column on ${authored.name}`);
  }
  return pairs.map((pair) => ({ source: property(pair.field), relatedKey: property(pair.field) }));
};

const treeKeys = (
  authored: Type,
  field: TypeField,
  pairs: { type: string; field: string }[],
  expanded: Map<string, Type>,
  property: (name: string) => string,
): { source: string; relatedKey: string }[] => {
  const shape = expanded.get(authored.name)!;
  return pairs.map((pair) => {
    const fk = shape.fields.find((item) => item.name === pair.field);
    const target = referenceOf(fk?.references);
    if (target === undefined)
      throw new Error(`${authored.name}.${field.name} tree reference ${pair.type}.${pair.field} has no parent key`);
    return { source: property(target.field), relatedKey: property(pair.field) };
  });
};

const childNames = (model: RepositoryModel): string[] => [
  ...new Set(
    model.enrichments.flatMap((item) => {
      const names = item.link === undefined ? [item.via] : [item.via, item.link];
      return names.filter((name) => name !== model.specName);
    }),
  ),
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
  const paths = new Paths();
  const naming = createCasing(settings);
  const typeName = (specName: string): string => casing.convertTypes(paths.typeStem(specName));
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
    if (
      type.tags.includes("view_type") &&
      type.inherits !== undefined &&
      type.inherits !== "dictionary" &&
      !type.tags.includes("tree") &&
      !type.tags.includes("graph")
    ) {
      viewByTable.set(type.inherits, type);
    }
  }
  const abstractParent = (table: DatasourceTable): boolean => {
    const inherited = spec.types.some((type) => type.inherits === table.name && type.tags.includes("datasource_type"));
    return inherited && table.mapping === undefined && !referenced.has(table.name);
  };
  const repoName = (tableName: string): string => viewByTable.get(tableName)?.name ?? tableName;
  const property = (name: string): string => casing.convertFields(name);
  const identity = (name: string): string => {
    const col = identityColumns(expanded.get(name), spec.datasource.find((table) => table.name === name))[0];
    if (col === undefined) throw new Error(`${name}: missing primary key`);
    return property(col);
  };
  const included = new Set(spec.datasource.map((table) => table.name));
  const skip = (table: DatasourceTable, repoType: Type | undefined): boolean => {
    if (abstractParent(table)) return true;
    if (repoType === undefined) throw new Error(`${table.name}: no matching type in types.yaml`);
    if (repoType.inherits === "dictionary") return true;
    if (
      repoType.tags.includes("view_type") &&
      !repoType.tags.includes("datasource_type") &&
      (repoType.inherits === undefined || repoType.inherits === "dictionary")
    )
      return true;
    const asView = types.get(table.name);
    return asView?.tags.includes("view_type") === true && asView.inherits !== undefined && included.has(asView.inherits);
  };
  const tableModels = spec.datasource.flatMap((table): RepositoryModel[] => {
    const overlay = viewByTable.get(table.name) ?? types.get(table.name);
    if (skip(table, overlay)) return [];
    const sourceType = types.get(table.name);
    const overlayTypes =
      sourceType !== undefined &&
      sourceType !== overlay &&
      overlay!.tags.includes("readonly_lookup") &&
      sourceType.tags.includes("datasource_type")
        ? [overlay!, sourceType]
        : [overlay!];
    const treeGraphViews = spec.types.filter(
      (type) =>
        (type.tags.includes("tree") === true || type.tags.includes("graph") === true) &&
        type.inherits === table.name,
    );
    const authoredTypes = [
      ...overlayTypes.filter((type) => !treeGraphViews.includes(type)),
      ...treeGraphViews,
    ];
    return authoredTypes.map((authored): RepositoryModel => {
    const shape = expanded.get(authored.name);
    if (shape === undefined) throw new Error(`${authored.name}: no expanded type`);
    const overlays = new Map(table.fields.map((field) => [field.name, field]));
    const enrichments = relations(authored, casing, types, expanded, repoName, identity, typeName);
    const skipColumns = new Set(enrichments.filter((item) => item.kind === "object").map((item) => item.destination));
    const schema = persistedColumnFields(shape).flatMap((field) => {
      const from = property(field.name);
      if (field.type.length === 0) return [];
      return [{
        from,
        type: field.type,
        nullable: field.isNullable === true,
        hasDefault: field.defaultValue !== undefined && field.defaultValue !== null,
        isId: field.isId === true,
      }];
    });
    const names = new Set(schema.filter((column) => !skipColumns.has(column.from)).map((column) => column.from));
    const columns = persistedColumnFields(shape).filter((field) => names.has(property(field.name)));
    const ids = identityColumns(shape, table);
    const parent = authored.inherits === undefined ? undefined : types.get(authored.inherits);
    const treeOrGraph = authored.tags.includes("tree") === true || authored.tags.includes("graph") === true;
    const readonly =
      !treeOrGraph &&
      (isReadonlyLookup(authored) || (parent !== undefined && isReadonlyLookup(parent)));
    const inherited = expanded.get(authored.inherits ?? "") ?? parent;
    const sqlColumn = (fieldName: string): string =>
      CreateTableStatement.columnName(overlays.get(fieldName)?.mapping ?? fieldName, naming.naming);
    const yamlFields = [
      ...overlays.keys(),
      ...(inherited?.fields.map((field) => field.name) ?? []),
      ...table.fields.map((field) => field.name),
    ];
    const mapped = new Map<string, string>();
    const mapField = (fieldName: string): void => {
      const from = property(fieldName);
      const to = sqlColumn(fieldName);
      if (from !== to) mapped.set(from, to);
    };
    for (const field of columns) mapField(field.name);
    const readProperties = columns.map((field) => property(field.name));
    const readSet = new Set(readProperties);
    for (const item of enrichments) {
      if (item.kind !== "object") continue;
      const yamlName = yamlFields.find((name) => property(name) === item.source) ?? item.source;
      const to = sqlColumn(yamlName);
      if (item.source !== to) mapped.set(item.source, to);
      if (!readSet.has(item.source)) {
        readSet.add(item.source);
        readProperties.push(item.source);
      }
    }
    return {
        specName: authored.name,
        typeName: typeName(authored.name),
        typeHome: authored.tags.includes("view_type") ? "view" : "datasource",
        table: CreateTableStatement.tableName(table.mapping ?? table.name, naming.naming),
        propertyMappings: [...mapped].map(([from, to]) => ({ from, to })),
        ids: ids.map(property),
        readProperties,
        createProperties: columns
          .filter((field) => writeInclusionOf(field, ids, overlays.get(field.name)).includeInCreate)
          .map((field) => property(field.name)),
        updateProperties: columns
          .filter((field) => writeInclusionOf(field, ids, overlays.get(field.name)).includeInUpdate)
          .map((field) => property(field.name)),
        schema,
        optimisticConcurrency: readonly ? undefined : concurrencyToken(table, shape, property),
        enrichments,
        readonly,
      };
    });
  });
  const usedMaps = new Set(
    spec.types.flatMap((type) => type.fields.filter((field) => field.isMap === true).map((field) => field.base)),
  );
  const dictionaries = spec.types.flatMap((dict): RepositoryModel[] => {
    if (dict.inherits !== "dictionary") return [];
    if (usedMaps.has(dict.name)) return [];
    const key = dict.fields.find((field) => field.name === "key");
    const keyRef = referenceOf(key?.references);
    if (keyRef === undefined) return [];
    const table = spec.datasource.find((item) => item.name === keyRef.type);
    if (table === undefined) throw new Error(`${dict.name}: no matching type in types.yaml`);
    const overlays = new Map(table.fields.map((field) => [field.name, field]));
    const columns = columnFields(dict.fields).filter(
      (field) => field.name === "key" || (field.name === "value" && field.kind === "primitive"),
    );
    const sqlName = (field: TypeField): string => {
      const ref = referenceOf(field.references);
      const yamlName = ref?.field ?? field.name;
      return CreateTableStatement.columnName(overlays.get(yamlName)?.mapping ?? yamlName, naming.naming);
    };
    const overlayFor = (field: TypeField) =>
      field.name === "key"
        ? { name: sqlName(field), isFixedId: true }
        : overlays.get(referenceOf(field.references)?.field ?? field.name);
    const ids = ["key"];
    const schema = columns.flatMap((field) => {
      if (field.type.length === 0) return [];
      return [{
        from: property(field.name),
        type: field.type,
        nullable: field.isNullable === true,
        hasDefault: field.defaultValue !== undefined && field.defaultValue !== null,
        isId: field.name === "key",
      }];
    });
    const mapped = columns.flatMap((field) => {
      const from = property(field.name);
      const to = sqlName(field);
      return from === to ? [] : [{ from, to }];
    });
    const tableType = types.get(table.name);
    const readonly =
      isReadonlyLookup(dict) || (tableType !== undefined && isReadonlyLookup(tableType));
    const shape = expanded.get(dict.name);
    if (shape === undefined) throw new Error(`${dict.name}: no expanded type`);
    return [
      {
        specName: dict.name,
        typeName: typeName(dict.name),
        typeHome: "view",
        table: CreateTableStatement.tableName(table.mapping ?? table.name, naming.naming),
        propertyMappings: mapped,
        ids: ids.map(property),
        readProperties: columns.map((field) => property(field.name)),
        createProperties: columns
          .filter((field) => writeInclusionOf(field, ids, overlayFor(field)).includeInCreate)
          .map((field) => property(field.name)),
        updateProperties: columns
          .filter((field) => writeInclusionOf(field, ids, overlayFor(field)).includeInUpdate)
          .map((field) => property(field.name)),
        schema,
        optimisticConcurrency: readonly ? undefined : concurrencyToken(table, shape, property),
        enrichments: [],
        readonly,
      },
    ];
  });
  const models = [...tableModels, ...dictionaries];
  guardCycles(models);
  return models.sort((left, right) => left.specName.localeCompare(right.specName));
};

const concurrencyToken = (
  table: DatasourceTable,
  shape: Type,
  property: (name: string) => string,
): { property: string; increment: boolean } | undefined => {
  if (table.useOptimisticConcurrency === false) return undefined;
  const token = table.fields.find((field) => field.isOptimisticConcurrency === true);
  if (token === undefined) return undefined;
  return {
    property: property(token.name),
    increment: shape.fields.some((field) => field.name === token.name && field.type === "integer"),
  };
};

const relations = (
  authored: Type,
  casing: ICasingStrategy,
  types: Map<string, Type>,
  expanded: Map<string, Type>,
  repoName: (tableName: string) => string,
  identity: (name: string) => string,
  typeName: (specName: string) => string,
): PlannedEnrichment[] => {
  const property = (name: string): string => casing.convertFields(name);
  const named = (tableName: string): { via: string; viaType: string } => {
    const via = repoName(tableName);
    return { via, viaType: typeName(via) };
  };
  const planned: PlannedEnrichment[] = [];
  for (const field of authored.fields) {
    if (!isCollectionField(field)) continue;
    const pairs = referencePairs(field.references);
    const keys = pairs.length > 1 ? compositeKeys(authored, field, pairs, expanded, property) : undefined;
    const matched = keys?.[0];
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
      const keyRef = referenceOf(dict.fields.find((entry) => entry.name === "key")?.references);
      if (keyRef === undefined) throw new Error(`${dict.name}.key has no references`);
      planned.push({
        destination: property(field.name),
        source: matched?.source ?? identity(authored.inherits ?? authored.name),
        relatedKey: matched?.relatedKey ?? property(owner.field),
        relatedProperty: nested === undefined ? property(roleColumn("value")) : undefined,
        dictionaryKey: property(keyRef.field),
        kind: "dictionary",
        via: via.via,
        viaType: via.viaType,
        ...(keys !== undefined ? { keys } : {}),
      });
      continue;
    }
    const ref = referenceOf(field.references);
    if (!ref) continue;
    const target = types.get(ref.type);
    const other = target?.fields.find((item) => item.name !== ref.field && referenceOf(item.references) !== undefined);
    const otherRef = other === undefined ? undefined : referenceOf(other.references);
    const hostTable = authored.inherits ?? authored.name;
    const graphSelf =
      authored.tags.includes("graph") === true &&
      field.base === authored.name &&
      target?.tags.includes("many_to_many") === true &&
      other !== undefined &&
      otherRef !== undefined &&
      (otherRef.type === hostTable || otherRef.type === authored.name);
    const treeSelf = authored.tags.includes("tree") === true && field.base === authored.name;
    if (graphSelf && other !== undefined && otherRef !== undefined) {
      const via = named(authored.name);
      const link = named(ref.type);
      planned.push({
        destination: property(field.name),
        source: identity(hostTable),
        relatedKey: identity(otherRef.type),
        kind: "graph",
        via: via.via,
        viaType: via.viaType,
        link: link.via,
        linkType: link.viaType,
        parentFk: property(ref.field),
        relatedFk: property(other.name),
      });
    } else if (target?.tags.includes("many_to_many") === true && other !== undefined && otherRef !== undefined) {
      const via = named(otherRef.type);
      const link = named(ref.type);
      const relatedType = types.get(otherRef.type) ?? expanded.get(otherRef.type);
      if (relatedType === undefined) throw new Error(`${otherRef.type}: missing type`);
      planned.push({
        destination: property(field.name),
        source: identity(authored.inherits ?? authored.name),
        relatedKey: identity(otherRef.type),
        relatedProperty: relatedType.fields.some((item) => item.name === "name") ? property("name") : undefined,
        kind: "linked",
        via: via.via,
        viaType: via.viaType,
        link: link.via,
        linkType: link.viaType,
        parentFk: property(ref.field),
        relatedFk: property(other.name),
      });
    } else if (treeSelf) {
      const via = named(authored.name);
      const treePairs = pairs.length > 1 ? treeKeys(authored, field, pairs, expanded, property) : undefined;
      const first = treePairs?.[0];
      planned.push({
        destination: property(field.name),
        source: first?.source ?? identity(hostTable),
        relatedKey: first?.relatedKey ?? property(ref.field),
        kind: "tree",
        via: via.via,
        viaType: via.viaType,
        ...(treePairs !== undefined ? { keys: treePairs } : {}),
      });
    } else {
      const via = named(ref.type);
      planned.push({
        destination: property(field.name),
        source: matched?.source ?? identity(authored.inherits ?? authored.name),
        relatedKey: matched?.relatedKey ?? property(ref.field),
        kind: "list",
        via: via.via,
        viaType: via.viaType,
        ...(keys !== undefined ? { keys } : {}),
      });
    }
  }
  const parent = types.get(authored.inherits ?? "") ?? expanded.get(authored.inherits ?? "");
  for (const memberName of authored.union ?? []) {
    const related = (expanded.get(memberName) ?? types.get(memberName)) as Type | undefined;
    const fk =
      authored.fields.find((field) => referenceOf(field.references)?.type === memberName) ??
      parent?.fields.find((field) => referenceOf(field.references)?.type === memberName);
    if (related === undefined || fk === undefined) continue;
    const extracted = (authored.extract ?? []).flatMap((item) => {
      const dotted = item.includes(".") ? item.split(".") : undefined;
      const owner = dotted?.[0];
      const fieldName = dotted?.[1] ?? item;
      if (owner !== undefined && owner !== memberName) return [];
      if (owner === undefined && !related.fields.some((field) => field.name === fieldName)) return [];
      return [fieldName];
    });
    const fields = extracted.length > 0 ? extracted : related.fields.some((field) => field.name === "name") ? ["name"] : [];
    const via = named(memberName);
    for (const fieldName of fields) {
      const dest =
        authored.mapping?.[`${memberName}.${fieldName}`] ?? authored.mapping?.[fieldName] ?? fieldName;
      planned.push({
        destination: property(dest),
        source: property(fk.name),
        relatedKey: identity(memberName),
        relatedProperty: property(fieldName),
        kind: "object",
        via: via.via,
        viaType: via.viaType,
      });
    }
  }
  return planned;
};
