import { createCasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { childClass, childField, children as childModels, isLinked, pluralCamel, repositoryFile, type OptimisticConcurrency, type PlannedEnrichment, type RepositoryModel, type RepositoryOptionsModel } from "./RepositoryModel.ts";
import { dialectsOf, quote, writeList } from "./writeLists.ts";

type Options = RepositoryOptionsModel & { entity: string };

const withEntity = (model: RepositoryModel, planned: RepositoryOptionsModel): Options =>
  ({ entity: model.entity, ...planned });

const entityAndMapping = (model: Options): string => {
  const mapping = model.propertyMappings.length === 0
    ? ""
    : `propertyMappings: new DictionaryMapper([${model.propertyMappings.map((pair) => `["${quote(pair.from)}", "${quote(pair.to)}"]`).join(", ")}]), `;
  return `entity: "${quote(model.entity)}", entityMapper: new DictionaryMapper([["${quote(model.entity)}", "${quote(model.table)}"]]), ${mapping}`;
};

const tsOptions = (model: Options): string =>
  `{ ${entityAndMapping(model)}ids: ${writeList("typescript", model.ids)}, createProperties: ${writeList("typescript", model.createProperties)}, updateProperties: ${writeList("typescript", model.updateProperties)}, readProperties: ${writeList("typescript", model.readProperties)} }`;

const tsReadonly = (model: Options): string =>
  `{ ${entityAndMapping(model)}readProperties: ${writeList("typescript", model.readProperties)} }`;

const tsOptionsExpr = (model: RepositoryModel): string =>
  dialectsOf(model).length === 0 ? tsOptions(model) : "options(connection)";

const tsBuilder = (name: string, model: RepositoryModel, write: (options: Options) => string): string =>
  `const ${name} = (connection: IConnection) =>\n  new CRUDRepositoryBuilder<${model.typeName}>()\n    .add(${write(model)})${dialectsOf(model)
    .map(([engine, planned]) => `\n    .add("${quote(engine)}", ${write(withEntity(model, planned))})`)
    .join("")}\n    .for(connection);\n\n`;

const tsOptionsFn = (model: RepositoryModel): string =>
  dialectsOf(model).length === 0 ? "" : `${tsBuilder("options", model, tsOptions)}${tsBuilder("readonlyOptions", model, tsReadonly)}`;

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

const createReadonly = (model: RepositoryModel): string => {
  const options = dialectsOf(model).length === 0 ? tsReadonly(model) : "readonlyOptions(connection)";
  const children = childModels(model);
  if (children.length === 0 && model.enrichments.length === 0)
    return `  static createReadonly = (connection: IConnection) => new ReadonlyRepository<${model.typeName}>(connection, ${options});`;
  const locals = children.map((child) => `    const ${childField("typescript", child.specName, child.typeName)} = new ${childClass(child.typeName)}(connection);`);
  const enrich = model.enrichments.map((item) => `    ${enrichment(item, "repo")}`);
  return `  static createReadonly(connection: IConnection) {\n    const repo = new ReadonlyRepository<${model.typeName}>(connection, ${options});\n${locals.join("\n")}\n${enrich.join("\n")}\n    return repo;\n  }`;
};

const tokenWhere = (model: RepositoryModel, token: OptimisticConcurrency, expected: string): string => {
  const [first, ...rest] = [...model.ids, token.property];
  const value = (name: string) => name === token.property ? expected : `item.${name}`;
  return `new Where("${quote(first as string)}", eq, ${value(first as string)})${rest.map((name) => `.and("${quote(name)}", eq, ${value(name)})`).join("")}`;
};

const occWrap = (model: RepositoryModel): string => {
  const token = model.optimisticConcurrency;
  if (token === undefined) return "";
  const bump = token.increment ? `      item.${token.property} = Number(expected) + 1;\n` : "";
  return `    const update = this.update;\n    const remove = this.delete;\n    this.update = async (item, byWhere) => {\n      const expected = item.${token.property};\n${bump}      const response = await update(item, byWhere ?? ${tokenWhere(model, token, "expected")});\n      if (response.rowsAffected !== 1) throw new DatasourceError(\`Update affected \${response.rowsAffected} rows\`);\n      return response;\n    };\n    this.delete = async (item, byWhere) => {\n      const response = await remove(item, byWhere ?? ${tokenWhere(model, token, `item.${token.property}`)});\n      if (response.rowsAffected !== 1) throw new DatasourceError(\`Delete affected \${response.rowsAffected} rows\`);\n      return response;\n    };\n`;
};

const repoImport = (specName: string): string => `./${specName.replaceAll("_", "-")}-repository`;

export const typescriptRepositories = (models: RepositoryModel[], settings: Record<string, string>): GenerateEntry[] => {
  const casing = createCasingStrategy("typescript", settings);
  const files = models.map((model) => {
    const folder = model.typeHome === "view" ? "views" : "datasource";
    const children = childModels(model);
    const fields = children.map((child) => `  readonly ${childField("typescript", child.specName, child.typeName)}: ${childClass(child.typeName)};`);
    const wrap = occWrap(model);
    const lines = [
      ...children.map((child) => `    this.${childField("typescript", child.specName, child.typeName)} = new ${childClass(child.typeName)}(connection);`),
      ...model.enrichments.map((item) => `    ${enrichment(item, "this")}`),
    ];
    const imports = children.map((child) => `import { ${childClass(child.typeName)} } from "${repoImport(child.specName)}";`);
    const builder = dialectsOf(model).length === 0 ? "" : "CRUDRepositoryBuilder, ";
    const core = model.optimisticConcurrency === undefined
      ? `import { CRUDRepository, ${builder}DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";`
      : `import { CRUDRepository, ${builder}DatasourceError, DictionaryMapper, eq, ReadonlyRepository, Where, type IConnection } from "@deterministic-code/datasources";`;
    return content(
      repositoryFile("typescript", model),
      `import type { ${model.typeName} } from "../types/generated/${folder}/${casing.convertFileName(model.specName)}";\n${core}\n${imports.length === 0 ? "" : `${imports.join("\n")}\n`}\n${tsOptionsFn(model)}export class ${model.typeName}Repository extends CRUDRepository<${model.typeName}> {\n${fields.length === 0 ? "" : `${fields.join("\n")}\n`}  constructor(connection: IConnection) {\n    super(connection, ${tsOptionsExpr(model)});\n${wrap}${lines.length === 0 ? "" : `${lines.join("\n")}\n`}  }\n${createReadonly(model)}\n}\n`,
    );
  });
  const registerImports = models.map((model) => `import { ${childClass(model.typeName)} } from "${repoImport(model.specName)}";`);
  const registerCalls = models.map((model) => `  factory.register("${quote(model.entity)}", "${quote(pluralCamel(model.typeName))}", (connection) => new ${childClass(model.typeName)}(connection), (connection) => ${childClass(model.typeName)}.createReadonly(connection));`);
  const props = models.map((model) => `    readonly ${pluralCamel(model.typeName)}: ${childClass(model.typeName)};`);
  return [
    ...files,
    content(
      "repository-registry.ts",
      `import { RepositoryFactory } from "@deterministic-code/datasources";\n${registerImports.join("\n")}\n\nexport const register = (factory: RepositoryFactory): void => {\n${registerCalls.join("\n")}\n};\nRepositoryFactory.bind(register);\n\ndeclare module "@deterministic-code/datasources" {\n  interface Datasource {\n${props.join("\n")}\n  }\n}\n`,
    ),
  ];
};
