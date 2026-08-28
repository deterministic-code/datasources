use crate::builders::{StatementBuilder, WhereBuilder};
use crate::command::Command;
use crate::connection::Connection;
use crate::converters::{mapping_with, Cells, ConvertSchema, FieldConverter, TypeMapping};
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
    pub ids: Vec<String>,
    pub assign_identity_on_add: bool,
    pub schema: HashMap<String, String>,
    pub converters: HashMap<String, FieldConverter>,
    pub names: HashMap<String, String>,
    pub mapping: Option<TypeMapping>,
}

impl TableRepositoryOptions {
    pub fn with_factory(
        datasource_name: impl Into<String>,
        connection: Arc<dyn Connection>,
    ) -> Self {
        Self {
            datasource_name: datasource_name.into(),
            connection,
            ids: vec!["id".into()],
            assign_identity_on_add: true,
            schema: HashMap::new(),
            converters: HashMap::new(),
            names: HashMap::new(),
            mapping: None,
        }
    }

    pub fn from_type<T: ConvertSchema>(connection: Arc<dyn Connection>) -> Self {
        Self {
            datasource_name: type_name::<T>(),
            ids: default_id::<T>(),
            assign_identity_on_add: true,
            schema: HashMap::new(),
            converters: HashMap::new(),
            names: HashMap::new(),
            mapping: None,
            connection,
        }
    }
}

fn type_name<T>() -> String {
    std::any::type_name::<T>()
        .rsplit("::")
        .next()
        .unwrap_or_default()
        .into()
}

fn default_id<T: ConvertSchema>() -> Vec<String> {
    let names: Vec<_> = T::schema().into_iter().map(|(name, _)| name).collect();
    if names.contains(&"id") {
        vec!["id".into()]
    } else if names.contains(&"Id") {
        vec!["Id".into()]
    } else {
        vec![]
    }
}

pub struct TableRepository {
    datasource_name: String,
    connection: Arc<dyn Connection>,
    mapping: TypeMapping,
    id: Vec<String>,
    ids: HashSet<String>,
    assign_identity_on_add: bool,
}

impl TableRepository {
    pub fn from_type<T: ConvertSchema>(connection: Arc<dyn Connection>) -> Result<Self> {
        Self::new(TableRepositoryOptions::from_type::<T>(connection))
    }

    pub fn new(options: TableRepositoryOptions) -> Result<Self> {
        let mapping = match options.mapping {
            Some(mapping) => mapping,
            None => {
                mapping_with::<Cells>(
                    options.connection.provider(),
                    "rust",
                    options.schema,
                    options.converters,
                    options.names,
                )?
            }
        };
        let ids = options.ids.iter().cloned().collect();
        Ok(Self {
            datasource_name: options.datasource_name,
            connection: options.connection,
            mapping,
            id: options.ids,
            ids,
            assign_identity_on_add: options.assign_identity_on_add,
        })
    }

    fn command(&self) -> Arc<dyn Command> {
        self.connection.create_command()
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
        let fields = self.params(&record, !self.assign_identity_on_add);
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
