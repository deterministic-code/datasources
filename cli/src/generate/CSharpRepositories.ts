import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import {
  childClass,
  childField,
  isLinked,
  children as childModels,
  pluralPascal,
  repositoryFile,
  type OptimisticConcurrency,
  type PlannedEnrichment,
  type RepositoryModel,
  type RepositoryOptionsModel,
} from "./RepositoryModel.ts";
import { dialectsOf, pad, quote, writeList } from "./writeLists.ts";

type Options = RepositoryOptionsModel & { entity: string };

const csharpOptionsBody = (model: Options): string =>
  `Entity = "${quote(model.entity)}",
EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["${quote(model.entity)}"] = "${quote(model.table)}" }),
PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
{
${model.propertyMappings.map((pair) => `    ["${quote(pair.from)}"] = "${quote(pair.to)}",`).join("\n")}
}),
Ids = ${writeList("csharp", model.ids)},
ReadProperties = ${writeList("csharp", model.readProperties)},
CreateProperties = ${writeList("csharp", model.createProperties)},
UpdateProperties = ${writeList("csharp", model.updateProperties)},`;

const csharpOptions = (model: Options, indent: number): string =>
  `new()\n${" ".repeat(indent)}{\n${pad(csharpOptionsBody(model), indent + 4)}\n${" ".repeat(indent)}}`;

const optionsMethod = (model: RepositoryModel, type: string): string => {
  const dialects = dialectsOf(model);
  return dialects.length === 0
    ? `    static CRUDRepositoryOptions<${type}> Options() => ${csharpOptions(model, 4)};`
    : `    static CRUDRepositoryOptions<${type}> Options(IConnection connection) =>\n        new CRUDRepositoryBuilder<${type}>()\n            .Add(${csharpOptions(model, 12)})${dialects
        .map(([engine, planned]) => `\n            .Add("${quote(engine)}", ${csharpOptions({ entity: model.entity, ...planned }, 12)})`)
        .join("")}\n            .For(connection);`;
};

const enrichment = (item: PlannedEnrichment, host: "ctor" | "readonly" = "ctor"): string => {
  const via = childField("csharp", item.via, item.viaType);
  const head = `"${quote(item.destination)}", "${quote(item.source)}", "${quote(item.relatedKey)}"`;
  if (host === "ctor" && isLinked(item)) {
    const link = childField("csharp", item.link, item.linkType);
    return `AddLinked(new(${head}, Association: new("${quote(item.parentFk)}", "${quote(item.relatedFk)}")), ${via}, ${link});`;
  }
  const related = item.relatedProperty === undefined ? "" : `, "${quote(item.relatedProperty)}"`;
  const type = item.kind === "object" ? "" : item.kind === "dictionary" ? ", Type: EnrichmentPropertyType.Dictionary" : ", Type: EnrichmentPropertyType.List";
  const call = host === "readonly" ? "repo.AddEnriched" : "AddEnriched";
  return `${call}(new(${head}${related}${type}), ${via});`;
};

const tokenWhere = (model: RepositoryModel, token: OptimisticConcurrency, expected: string): string => {
  const [first, ...rest] = [...model.ids, token.property];
  const value = (name: string) => name === token.property ? expected : `item.${name}`;
  return `new("${quote(first as string)}", eq, Value.From(${value(first as string)}))${rest.map((name) => `.And("${quote(name)}", eq, Value.From(${value(name)}))`).join("")}`;
};

const occMethods = (model: RepositoryModel, type: string): string => {
  const token = model.optimisticConcurrency;
  if (token === undefined) return "";
  const bump = token.increment ? `\n        item.${token.property} += 1;` : "";
  return `
    public new async Task<Response> Update(${type} item, IWhere? byWhere = null)
    {
        var expected = item.${token.property};${bump}
        var response = await base.Update(item, byWhere ?? ${tokenWhere(model, token, "expected")});
        return response.RowsAffected == 1 ? response : throw new DatasourceException($"Update affected {response.RowsAffected} rows");
    }

    public new async Task<Response> Delete(${type} item, IWhere? byWhere = null)
    {
        var response = await base.Delete(item, byWhere ?? ${tokenWhere(model, token, `item.${token.property}`)});
        return response.RowsAffected == 1 ? response : throw new DatasourceException($"Delete affected {response.RowsAffected} rows");
    }
`;
};

const optionsCall = (model: RepositoryModel): string =>
  dialectsOf(model).length === 0 ? "Options()" : "Options(connection)";

const createReadonly = (model: RepositoryModel, type: string): string => {
  const children = childModels(model);
  if (children.length === 0 && model.enrichments.length === 0)
    return `    public static IReadonlyRepository<${type}> CreateReadonly(IConnection connection) =>\n        new ReadonlyRepository<${type}>(connection, ${optionsCall(model)});`;
  const locals = children.map((child) => `        var ${childField("csharp", child.specName, child.typeName)} = new ${childClass(child.typeName)}(connection);`);
  const enrich = model.enrichments.map((item) => `        ${enrichment(item, "readonly")}`);
  return `    public static IReadonlyRepository<${type}> CreateReadonly(IConnection connection)\n    {\n        var repo = new ReadonlyRepository<${type}>(connection, ${optionsCall(model)});\n${locals.join("\n")}\n${enrich.join("\n")}\n        return repo;\n    }`;
};

export const csharpRepositories = (models: RepositoryModel[]): GenerateEntry[] => {
  const files = models.map((model) => {
    const type = `${model.typeHome === "view" ? "Backend.Types.View" : "Backend.Types.Datasource"}.${model.typeName}`;
    const children = childModels(model);
    const fields = children.map((child) => `    public ${childClass(child.typeName)} ${childField("csharp", child.specName, child.typeName)} { get; }`);
    const lines = [
      ...children.map((child) => `        ${childField("csharp", child.specName, child.typeName)} = new ${childClass(child.typeName)}(connection);`),
      ...model.enrichments.map((item) => `        ${enrichment(item)}`),
    ];
    const constructor = lines.length === 0
      ? `    public ${model.typeName}Repository(IConnection connection) : base(connection, ${optionsCall(model)}) { }`
      : `    public ${model.typeName}Repository(IConnection connection) : base(connection, ${optionsCall(model)})\n    {\n${lines.join("\n")}\n    }`;
    const implementsOcc = model.optimisticConcurrency === undefined ? "" : `, ICRUDRepository<${type}>`;
    const usings = model.optimisticConcurrency === undefined ? "using Datasources;\n" : "using Datasources;\nusing static Datasources.Cmp;\n";
    return content(
      repositoryFile("csharp", model),
      `${usings}\npublic sealed class ${model.typeName}Repository : CRUDRepository<${type}>${implementsOcc}\n{\n${fields.length === 0 ? "" : `${fields.join("\n")}\n\n`}${constructor}\n\n${createReadonly(model, type)}\n${occMethods(model, type)}\n${optionsMethod(model, type)}\n}\n`,
    );
  });
  const register = models.map((model) => {
    const type = `${model.typeName}Repository`;
    return `        factory.Register("${quote(model.entity)}", "${quote(pluralPascal(model.typeName))}", connection => new ${type}(connection), connection => ${type}.CreateReadonly(connection));`;
  });
  return [
    ...files,
    content(
      "RepositoryRegistry.cs",
      `using System.Runtime.CompilerServices;\nusing Datasources;\n\npublic static class RepositoryRegistry\n{\n    [ModuleInitializer]\n    internal static void Bind() => RepositoryFactory.Bind(Register);\n\n    public static void Register(RepositoryFactory factory)\n    {\n${register.join("\n")}\n    }\n}\n`,
    ),
  ];
};
