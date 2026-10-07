import { createCasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { childClass, childField, isLinked, ownedChildren, repositoryFile, type PlannedEnrichment, type RepositoryModel } from "./RepositoryModel.ts";

const quote = (value: string): string => value.replaceAll("\\", "\\\\").replaceAll("\"", "\\\"");

const enrichment = (item: PlannedEnrichment): string => {
  const via = `this.${childField("typescript", item.via, item.viaType)}`;
  const related = item.relatedProperty === undefined ? "" : `, relatedProperty: "${quote(item.relatedProperty)}"`;
  const call = isLinked(item) ? "addLinked" : "addEnriched";
  const association = isLinked(item) ? `, association: { parent: "${quote(item.parentFk)}", related: "${quote(item.relatedFk)}" }` : "";
  const link = isLinked(item) ? `, this.${childField("typescript", item.link, item.linkType)}` : "";
  const type = item.kind === "object" ? "object" : "list";
  return `this.${call}({ destinationProperty: "${quote(item.destination)}", sourceProperty: "${quote(item.source)}", relatedKey: "${quote(item.relatedKey)}"${related}${association}, type: "${type}" }${link === "" ? `, ${via}` : `, ${via}${link}`});`;
};

export const typescriptRepositories = (models: RepositoryModel[]): GenerateEntry[] => {
  const casing = createCasingStrategy("typescript");
  return models.map((model) => {
    const folder = model.typeHome === "view" ? "views" : "datasource";
    const owned = ownedChildren(model);
    const mapping = model.propertyMappings.length === 0
      ? ""
      : `propertyMappings: new DictionaryMapper([${model.propertyMappings.map((pair) => `["${quote(pair.from)}", "${quote(pair.to)}"]`).join(", ")}]), `;
    const options = `{ entity: "${quote(model.entity)}", entityMapper: new DictionaryMapper([["${quote(model.entity)}", "${quote(model.table)}"]]), ${mapping}ids: [${model.ids.map((id) => `"${quote(id)}"`).join(", ")}], readProperties: [${model.readProperties.map((name) => `"${quote(name)}"`).join(", ")}] }`;
    const fields = owned.map((child) => `  readonly ${childField("typescript", child.specName, child.typeName)}: ${childClass(child.typeName)};`);
    const lines = [
      ...owned.map((child) => `    this.${childField("typescript", child.specName, child.typeName)} = new ${childClass(child.typeName)}(connection);`),
      ...model.enrichments.map((item) => `    ${enrichment(item)}`),
    ];
    const imports = owned.map((child) => `import { ${childClass(child.typeName)} } from "./${child.specName.replaceAll("_", "-")}-repository.ts";`);
    return content(
      repositoryFile("typescript", model),
      `import type { ${model.typeName} } from "../types/generated/${folder}/${casing.convertFileName(model.specName)}.ts";\nimport { CRUDRepository, DictionaryMapper, type IConnection } from "@datasources/core";\n${imports.length === 0 ? "" : `${imports.join("\n")}\n`}\nexport class ${model.typeName}Repository extends CRUDRepository<${model.typeName}> {\n${fields.length === 0 ? "" : `${fields.join("\n")}\n`}  constructor(connection: IConnection) {\n    super(connection, ${options});\n${lines.length === 0 ? "" : `${lines.join("\n")}\n`}  }\n}\n`,
    );
  });
};
