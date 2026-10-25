import { createCasingStrategy, type ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { childField, children as childModels, isLinked, pluralSnake, registryFile, repositoryFile, rustIdent, rustModLine, type OptimisticConcurrency, type PlannedEnrichment, type RepositoryModel, type RepositoryOptionsModel } from "./RepositoryModel.ts";
import { dialectsOf, pad, quote, writeList } from "./writeLists.ts";

type Options = RepositoryOptionsModel & { entity: string };

const rustString = (value: string): string => `"${quote(value)}"`;

const uses = (model: RepositoryModel): string =>
  dialectsOf(model).length === 0
    ? `use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Result};\nuse std::sync::Arc;`
    : `use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Result};\nuse std::sync::Arc;`;

const rustFill = (model: Options): string => {
  const mapping = model.propertyMappings.length === 0
    ? ""
    : `options.property_mappings = Some(Arc::new(DictionaryMapper::new([\n${model.propertyMappings.map((pair) => `    (${rustString(pair.from)}.into(), ${rustString(pair.to)}.into()),`).join("\n")}\n])));\n`;
  return `let mut options = CRUDRepositoryOptions::default();\noptions.entity = ${rustString(model.entity)}.into();\noptions.entity_mapper = Some(Arc::new(DictionaryMapper::new([\n    (${rustString(model.entity)}.into(), ${rustString(model.table)}.into()),\n])));\n${mapping}options.ids = ${writeList("rust", model.ids)};\noptions.read_properties = ${writeList("rust", model.readProperties)};\noptions.create_properties = Some(${writeList("rust", model.createProperties)});\noptions.update_properties = Some(${writeList("rust", model.updateProperties)});`;
};

const rustBlock = (model: RepositoryModel): string => {
  const dialects = dialectsOf(model);
  return dialects.length === 0
    ? rustFill(model)
    : `let options = CRUDRepositoryBuilder::new()
    .add({
${pad(rustFill(model), 8)}
        options
    })${dialects
      .map(([engine, planned]) => `
    .add_for(${rustString(engine)}, {
${pad(rustFill({ entity: model.entity, ...planned }), 8)}
        options
    })`)
      .join("")}
    .for_connection(connection.as_ref())?;`;
};

const crudOptions = (model: RepositoryModel): string => pad(rustBlock(model), 4);

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

const fileStem = (specName: string, casing: ICasingStrategy): string =>
  casing.convertFileName(`${specName}_repository`);

const moduleStem = (specName: string, casing: ICasingStrategy): string => rustIdent(fileStem(specName, casing));

const repoFn = (specName: string, casing: ICasingStrategy): string =>
  casing.convertFields(`${specName}_repository`);

const readonlyFnName = (specName: string, casing: ICasingStrategy): string =>
  casing.convertFields(`${specName}_readonly_repository`);

const enrichment = (item: PlannedEnrichment, linked: boolean, nested: Set<string>, casing: ICasingStrategy): string => {
  const via = childField(casing, item.via);
  if (linked && isLinked(item)) {
    const link = childField(casing, item.link);
    return `repo.add_linked(Enrichment::new(${rustString(item.destination)}, ${rustString(item.source)}, ${rustString(item.relatedKey)}, None, EnrichmentPropertyType::List).linked(Association::new(${rustString(item.parentFk)}, ${rustString(item.relatedFk)})), ${viaClone(via, item.via, nested)}, ${viaClone(link, item.link, nested)} as Arc<dyn ICRUDRepository>, None);`;
  }
  const related = item.relatedProperty === undefined ? "None" : `Some(${rustString(item.relatedProperty)}.into())`;
  const type = item.kind === "object" ? "EnrichmentPropertyType::Object" : item.kind === "dictionary" ? "EnrichmentPropertyType::Dictionary" : "EnrichmentPropertyType::List";
  return `repo.add_enriched(Enrichment::new(${rustString(item.destination)}, ${rustString(item.source)}, ${rustString(item.relatedKey)}, ${related}, ${type}), ${viaClone(via, item.via, nested)}, None);`;
};

const childLets = (children: Array<{ specName: string; typeName: string }>, nested: Set<string>, casing: ICasingStrategy): string =>
  children
    .map((child) => {
      const field = childField(casing, child.specName);
      const module = moduleStem(child.specName, casing);
      return nested.has(child.specName)
        ? `        let ${field} = Arc::new(super::${module}::${child.typeName}Repository::new(Arc::clone(&connection))?);`
        : `        let ${field} = Arc::new(super::${module}::${repoFn(child.specName, casing)}(Arc::clone(&connection))?);`;
    })
    .join("\n");

const readonlyChildren = (model: RepositoryModel): Array<{ specName: string; typeName: string }> => {
  const rows = new Map<string, string>();
  for (const item of model.enrichments) rows.set(item.via, item.viaType);
  return [...rows].map(([specName, typeName]) => ({ specName, typeName }));
};

const readonlyFn = (model: RepositoryModel, nested: Set<string>, casing: ICasingStrategy): string => {
  const lets = childLets(readonlyChildren(model), nested, casing);
  const enrich = model.enrichments.map((item) => `        ${enrichment(item, false, nested, casing)}`).join("\n");
  const body = `${readonlyOptions(model)}\n${lets}${lets.length === 0 ? "" : "\n"}        let ${model.enrichments.length === 0 ? "repo" : "mut repo"} = ReadonlyRepository::new(connection, options)?;\n${enrich}${enrich.length === 0 ? "" : "\n"}        Ok(repo)`;
  return `pub fn ${readonlyFnName(model.specName, casing)}(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository> {\n${body}\n}`;
};

const rustTokenWhere = (model: RepositoryModel, token: OptimisticConcurrency): string => {
  const [first, ...rest] = [...model.ids, token.property] as [string, ...string[]];
  const value = (name: string) => name === token.property ? "expected.clone()" : `item.get(${rustString(name)}).cloned().ok_or_else(|| DatasourceError::message(${rustString(name)}))?`;
  return `        let expected = item.get(${rustString(token.property)}).cloned().ok_or_else(|| DatasourceError::message(${rustString(token.property)}))?;\n        let clause = Where::binary(${rustString(first)}, eq, ${value(first)})${rest.map((name) => `.and(${rustString(name)}, eq, ${value(name)})`).join("")};`;
};

const rustOcc = (model: RepositoryModel): string => {
  const token = model.optimisticConcurrency;
  if (token === undefined) return "";
  const name = `${model.typeName}Optimistic`;
  const bump = token.increment
    ? `        if let Some(value) = item.get(${rustString(token.property)}).cloned() {\n            item.insert(${rustString(token.property)}.into(), match value {\n                Value::Int(n) => Value::Int(n + 1),\n                Value::Float(n) => Value::Float(n + 1.0),\n                other => other,\n            });\n        }\n`
    : "";
  const where = rustTokenWhere(model, token);
  return `
use datasources::{async_trait, CommandMiddleware, DatasourceError, IReadonlyRepository, IRepository, IdKey, IWhere, Parameter, ReadInput, Response, Value, Where, eq};
use futures_core::Stream;
use std::any::TypeId;
use std::pin::Pin;

pub struct ${name} {
    repo: CRUDRepository,
}

impl ${name} {
    pub fn new(repo: CRUDRepository) -> Self {
        Self { repo }
    }
}

impl IRepository for ${name} {
    fn entity_type(&self) -> TypeId {
        self.repo.entity_type()
    }
}

#[async_trait]
impl IReadonlyRepository for ${name} {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[Parameter]>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<datasources::Row>> {
        self.repo.read_single(query, parameters, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn ReadInput>,
        parameters: Option<&'a [Parameter]>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<datasources::Row>> + Send + 'a>> {
        self.repo.read(query, parameters, options)
    }
}

#[async_trait]
impl ICRUDRepository for ${name} {
    async fn create(&self, item: &mut datasources::Row) -> Result<datasources::Row> {
        self.repo.create(item).await
    }

    async fn update(&self, item: &mut datasources::Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
${where}
${bump}        let response = self.repo.update(item, Some(by_where.unwrap_or(&clause))).await?;
        if response.rows_affected != 1 {
            return Err(DatasourceError::message(format!("Update affected {} rows", response.rows_affected)));
        }
        Ok(response)
    }

    async fn delete(&self, item: &datasources::Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
${where}
        let response = self.repo.delete(item, Some(by_where.unwrap_or(&clause))).await?;
        if response.rows_affected != 1 {
            return Err(DatasourceError::message(format!("Delete affected {} rows", response.rows_affected)));
        }
        Ok(response)
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<datasources::Row>> {
        self.repo.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.repo.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.repo.add_middleware(middleware)
    }
}
`;
};

const plain = (model: RepositoryModel, nested: Set<string>, casing: ICasingStrategy): string =>
  `${uses(model)}\n\npub fn ${repoFn(model.specName, casing)}(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {\n${crudOptions(model)}\n    CRUDRepository::new(connection, options)\n}\n\n${readonlyFn(model, nested, casing)}\n${rustOcc(model)}`;

const wrapped = (model: RepositoryModel, nested: Set<string>, casing: ICasingStrategy): string => {
  const type = `${model.typeName}Repository`;
  const children = childModels(model);
  const fields = children
    .map((child) => {
      const field = childField(casing, child.specName);
      const module = moduleStem(child.specName, casing);
      return nested.has(child.specName)
        ? `    pub ${field}: Arc<super::${module}::${child.typeName}Repository>,`
        : `    pub ${field}: Arc<CRUDRepository>,`;
    })
    .join("\n");
  const assign = children.map((child) => childField(casing, child.specName)).join(", ");
  const enrich = model.enrichments.map((item) => `        ${enrichment(item, true, nested, casing)}`).join("\n");
  return `${uses(model)}\nuse std::ops::Deref;\n\npub struct ${type} {\n    pub repo: Arc<CRUDRepository>,\n${fields}\n}\n\nimpl ${type} {\n    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {\n${pad(crudOptions(model), 4)}\n${childLets(children, nested, casing)}\n        let mut repo = CRUDRepository::new(Arc::clone(&connection), options)?;\n${enrich}\n        Ok(Self { repo: Arc::new(repo), ${assign} })\n    }\n}\n\nimpl Deref for ${type} {\n    type Target = CRUDRepository;\n    fn deref(&self) -> &Self::Target {\n        self.repo.as_ref()\n    }\n}\n\npub fn ${repoFn(model.specName, casing)}(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {\n    Arc::try_unwrap(${type}::new(connection)?.repo).map_err(|_| datasources::DatasourceError::message("repository"))\n}\n\n${readonlyFn(model, nested, casing)}\n${rustOcc(model)}`;
};

export const rustRepositories = (models: RepositoryModel[], settings: Record<string, string>): GenerateEntry[] => {
  const casing = createCasingStrategy("rust", settings);
  const nested = nestedOf(models);
  const files = models.map((model) =>
    content(repositoryFile("rs", model.specName, casing), childModels(model).length === 0 ? plain(model, nested, casing) : wrapped(model, nested, casing)),
  );
  const register = models.map((model) => {
    const module = moduleStem(model.specName, casing);
    const fn = repoFn(model.specName, casing);
    const crud = model.optimisticConcurrency === undefined
      ? `Arc::new(super::${module}::${fn}(connection).expect("repository")) as Arc<dyn ICRUDRepository>`
      : `Arc::new(super::${module}::${model.typeName}Optimistic::new(super::${module}::${fn}(connection).expect("repository"))) as Arc<dyn ICRUDRepository>`;
    return `    factory.register_named(\n        ${rustString(model.entity)},\n        ${rustString(pluralSnake(model.specName))},\n        |connection| ${crud},\n        |connection| Arc::new(super::${module}::${readonlyFnName(model.specName, casing)}(connection).expect("repository")) as Arc<dyn IReadonlyRepository>,\n    );`;
  });
  const methods = models.map((model) => {
    const name = pluralSnake(model.specName);
    return `    fn ${name}(&self) -> Arc<dyn ICRUDRepository>;`;
  });
  const impls = models.map((model) => {
    const name = pluralSnake(model.specName);
    return `    fn ${name}(&self) -> Arc<dyn ICRUDRepository> {\n        self.create_repository_by_name(${rustString(name)}, None).expect(${rustString(name)})\n    }`;
  });
  const registry = `use datasources::{Datasource, ICRUDRepository, IReadonlyRepository, RepositoryFactory};\nuse std::sync::Arc;\n\npub fn register(factory: &RepositoryFactory) {\n${register.join("\n")}\n}\n\n#[ctor::ctor]\nfn bind() {\n    RepositoryFactory::bind(register);\n}\n\npub trait Repositories {\n${methods.join("\n")}\n}\n\nimpl Repositories for dyn Datasource {\n${impls.join("\n")}\n}\n`;
  return [
    ...files,
    content(registryFile("rs", casing), registry),
    content("mod.rs", `${models.map((model) => rustModLine(fileStem(model.specName, casing))).join("\n")}\n${rustModLine(casing.convertFileName("repository_registry"))}\n`),
  ];
};
