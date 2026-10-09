import { createCasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { childClass, childField, children as childModels, isLinked, pluralCamel, repositoryFile, type PlannedEnrichment, type RepositoryModel } from "./RepositoryModel.ts";

const quote = (value: string): string => value.replaceAll("\\", "\\\\").replaceAll("\"", "\\\"");

const enrichment = (item: PlannedEnrichment, host: "this" | "repo"): string => {
  const via = host === "this"
    ? `this.${childField("typescript", item.via, item.viaType)}`
    : childField("typescript", item.via, item.viaType);
  const related = item.relatedProperty === undefined ? "" : `, relatedProperty: "${quote(item.relatedProperty)}"`;
  const type = item.kind === "object" ? "object" : item.kind === "dictionary" ? "dictionary" : "list";
  if (host === "this" && isLinked(item)) {
    const link = `this.${childField("typescript", item.link, item.linkType)}`;
    return `this.addLinked({ destinationProperty: "${quote(item.destination)}", sourceProperty: "${quote(item.source)}", relatedKey: "${quote(item.relatedKey)}"${related}, association: { parent: "${quote(item.parentFk)}", related: "${quote(item.relatedFk)}" }, type: "${type}" }, ${via}, ${link});`;
  }
  return `${host}.addEnriched({ destinationProperty: "${quote(item.destination)}", sourceProperty: "${quote(item.source)}", relatedKey: "${quote(item.relatedKey)}"${related}, type: "${type}" }, ${via});`;
};

const createReadonly = (model: RepositoryModel, options: string): string => {
  const children = childModels(model);
  if (children.length === 0 && model.enrichments.length === 0)
    return `  static createReadonly = (connection: IConnection) => new ReadonlyRepository<${model.typeName}>(connection, ${options});`;
  const locals = children.map((child) => `    const ${childField("typescript", child.specName, child.typeName)} = new ${childClass(child.typeName)}(connection);`);
  const enrich = model.enrichments.map((item) => `    ${enrichment(item, "repo")}`);
  return `  static createReadonly(connection: IConnection) {\n    const repo = new ReadonlyRepository<${model.typeName}>(connection, ${options});\n${locals.join("\n")}\n${enrich.join("\n")}\n    return repo;\n  }`;
};

export const typescriptRepositories = (models: RepositoryModel[]): GenerateEntry[] => {
  const casing = createCasingStrategy("typescript");
  const files = models.map((model) => {
    const folder = model.typeHome === "view" ? "views" : "datasource";
    const children = childModels(model);
    const mapping = model.propertyMappings.length === 0
      ? ""
      : `propertyMappings: new DictionaryMapper([${model.propertyMappings.map((pair) => `["${quote(pair.from)}", "${quote(pair.to)}"]`).join(", ")}]), `;
    const options = `{ entity: "${quote(model.entity)}", entityMapper: new DictionaryMapper([["${quote(model.entity)}", "${quote(model.table)}"]]), ${mapping}ids: [${model.ids.map((id) => `"${quote(id)}"`).join(", ")}], readProperties: [${model.readProperties.map((name) => `"${quote(name)}"`).join(", ")}] }`;
    const fields = children.map((child) => `  readonly ${childField("typescript", child.specName, child.typeName)}: ${childClass(child.typeName)};`);
    const lines = [
      ...children.map((child) => `    this.${childField("typescript", child.specName, child.typeName)} = new ${childClass(child.typeName)}(connection);`),
      ...model.enrichments.map((item) => `    ${enrichment(item, "this")}`),
    ];
    const imports = children.map((child) => `import { ${childClass(child.typeName)} } from "./${child.specName.replaceAll("_", "-")}-repository.ts";`);
    return content(
      repositoryFile("typescript", model),
      `import type { ${model.typeName} } from "../types/generated/${folder}/${casing.convertFileName(model.specName)}.ts";\nimport { CRUDRepository, DictionaryMapper, ReadonlyRepository, type IConnection } from "@datasources/core";\n${imports.length === 0 ? "" : `${imports.join("\n")}\n`}\nexport class ${model.typeName}Repository extends CRUDRepository<${model.typeName}> {\n${fields.length === 0 ? "" : `${fields.join("\n")}\n`}  constructor(connection: IConnection) {\n    super(connection, ${options});\n${lines.length === 0 ? "" : `${lines.join("\n")}\n`}  }\n${createReadonly(model, options)}\n}\n`,
    );
  });
  const registerImports = models.map((model) => `import { ${childClass(model.typeName)} } from "./${model.specName.replaceAll("_", "-")}-repository.ts";`);
  const registerCalls = models.map((model) => `  factory.register("${quote(model.entity)}", "${quote(pluralCamel(model.typeName))}", (connection) => new ${childClass(model.typeName)}(connection), (connection) => ${childClass(model.typeName)}.createReadonly(connection));`);
  const props = models.map((model) => `    readonly ${pluralCamel(model.typeName)}: ${childClass(model.typeName)};`);
  return [
    ...files,
    content(
      "repository-registry.ts",
      `import type { RepositoryFactory } from "@datasources/core";\n${registerImports.join("\n")}\n\nexport const register = (factory: RepositoryFactory): void => {\n${registerCalls.join("\n")}\n};\n\ndeclare module "@datasources/core" {\n  interface Datasource {\n${props.join("\n")}\n  }\n}\n`,
    ),
  ];
};
