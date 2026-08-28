use crate::builders::{StatementBuilder, WhereBuilder};
use crate::command::Command;
use crate::connection::Connection;
use crate::converters::{mapping_with, Cells, FieldConverter, TypeMapping};
use crate::datasource_error::DatasourceError;
use crate::parameter::Parameter;
use crate::query::Query;
use crate::record::Record;
use crate::repository::{ReadonlyRepository, Repository};
use crate::response::Response;
use crate::result::Result;
use crate::statement::Statement;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct TableRepositoryOptions {
    pub datasource_name: String,
    pub connection: Arc<dyn Connection>,
    pub create_command: Arc<dyn Fn(Arc<dyn Connection>) -> Arc<dyn Command> + Send + Sync>,
    pub provider: Option<String>,
    pub id: Vec<String>,
    pub generated: bool,
    pub schema: HashMap<String, String>,
    pub converters: HashMap<String, FieldConverter>,
    pub names: HashMap<String, String>,
    pub mapping: Option<TypeMapping>,
}

impl TableRepositoryOptions {
    pub fn with_factory(
        datasource_name: impl Into<String>,
        connection: Arc<dyn Connection>,
        command: Arc<dyn Command>,
    ) -> Self {
        Self {
            datasource_name: datasource_name.into(),
            connection,
            create_command: Arc::new(move |_| command.clone()),
            provider: None,
            id: vec!["id".into()],
            generated: false,
            schema: HashMap::new(),
            converters: HashMap::new(),
            names: HashMap::new(),
            mapping: None,
        }
    }
}

pub struct TableRepository {
    datasource_name: String,
    connection: Arc<dyn Connection>,
    create_command: Arc<dyn Fn(Arc<dyn Connection>) -> Arc<dyn Command> + Send + Sync>,
    mapping: TypeMapping,
    id: Vec<String>,
    ids: HashSet<String>,
    generated: bool,
}

impl TableRepository {
    pub fn new(options: TableRepositoryOptions) -> Result<Self> {
        let mapping = match options.mapping {
            Some(mapping) => mapping,
            None => {
                let provider = options.provider.ok_or_else(|| {
                    DatasourceError::message("Repository requires Provider or Mapping")
                })?;
                mapping_with::<Cells>(
                    &provider,
                    "rust",
                    options.schema,
                    options.converters,
                    options.names,
                )?
            }
        };
        let ids = options.id.iter().cloned().collect();
        Ok(Self {
            datasource_name: options.datasource_name,
            connection: options.connection,
            create_command: options.create_command,
            mapping,
            id: options.id,
            ids,
            generated: options.generated,
        })
    }

    fn command(&self) -> Arc<dyn Command> {
        (self.create_command)(self.connection.clone())
    }

    fn fields(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.mapping.schema.keys().cloned().collect();
        keys.sort();
        keys
    }

    fn params(&self, record: &Record, include_id: bool) -> Vec<Parameter> {
        let mut pairs: Vec<_> = record
            .iter()
            .filter(|(key, _)| include_id || !self.ids.contains(*key))
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        pairs.sort_by(|left, right| left.0.cmp(&right.0));
        pairs
            .into_iter()
            .map(|(name, value)| Parameter::new(name, value))
            .collect()
    }

    fn id_where(&self, record: &Record) -> Result<WhereBuilder> {
        let children = self
            .id
            .iter()
            .map(|col| {
                let value = record.get(col).ok_or_else(|| {
                    DatasourceError::message(format!("Missing Id value \"{col}\""))
                })?;
                Ok(WhereBuilder::equals(col.clone(), value.clone()))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(WhereBuilder::and(children))
    }

    fn query_parts(&self, query: Option<&dyn Query>) -> (String, Vec<Parameter>) {
        if let Some(query) = query {
            let params = query
                .where_clause()
                .map(|predicate| predicate.parameters())
                .unwrap_or_else(|| query.parameters());
            (query.get_text(), params)
        } else {
            let keys = self.fields();
            let refs: Vec<&str> = keys.iter().map(String::as_str).collect();
            let built = StatementBuilder::create_query(&self.datasource_name, &refs, None);
            (built.get_text(), built.parameters())
        }
    }
}

#[async_trait]
impl ReadonlyRepository for TableRepository {
    async fn get_single(&self, query: Option<&dyn Query>) -> Result<Option<Record>> {
        let (text, params) = self.query_parts(query);
        let reader = self
            .command()
            .execute_reader(Some(&text), Some(&params))
            .await?;
        if !reader.next().await? {
            return Ok(None);
        }
        reader
            .current()
            .map(|row| self.mapping.to(&row))
            .transpose()
    }

    async fn get(&self, query: Option<&dyn Query>) -> Result<Vec<Record>> {
        let (text, params) = self.query_parts(query);
        let reader = self
            .command()
            .execute_reader(Some(&text), Some(&params))
            .await?;
        let mut items = Vec::new();
        while reader.next().await? {
            if let Some(row) = reader.current() {
                items.push(self.mapping.to(&row)?);
            }
        }
        Ok(items)
    }
}

#[async_trait]
impl Repository for TableRepository {
    async fn add(&self, obj: &Record) -> Result<Response> {
        let record = self.mapping.from(obj)?;
        let fields = self.params(&record, !self.generated);
        let statement = StatementBuilder::create_insert(&self.datasource_name, &fields);
        self.command()
            .execute(Some(&statement.get_text()), Some(&fields))
            .await
    }

    async fn update(&self, obj: &Record, by_where: Option<&dyn Where>) -> Result<Response> {
        let record = self.mapping.from(obj)?;
        let owned = if by_where.is_none() {
            Some(self.id_where(&record)?)
        } else {
            None
        };
        let predicate: &dyn Where = match (by_where, owned.as_ref()) {
            (Some(where_clause), _) => where_clause,
            (_, Some(where_clause)) => where_clause,
            _ => unreachable!(),
        };
        let fields = self.params(&record, false);
        if fields.is_empty() {
            return Err(DatasourceError::message(
                "Update requires at least one non-Id field",
            ));
        }
        let statement = StatementBuilder::create_update(&self.datasource_name, &fields, Some(predicate));
        let mut parameters = fields;
        parameters.extend(predicate.parameters());
        self.command()
            .execute(Some(&statement.get_text()), Some(&parameters))
            .await
    }

    async fn delete(&self, obj: &Record, by_where: Option<&dyn Where>) -> Result<Response> {
        let record = self.mapping.from(obj)?;
        let owned = if by_where.is_none() {
            Some(self.id_where(&record)?)
        } else {
            None
        };
        let predicate: &dyn Where = match (by_where, owned.as_ref()) {
            (Some(where_clause), _) => where_clause,
            (_, Some(where_clause)) => where_clause,
            _ => unreachable!(),
        };
        let statement = StatementBuilder::create_delete(&self.datasource_name, Some(predicate));
        let parameters = predicate.parameters();
        self.command()
            .execute(Some(&statement.get_text()), Some(&parameters))
            .await
    }
}
