import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { childField, isLinked, ownedChildren, repositoryFile, type PlannedEnrichment, type RepositoryModel } from "./RepositoryModel.ts";

const rustString = (value: string): string => `"${value.replaceAll("\\", "\\\\").replaceAll("\"", "\\\"")}"`;

const enrichment = (item: PlannedEnrichment): string => {
  const via = childField("rust", item.via, item.viaType);
  if (isLinked(item)) {
    const link = childField("rust", item.link, item.linkType);
    return `repo.add_linked(Enrichment::new(${rustString(item.destination)}, ${rustString(item.source)}, ${rustString(item.relatedKey)}, None, EnrichmentPropertyType::List).linked(Association::new(${rustString(item.parentFk)}, ${rustString(item.relatedFk)})), Arc::new(${via}), Arc::new(${link}) as Arc<dyn ICRUDRepository>, None);`;
  }
  const related = item.relatedProperty === undefined ? "None" : `Some(${rustString(item.relatedProperty)}.into())`;
  const type = item.kind === "list" ? "EnrichmentPropertyType::List" : "EnrichmentPropertyType::Object";
  return `repo.add_enriched(Enrichment::new(${rustString(item.destination)}, ${rustString(item.source)}, ${rustString(item.relatedKey)}, ${related}, ${type}), Arc::new(${via}), None);`;
};

export const rustRepositories = (models: RepositoryModel[]): GenerateEntry[] => {
  const files = models.map((model) => {
    const owned = ownedChildren(model);
    const mapping = model.propertyMappings.length === 0
      ? ""
      : `    options.property_mappings = Some(Arc::new(DictionaryMapper::new([\n${model.propertyMappings.map((pair) => `        (${rustString(pair.from)}.into(), ${rustString(pair.to)}.into()),`).join("\n")}\n    ])));\n`;
    const lines = [
      ...owned.map((child) => `    let ${child.specName} = super::${child.specName}_repository::${child.specName}_repository(Arc::clone(&connection))?;`),
      "    let mut repo = CRUDRepository::new(Arc::clone(&connection), options)?;",
      ...model.enrichments.map((item) => `    ${enrichment(item)}`),
      "    Ok(repo)",
    ];
    const body = model.enrichments.length === 0
      ? "    CRUDRepository::new(connection, options)\n"
      : `${lines.join("\n")}\n`;
    return content(
      repositoryFile("rust", model),
      `use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, Result};\nuse std::sync::Arc;\n\npub fn ${model.specName}_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {\n    let mut options = CRUDRepositoryOptions::default();\n    options.entity = ${rustString(model.entity)}.into();\n    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([\n        (${rustString(model.entity)}.into(), ${rustString(model.table)}.into()),\n    ])));\n${mapping}    options.ids = vec![${model.ids.map((id) => `${rustString(id)}.into()`).join(", ")}];\n    options.read_properties = vec![${model.readProperties.map((name) => `${rustString(name)}.into()`).join(", ")}];\n${body}}\n`,
    );
  });
  return [...files, content("mod.rs", `${models.map((model) => `pub mod ${model.specName}_repository;`).join("\n")}\n`)];
};
