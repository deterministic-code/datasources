import { createCasingStrategy, type ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { authoredViewTypesOf, datasourceTypesOf } from "@deterministic-code/generators-common/spec-types";
import type { IDeterministic } from "@deterministic-code/deterministic-generators-common/parser";
import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { childField, children as childModels, isLinked, pluralSnake, registryFile, repositoryFile, rustIdent, rustModLine, type OptimisticConcurrency, type PlannedEnrichment, type RepositoryModel, type RepositoryOptionsModel } from "./RepositoryModel.ts";
import { schemaPairs } from "./schemaPairs.ts";
import { dialectsOf, pad, quote, writeList } from "./writeLists.ts";

type Options = RepositoryOptionsModel & { entity: string };

const rustString = (value: string): string => `"${quote(value)}"`;

const uses = (model: RepositoryModel): string => {
  const builder = dialectsOf(model).length === 0 ? "" : "CRUDRepositoryBuilder, ";
  const value = model.createZeros.length === 0 ? "" : "Value, ";
  return `use datasources::{Association, Connection, CRUDRepository, ${builder}CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, ${value}Result};\nuse super::super::${model.typeName};\nuse std::collections::HashMap;\nuse std::sync::Arc;`;
};

const childNames = (model: RepositoryModel): PlannedEnrichment[] =>
  model.enrichments.filter((item) => item.kind === "list" || item.kind === "linked");

const rustColumnMap = (columns: Array<{ from: string; to: string }>, indent: string): string =>
  `HashMap::from([\n${columns.map((pair) => `${indent}    (${rustString(pair.from)}.into(), ${rustString(pair.to)}.into()),`).join("\n")}\n${indent}])`;

const rustSchemaAssign = (model: RepositoryModel, spec: IDeterministic | undefined, casing: ICasingStrategy): string =>
  `options.schema = Schema::new(${rustColumnMap(schemaPairs(model, spec, casing), "    ")});\n`;

const rustSchemaObjects = (models: RepositoryModel[]): string => {
  const names = [...new Set(models.flatMap((model) => [model.typeName, ...childNames(model).map((item) => item.viaType)]))];
  return `use datasources::SchemaObject;\nuse super::*;\n\n${names.map((name) => `impl SchemaObject for ${name} {}\n`).join("\n")}`;
};

const rustZeros = (zeros: RepositoryModel["createZeros"]): string =>
  zeros.length === 0
    ? ""
    : `.map(|repo| repo.with_create_zeros([\n${zeros
        .map((zero) =>
          zero.kind === "boolean"
            ? `        (${rustString(zero.name)}.into(), Value::from(false)),`
            : `        (${rustString(zero.name)}.into(), Value::from("")),`,
        )
        .join("\n")}\n    ].into_iter().collect()))`;

const rustFill = (model: RepositoryModel, planned: Options, spec: IDeterministic | undefined, casing: ICasingStrategy): string => {
  const mapping = planned.propertyMappings.length === 0
    ? ""
    : `options.property_mappings = Some(Arc::new(DictionaryMapper::new([\n${planned.propertyMappings.map((pair) => `    (${rustString(pair.from)}.into(), ${rustString(pair.to)}.into()),`).join("\n")}\n])));\n`;
  return `let mut options = CRUDRepositoryOptions::default();\noptions.entity = ${rustString(planned.entity)}.into();\noptions.entity_mapper = Some(Arc::new(DictionaryMapper::new([\n    (${rustString(planned.entity)}.into(), ${rustString(planned.table)}.into()),\n])));\n${mapping}${rustSchemaAssign({ ...model, ...planned }, spec, casing)}options.ids = ${writeList("rust", planned.ids)};\noptions.read_properties = ${writeList("rust", planned.readProperties)};\noptions.create_properties = Some(${writeList("rust", planned.createProperties)});\noptions.update_properties = Some(${writeList("rust", planned.updateProperties)});`;
};

const rustBlock = (model: RepositoryModel, spec: IDeterministic | undefined, casing: ICasingStrategy): string => {
  const dialects = dialectsOf(model);
  return dialects.length === 0
    ? rustFill(model, model, spec, casing)
    : `let options = CRUDRepositoryBuilder::new()
    .add({
${pad(rustFill(model, model, spec, casing), 8)}
        options
    })${dialects
      .map(([engine, planned]) => `
    .add_for(${rustString(engine)}, {
${pad(rustFill(model, { entity: model.entity, ...planned }, spec, casing), 8)}
        options
    })`)
      .join("")}
    .for_connection(connection.as_ref())?;`;
};

const crudOptions = (model: RepositoryModel, spec: IDeterministic | undefined, casing: ICasingStrategy): string =>
  pad(rustBlock(model, spec, casing), 4);

const readonlyOptions = (model: RepositoryModel, spec: IDeterministic | undefined, casing: ICasingStrategy): string => {
  const mapping = model.propertyMappings.length === 0
    ? ""
    : `    options.property_mappings = Some(Arc::new(DictionaryMapper::new([\n${model.propertyMappings.map((pair) => `        (${rustString(pair.from)}.into(), ${rustString(pair.to)}.into()),`).join("\n")}\n    ])));\n`;
  return `    let mut options = ReadonlyRepositoryMappingOptions::default();\n    options.entity = ${rustString(model.entity)}.into();\n    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([\n        (${rustString(model.entity)}.into(), ${rustString(model.table)}.into()),\n    ])));\n${mapping}    ${rustSchemaAssign(model, spec, casing).replace(/^/gm, "    ").trimStart()}    options.read_properties = vec![${model.readProperties.map((name) => `${rustString(name)}.into()`).join(", ")}];`;
};

const viaClone = (name: string): string => `Arc::clone(&${name})`;

const fileStem = (specName: string, casing: ICasingStrategy): string =>
  casing.convertFileName(`${specName}_repository`);

const moduleStem = (specName: string, casing: ICasingStrategy): string => rustIdent(fileStem(specName, casing));

const repoFn = (specName: string, casing: ICasingStrategy): string =>
  casing.convertFields(`${specName}_repository`);

const rowRepoFn = (specName: string, casing: ICasingStrategy): string =>
  casing.convertFields(`${specName}_row_repository`);

const readonlyFnName = (specName: string, casing: ICasingStrategy): string =>
  casing.convertFields(`${specName}_readonly_repository`);

const enrichment = (item: PlannedEnrichment, linked: boolean, casing: ICasingStrategy): string => {
  const via = childField(casing, item.via);
  if (linked && isLinked(item)) {
    const link = childField(casing, item.link);
    return `repo.add_linked(Enrichment::new(${rustString(item.destination)}, ${rustString(item.source)}, ${rustString(item.relatedKey)}, None, EnrichmentPropertyType::List).linked(Association::new(${rustString(item.parentFk)}, ${rustString(item.relatedFk)})), ${viaClone(via)}, ${viaClone(link)} as Arc<dyn ICRUDRepository>, None);`;
  }
  const related = item.relatedProperty === undefined ? "None" : `Some(${rustString(item.relatedProperty)}.into())`;
  const type = item.kind === "object" ? "EnrichmentPropertyType::Object" : item.kind === "dictionary" ? "EnrichmentPropertyType::Dictionary" : "EnrichmentPropertyType::List";
  return `repo.add_enriched(Enrichment::new(${rustString(item.destination)}, ${rustString(item.source)}, ${rustString(item.relatedKey)}, ${related}, ${type}), ${viaClone(via)}, None);`;
};

const childLets = (children: Array<{ specName: string; typeName: string }>, casing: ICasingStrategy): string =>
  children
    .map((child) => {
      const field = childField(casing, child.specName);
      const module = moduleStem(child.specName, casing);
      return `        let ${field} = Arc::new(super::${module}::${rowRepoFn(child.specName, casing)}(Arc::clone(&connection))?);`;
    })
    .join("\n");

const readonlyChildren = (model: RepositoryModel): Array<{ specName: string; typeName: string }> => {
  const rows = new Map<string, string>();
  for (const item of model.enrichments) rows.set(item.via, item.viaType);
  return [...rows].map(([specName, typeName]) => ({ specName, typeName }));
};

const readonlyFn = (model: RepositoryModel, spec: IDeterministic | undefined, casing: ICasingStrategy): string => {
  const lets = childLets(readonlyChildren(model), casing);
  const enrich = model.enrichments.map((item) => `        ${enrichment(item, false, casing)}`).join("\n");
  const body = `${readonlyOptions(model, spec, casing)}\n${lets}${lets.length === 0 ? "" : "\n"}        let ${model.enrichments.length === 0 ? "repo" : "mut repo"} = ReadonlyRepository::<${model.typeName}>::new(connection, options)?;\n${enrich}${enrich.length === 0 ? "" : "\n"}        Ok(repo)`;
  return `pub fn ${readonlyFnName(model.specName, casing)}(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<${model.typeName}>> {\n${body}\n}`;
};

const rustTokenWhere = (model: RepositoryModel, token: OptimisticConcurrency): string => {
  const [first, ...rest] = [...model.ids, token.property] as [string, ...string[]];
  const value = (name: string) => name === token.property ? "expected.clone()" : `stored.get(${rustString(name)}).cloned().ok_or_else(|| DatasourceError::message(${rustString(name)}))?`;
  return `        let mut stored = item.to_schema(self.repo.schema())?;\n        let expected = stored.get(${rustString(token.property)}).cloned().ok_or_else(|| DatasourceError::message(${rustString(token.property)}))?;\n        let clause = Where::binary(${rustString(first)}, eq, ${value(first)})${rest.map((name) => `.and(${rustString(name)}, eq, ${value(name)})`).join("")};`;
};

const rustOcc = (model: RepositoryModel): string => {
  const token = model.optimisticConcurrency;
  if (token === undefined) return "";
  const name = `${model.typeName}Optimistic`;
  const entity = model.typeName;
  const bump = token.increment
    ? `        if let Some(value) = stored.get(${rustString(token.property)}).cloned() {\n            stored.insert(${rustString(token.property)}.into(), match value {\n                Value::Int(n) => Value::Int(n + 1),\n                Value::Float(n) => Value::Float(n + 1.0),\n                other => other,\n            });\n        }\n`
    : "";
  const where = rustTokenWhere(model, token);
  return `
use datasources::{async_trait, CommandMiddleware, DatasourceError, FromSchema, ICRUDRepository, IReadonlyRepository, IRepository, IdKey, IWhere, ReadInput, Response, Result, Value, Where, eq};
use futures_core::Stream;
use std::any::TypeId;
use std::pin::Pin;

pub struct ${name} {
    repo: CRUDRepository<${entity}>,
}

impl ${name} {
    pub fn new(repo: CRUDRepository<${entity}>) -> Self {
        Self { repo }
    }

    pub async fn update(&self, item: ${entity}) -> Result<Response> {
        ICRUDRepository::update(self, item, None).await
    }

    pub async fn delete(&self, item: ${entity}) -> Result<Response> {
        ICRUDRepository::delete(self, item, None).await
    }
}

impl IRepository for ${name} {
    fn entity_type(&self) -> TypeId {
        self.repo.entity_type()
    }
}

#[async_trait]
impl IReadonlyRepository<${entity}> for ${name} {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<${entity}>>> {
        Some(self)
    }

    async fn read_single(&self, query: &dyn ReadInput) -> Result<Option<${entity}>> {
        self.repo.read_single(query).await
    }

    fn read<'a>(&'a self, query: &'a dyn ReadInput) -> Pin<Box<dyn Stream<Item = Result<${entity}>> + Send + 'a>> {
        self.repo.read(query)
    }
}

#[async_trait]
impl ICRUDRepository<${entity}> for ${name} {
    async fn create(&self, item: ${entity}) -> Result<${entity}> {
        self.repo.create(item).await
    }

    async fn update(&self, item: ${entity}, by_where: Option<&dyn IWhere>) -> Result<Response> {
${where}
${bump}        let item = ${entity}::from_schema(stored, self.repo.schema())?;
        let response = ICRUDRepository::update(&self.repo, item, Some(by_where.unwrap_or(&clause))).await?;
        if response.rows_affected != 1 {
            return Err(DatasourceError::message(format!("Update affected {} rows", response.rows_affected)));
        }
        Ok(response)
    }

    async fn delete(&self, item: ${entity}, by_where: Option<&dyn IWhere>) -> Result<Response> {
${where}
        let response = ICRUDRepository::delete(&self.repo, item, Some(by_where.unwrap_or(&clause))).await?;
        if response.rows_affected != 1 {
            return Err(DatasourceError::message(format!("Delete affected {} rows", response.rows_affected)));
        }
        Ok(response)
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<${entity}>> {
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

const rustRowFn = (model: RepositoryModel, spec: IDeterministic | undefined, casing: ICasingStrategy): string => {
  const children = childModels(model);
  const lets = childLets(children, casing);
  const enrich = model.enrichments.map((item) => `        ${enrichment(item, true, casing)}`).join("\n");
  const mut = model.enrichments.length === 0 ? "" : "mut ";
  const body = model.enrichments.length === 0
    ? `${crudOptions(model, spec, casing)}\n    CRUDRepository::<datasources::Row>::new(connection, options)`
    : `${crudOptions(model, spec, casing)}\n${lets}\n        let ${mut}repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;\n${enrich}\n        Ok(repo)`;
  return `pub fn ${rowRepoFn(model.specName, casing)}(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {\n${body}\n}`;
};

const plain = (model: RepositoryModel, spec: IDeterministic | undefined, casing: ICasingStrategy): string =>
  `${uses(model)}\n\npub fn ${repoFn(model.specName, casing)}(connection: Arc<dyn Connection>) -> Result<CRUDRepository<${model.typeName}>> {\n${crudOptions(model, spec, casing)}\n    CRUDRepository::<${model.typeName}>::new(connection, options)${rustZeros(model.createZeros)}\n}\n\n${rustRowFn(model, spec, casing)}\n\n${readonlyFn(model, spec, casing)}\n${rustOcc(model)}`;

const wrapped = (model: RepositoryModel, spec: IDeterministic | undefined, casing: ICasingStrategy): string => {
  const type = `${model.typeName}Repository`;
  const children = childModels(model);
  const fields = children
    .map((child) => {
      const field = childField(casing, child.specName);
      const module = moduleStem(child.specName, casing);
      return `    pub ${field}: Arc<CRUDRepository>,`;
    })
    .join("\n");
  const assign = children.map((child) => childField(casing, child.specName)).join(", ");
  const enrich = model.enrichments.map((item) => `        ${enrichment(item, true, casing)}`).join("\n");
  return `${uses(model)}\nuse std::ops::Deref;\n\npub struct ${type} {\n    pub repo: Arc<CRUDRepository<${model.typeName}>>,\n${fields}\n}\n\nimpl ${type} {\n    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {\n${pad(crudOptions(model, spec, casing), 4)}\n${childLets(children, casing)}\n        let mut repo = CRUDRepository::<${model.typeName}>::new(Arc::clone(&connection), options)${rustZeros(model.createZeros)}?;\n${enrich}\n        Ok(Self { repo: Arc::new(repo), ${assign} })\n    }\n}\n\nimpl Deref for ${type} {\n    type Target = CRUDRepository<${model.typeName}>;\n    fn deref(&self) -> &Self::Target {\n        self.repo.as_ref()\n    }\n}\n\npub fn ${repoFn(model.specName, casing)}(connection: Arc<dyn Connection>) -> Result<CRUDRepository<${model.typeName}>> {\n    Arc::try_unwrap(${type}::new(connection)?.repo).map_err(|_| datasources::DatasourceError::message("repository"))\n}\n\n${rustRowFn(model, spec, casing)}\n\n${readonlyFn(model, spec, casing)}\n${rustOcc(model)}`;
};

export const rustRepositories = (
  models: RepositoryModel[],
  settings: Record<string, string>,
  spec?: IDeterministic,
): GenerateEntry[] => {
  const casing = createCasingStrategy("rust", settings);
  const typeStems = spec === undefined
    ? models.map((model) => casing.convertFileName(model.specName))
    : [...datasourceTypesOf(spec), ...authoredViewTypesOf(spec)].map((type) => casing.convertFileName(type.name));
  const files = models.map((model) =>
    content(repositoryFile("rs", model.specName, casing), childModels(model).length === 0 ? plain(model, spec, casing) : wrapped(model, spec, casing)),
  );
  const register = models.map((model) => {
    const module = moduleStem(model.specName, casing);
    const fn = repoFn(model.specName, casing);
    const crud = model.optimisticConcurrency === undefined
      ? `Arc::new(super::${module}::${fn}(connection).expect("repository")) as Arc<dyn ICRUDRepository<${model.typeName}>>`
      : `Arc::new(super::${module}::${model.typeName}Optimistic::new(super::${module}::${fn}(connection).expect("repository"))) as Arc<dyn ICRUDRepository<${model.typeName}>>`;
    return `    factory.register_named(\n        ${rustString(model.entity)},\n        ${rustString(pluralSnake(model.specName))},\n        |connection| ${crud},\n        |connection| Arc::new(super::${module}::${readonlyFnName(model.specName, casing)}(connection).expect("repository")) as Arc<dyn IReadonlyRepository<${model.typeName}>>,\n    );`;
  });
  const methods = models.map((model) => {
    const name = pluralSnake(model.specName);
    const module = moduleStem(model.specName, casing);
    const fn = repoFn(model.specName, casing);
    const returned = model.optimisticConcurrency === undefined
      ? `Arc<CRUDRepository<${model.typeName}>>`
      : `Arc<super::${module}::${model.typeName}Optimistic>`;
    return `    fn ${name}(&self) -> ${returned};`;
  });
  const impls = models.map((model) => {
    const name = pluralSnake(model.specName);
    const module = moduleStem(model.specName, casing);
    const fn = repoFn(model.specName, casing);
    const built = `super::${module}::${fn}(self.bind_connection(None)).expect(${rustString(name)})`;
    const value = model.optimisticConcurrency === undefined
      ? `Arc::new(${built})`
      : `Arc::new(super::${module}::${model.typeName}Optimistic::new(${built}))`;
    const returned = model.optimisticConcurrency === undefined
      ? `Arc<CRUDRepository<${model.typeName}>>`
      : `Arc<super::${module}::${model.typeName}Optimistic>`;
    return `    fn ${name}(&self) -> ${returned} {\n        ${value}\n    }`;
  });
  const typeMods = typeStems.map((stem) => rustModLine(stem));
  const typeUses = typeStems.map((stem) => `pub use ${rustIdent(stem)}::*;`);
  const registry = `use datasources::{CRUDRepository, Datasource, ICRUDRepository, IReadonlyRepository, RepositoryFactory};\nuse super::super::{${models.map((model) => model.typeName).join(", ")}};\nuse std::sync::Arc;\n\npub fn register(factory: &RepositoryFactory) {\n${register.join("\n")}\n}\n\ndatasources::bind_repositories!(register);\n\npub trait Repositories {\n${methods.join("\n")}\n}\n\nimpl Repositories for dyn Datasource {\n${impls.join("\n")}\n}\n`;
  return [
    ...files,
    content(registryFile("rs", casing), registry),
    content("mod.rs", `${models.map((model) => rustModLine(fileStem(model.specName, casing))).join("\n")}\n${rustModLine(casing.convertFileName("repository_registry"))}\n`),
    content(
      "../mod.rs",
      `${typeMods.join("\n")}\n${typeUses.join("\n")}\nmod schema_objects;\npub mod repositories;\npub use repositories::${rustIdent(casing.convertFileName("repository_registry"))}::Repositories;\n`,
    ),
    content("../schema_objects.rs", rustSchemaObjects(models)),
  ];
};
