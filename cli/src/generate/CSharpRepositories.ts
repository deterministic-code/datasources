import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import {
  childClass,
  childField,
  isLinked,
  ownedChildren,
  repositoryFile,
  type PlannedEnrichment,
  type RepositoryModel,
} from "./RepositoryModel.ts";

const quote = (value: string): string => value.replaceAll("\\", "\\\\").replaceAll("\"", "\\\"");

const enrichment = (item: PlannedEnrichment): string => {
  const via = childField("csharp", item.via, item.viaType);
  const head = `"${quote(item.destination)}", "${quote(item.source)}", "${quote(item.relatedKey)}"`;
  if (isLinked(item)) {
    const link = childField("csharp", item.link, item.linkType);
    return `AddLinked(new(${head}, Association: new("${quote(item.parentFk)}", "${quote(item.relatedFk)}")), ${via}, ${link});`;
  }
  const related = item.relatedProperty === undefined ? "" : `, "${quote(item.relatedProperty)}"`;
  const type = item.kind === "list" ? ", Type: EnrichmentPropertyType.List" : "";
  return `AddEnriched(new(${head}${related}${type}), ${via});`;
};

export const csharpRepositories = (models: RepositoryModel[]): GenerateEntry[] =>
  models.map((model) => {
    const type = `${model.typeHome === "view" ? "Backend.Types.View" : "Backend.Types.Datasource"}.${model.typeName}`;
    const owned = ownedChildren(model);
    const fields = owned.map((child) => `    public ${childClass(child.typeName)} ${childField("csharp", child.specName, child.typeName)} { get; }`);
    const lines = [
      ...owned.map((child) => `        ${childField("csharp", child.specName, child.typeName)} = new ${childClass(child.typeName)}(connection);`),
      ...model.enrichments.map((item) => `        ${enrichment(item)}`),
    ];
    const mapping = `        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>\n        {\n${model.propertyMappings.map((pair) => `            ["${quote(pair.from)}"] = "${quote(pair.to)}",`).join("\n")}\n        }),\n`;
    const constructor = lines.length === 0
      ? `    public ${model.typeName}Repository(IConnection connection) : base(connection, Options()) { }`
      : `    public ${model.typeName}Repository(IConnection connection) : base(connection, Options())\n    {\n${lines.join("\n")}\n    }`;
    const ids = model.ids.map((name) => `"${quote(name)}"`).join(", ");
    const read = model.readProperties.map((name) => `"${quote(name)}"`).join(", ");
    return content(
      repositoryFile("csharp", model),
      `using Datasources;\n\npublic sealed class ${model.typeName}Repository : CRUDRepository<${type}>\n{\n${fields.length === 0 ? "" : `${fields.join("\n")}\n\n`}${constructor}\n\n    static CRUDRepositoryOptions<${type}> Options() => new()\n    {\n        Entity = "${quote(model.entity)}",\n        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["${quote(model.entity)}"] = "${quote(model.table)}" }),\n${mapping}        Ids = [${ids}],\n        ReadProperties = [${read}],\n    };\n}\n`,
    );
  });
