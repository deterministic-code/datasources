import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { childField, isLinked, ownedChildren, repositoryFile, type PlannedEnrichment, type RepositoryModel } from "./RepositoryModel.ts";

const rustString = (value: string): string => `"${value.replaceAll("\\", "\\\\").replaceAll("\"", "\\\"")}"`;

const uses = `use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, Result};\nuse std::sync::Arc;`;

const options = (model: RepositoryModel): string => {
  const mapping = model.propertyMappings.length === 0
    ? ""
    : `    options.property_mappings = Some(Arc::new(DictionaryMapper::new([\n${model.propertyMappings.map((pair) => `        (${rustString(pair.from)}.into(), ${rustString(pair.to)}.into()),`).join("\n")}\n    ])));\n`;
  return `    let mut options = CRUDRepositoryOptions::default();\n    options.entity = ${rustString(model.entity)}.into();\n    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([\n        (${rustString(model.entity)}.into(), ${rustString(model.table)}.into()),\n    ])));\n${mapping}    options.ids = vec![${model.ids.map((id) => `${rustString(id)}.into()`).join(", ")}];\n    options.read_properties = vec![${model.readProperties.map((name) => `${rustString(name)}.into()`).join(", ")}];`;
};

const enrichment = (item: PlannedEnrichment): string => {
  const via = childField("rust", item.via, item.viaType);
  if (isLinked(item)) {
    const link = childField("rust", item.link, item.linkType);
    return `repo.add_linked(Enrichment::new(${rustString(item.destination)}, ${rustString(item.source)}, ${rustString(item.relatedKey)}, None, EnrichmentPropertyType::List).linked(Association::new(${rustString(item.parentFk)}, ${rustString(item.relatedFk)})), Arc::clone(&${via}), Arc::clone(&${link}) as Arc<dyn ICRUDRepository>, None);`;
  }
  const related = item.relatedProperty === undefined ? "None" : `Some(${rustString(item.relatedProperty)}.into())`;
  const type = item.kind === "list" ? "EnrichmentPropertyType::List" : "EnrichmentPropertyType::Object";
  return `repo.add_enriched(Enrichment::new(${rustString(item.destination)}, ${rustString(item.source)}, ${rustString(item.relatedKey)}, ${related}, ${type}), Arc::clone(&${via}), None);`;
};

const plain = (model: RepositoryModel): string =>
  `${uses}\n\npub fn ${model.specName}_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {\n${options(model)}\n    CRUDRepository::new(connection, options)\n}\n`;

const pad = (text: string): string => text.split("\n").map((line) => `    ${line}`).join("\n");

const wrapped = (model: RepositoryModel): string => {
  const type = `${model.typeName}Repository`;
  const owned = ownedChildren(model);
  const fields = owned.map((child) => `    pub ${child.specName}: Arc<CRUDRepository>,`).join("\n");
  const childLets = owned.map((child) => `        let ${child.specName} = Arc::new(super::${child.specName}_repository::${child.specName}_repository(Arc::clone(&connection))?);`).join("\n");
  const assign = owned.map((child) => child.specName).join(", ");
  const enrich = model.enrichments.map((item) => `        ${enrichment(item)}`).join("\n");
  return `${uses}\nuse std::ops::{Deref, DerefMut};\n\npub struct ${type} {\n    repo: CRUDRepository,\n${fields}\n}\n\nimpl ${type} {\n    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {\n${pad(options(model))}\n${childLets}\n        let mut repo = CRUDRepository::new(Arc::clone(&connection), options)?;\n${enrich}\n        Ok(Self { repo, ${assign} })\n    }\n}\n\nimpl Deref for ${type} {\n    type Target = CRUDRepository;\n    fn deref(&self) -> &Self::Target {\n        &self.repo\n    }\n}\n\nimpl DerefMut for ${type} {\n    fn deref_mut(&mut self) -> &mut Self::Target {\n        &mut self.repo\n    }\n}\n\npub fn ${model.specName}_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {\n    Ok(${type}::new(connection)?.repo)\n}\n`;
};

export const rustRepositories = (models: RepositoryModel[]): GenerateEntry[] => {
  const files = models.map((model) =>
    content(repositoryFile("rust", model), ownedChildren(model).length === 0 ? plain(model) : wrapped(model)),
  );
  return [...files, content("mod.rs", `${models.map((model) => `pub mod ${model.specName}_repository;`).join("\n")}\n`)];
};
