import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { childField, children as childModels, isLinked, pluralSnake, repositoryFile, type PlannedEnrichment, type RepositoryModel } from "./RepositoryModel.ts";

const rustString = (value: string): string => `"${value.replaceAll("\\", "\\\\").replaceAll("\"", "\\\"")}"`;

const uses = `use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Result};\nuse std::sync::Arc;`;

const crudOptions = (model: RepositoryModel): string => {
  const mapping = model.propertyMappings.length === 0
    ? ""
    : `    options.property_mappings = Some(Arc::new(DictionaryMapper::new([\n${model.propertyMappings.map((pair) => `        (${rustString(pair.from)}.into(), ${rustString(pair.to)}.into()),`).join("\n")}\n    ])));\n`;
  return `    let mut options = CRUDRepositoryOptions::default();\n    options.entity = ${rustString(model.entity)}.into();\n    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([\n        (${rustString(model.entity)}.into(), ${rustString(model.table)}.into()),\n    ])));\n${mapping}    options.ids = vec![${model.ids.map((id) => `${rustString(id)}.into()`).join(", ")}];\n    options.read_properties = vec![${model.readProperties.map((name) => `${rustString(name)}.into()`).join(", ")}];`;
};

const readonlyOptions = (model: RepositoryModel): string => {
  const mapping = model.propertyMappings.length === 0
    ? ""
    : `    options.property_mappings = Some(Arc::new(DictionaryMapper::new([\n${model.propertyMappings.map((pair) => `        (${rustString(pair.from)}.into(), ${rustString(pair.to)}.into()),`).join("\n")}\n    ])));\n`;
  return `    let mut options = ReadonlyRepositoryMappingOptions::default();\n    options.entity = ${rustString(model.entity)}.into();\n    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([\n        (${rustString(model.entity)}.into(), ${rustString(model.table)}.into()),\n    ])));\n${mapping}    options.read_properties = vec![${model.readProperties.map((name) => `${rustString(name)}.into()`).join(", ")}];`;
};

const nestedOf = (models: RepositoryModel[]): Set<string> =>
  new Set(models.filter((model) => childModels(model).length > 0).map((model) => model.specName));

const viaClone = (name: string, specName: string, nested: Set<string>): string =>
  nested.has(specName) ? `Arc::clone(&${name}.repo)` : `Arc::clone(&${name})`;

const enrichment = (item: PlannedEnrichment, linked: boolean, nested: Set<string>): string => {
  const via = childField("rust", item.via, item.viaType);
  if (linked && isLinked(item)) {
    const link = childField("rust", item.link, item.linkType);
    return `repo.add_linked(Enrichment::new(${rustString(item.destination)}, ${rustString(item.source)}, ${rustString(item.relatedKey)}, None, EnrichmentPropertyType::List).linked(Association::new(${rustString(item.parentFk)}, ${rustString(item.relatedFk)})), ${viaClone(via, item.via, nested)}, ${viaClone(link, item.link, nested)} as Arc<dyn ICRUDRepository>, None);`;
  }
  const related = item.relatedProperty === undefined ? "None" : `Some(${rustString(item.relatedProperty)}.into())`;
  const type = item.kind === "object" ? "EnrichmentPropertyType::Object" : item.kind === "dictionary" ? "EnrichmentPropertyType::Dictionary" : "EnrichmentPropertyType::List";
  return `repo.add_enriched(Enrichment::new(${rustString(item.destination)}, ${rustString(item.source)}, ${rustString(item.relatedKey)}, ${related}, ${type}), ${viaClone(via, item.via, nested)}, None);`;
};

const childLets = (children: Array<{ specName: string; typeName: string }>, nested: Set<string>): string =>
  children
    .map((child) =>
      nested.has(child.specName)
        ? `        let ${child.specName} = Arc::new(super::${child.specName}_repository::${child.typeName}Repository::new(Arc::clone(&connection))?);`
        : `        let ${child.specName} = Arc::new(super::${child.specName}_repository::${child.specName}_repository(Arc::clone(&connection))?);`,
    )
    .join("\n");

const readonlyChildren = (model: RepositoryModel): Array<{ specName: string; typeName: string }> => {
  const rows = new Map<string, string>();
  for (const item of model.enrichments) rows.set(item.via, item.viaType);
  return [...rows].map(([specName, typeName]) => ({ specName, typeName }));
};

const readonlyFn = (model: RepositoryModel, nested: Set<string>): string => {
  const lets = childLets(readonlyChildren(model), nested);
  const enrich = model.enrichments.map((item) => `        ${enrichment(item, false, nested)}`).join("\n");
  const body = `${readonlyOptions(model)}\n${lets}${lets.length === 0 ? "" : "\n"}        let ${model.enrichments.length === 0 ? "repo" : "mut repo"} = ReadonlyRepository::new(connection, options)?;\n${enrich}${enrich.length === 0 ? "" : "\n"}        Ok(repo)`;
  return `pub fn ${model.specName}_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository> {\n${body}\n}`;
};

const pad = (text: string): string => text.split("\n").map((line) => `    ${line}`).join("\n");

const plain = (model: RepositoryModel, nested: Set<string>): string =>
  `${uses}\n\npub fn ${model.specName}_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {\n${crudOptions(model)}\n    CRUDRepository::new(connection, options)\n}\n\n${readonlyFn(model, nested)}\n`;

const wrapped = (model: RepositoryModel, nested: Set<string>): string => {
  const type = `${model.typeName}Repository`;
  const children = childModels(model);
  const fields = children
    .map((child) =>
      nested.has(child.specName)
        ? `    pub ${child.specName}: Arc<super::${child.specName}_repository::${child.typeName}Repository>,`
        : `    pub ${child.specName}: Arc<CRUDRepository>,`,
    )
    .join("\n");
  const assign = children.map((child) => child.specName).join(", ");
  const enrich = model.enrichments.map((item) => `        ${enrichment(item, true, nested)}`).join("\n");
  return `${uses}\nuse std::ops::Deref;\n\npub struct ${type} {\n    pub repo: Arc<CRUDRepository>,\n${fields}\n}\n\nimpl ${type} {\n    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {\n${pad(crudOptions(model))}\n${childLets(children, nested)}\n        let mut repo = CRUDRepository::new(Arc::clone(&connection), options)?;\n${enrich}\n        Ok(Self { repo: Arc::new(repo), ${assign} })\n    }\n}\n\nimpl Deref for ${type} {\n    type Target = CRUDRepository;\n    fn deref(&self) -> &Self::Target {\n        self.repo.as_ref()\n    }\n}\n\npub fn ${model.specName}_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {\n    Arc::try_unwrap(${type}::new(connection)?.repo).map_err(|_| datasources::DatasourceError::message("repository"))\n}\n\n${readonlyFn(model, nested)}\n`;
};

export const rustRepositories = (models: RepositoryModel[]): GenerateEntry[] => {
  const nested = nestedOf(models);
  const files = models.map((model) =>
    content(repositoryFile("rust", model), childModels(model).length === 0 ? plain(model, nested) : wrapped(model, nested)),
  );
  const register = models.map((model) => {
    const module = `${model.specName}_repository`;
    return `    factory.register_named(\n        ${rustString(model.entity)},\n        ${rustString(pluralSnake(model.specName))},\n        |connection| Arc::new(super::${module}::${model.specName}_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository>,\n        |connection| Arc::new(super::${module}::${model.specName}_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository>,\n    );`;
  });
  const methods = models.map((model) => {
    const name = pluralSnake(model.specName);
    return `    fn ${name}(&self) -> Arc<dyn ICRUDRepository>;`;
  });
  const impls = models.map((model) => {
    const name = pluralSnake(model.specName);
    return `    fn ${name}(&self) -> Arc<dyn ICRUDRepository> {\n        bind();\n        self.create_repository_by_name(${rustString(name)}, None).expect(${rustString(name)})\n    }`;
  });
  const registry = `use datasources::{Datasource, ICRUDRepository, IReadonlyRepository, RepositoryFactory};\nuse std::sync::{Arc, Once};\n\npub fn register(factory: &RepositoryFactory) {\n${register.join("\n")}\n}\n\nfn bind() {\n    static ONCE: Once = Once::new();\n    ONCE.call_once(|| RepositoryFactory::bind(register));\n}\n\npub trait Repositories {\n${methods.join("\n")}\n}\n\nimpl Repositories for dyn Datasource {\n${impls.join("\n")}\n}\n`;
  return [
    ...files,
    content("repository_registry.rs", registry),
    content("mod.rs", `${models.map((model) => `pub mod ${model.specName}_repository;`).join("\n")}\npub mod repository_registry;\n`),
  ];
};
