use crate::{Expression, Literal, Operator, WhereClause};
use crate::built_repository::BuiltRepository;
use crate::crud_mapper::CRUDMapper;
use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::converters::{ConvertSchema, RowTypeConverter};
use crate::create_repository_builder_options::create_repository_builder_options;
use crate::datasource_error::DatasourceError;
use crate::i_repository::IRepository;
use crate::parameter::Parameter;
use crate::query::Query;
use crate::readonly_repository::ReadonlyRepository;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::mappers::Mapper;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use crate::table_repository_builder::RepositoryBuilder;
use crate::table_repository_builder_options::RepositoryBuilderOptions;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::any::TypeId;
use std::collections::HashSet;
use std::sync::Arc;

pub struct CRUDRepository {
    inner: ReadonlyRepository,
    pub crud_mapper: Arc<CRUDMapper>,
    id: Vec<String>,
    ids: HashSet<String>,
}

impl RepositoryBuilder {
    pub fn build(options: RepositoryBuilderOptions) -> Result<CRUDRepository> {
        CRUDRepository::new(options)
    }
}

impl CRUDRepository {
    pub fn from_type<T: ConvertSchema + 'static>(connection: Arc<dyn Connection>) -> Result<Self> {
        let mut options = create_repository_builder_options::<T>(connection.clone());
        options.mapping = Some(RowTypeConverter::of::<T>());
        let mut repo = Self::new(options)?;
        repo.inner.entity_type = TypeId::of::<T>();
        Ok(repo)
    }

    pub fn new(options: RepositoryBuilderOptions) -> Result<Self> {
        Ok(Self::from_built(RepositoryBuilder::parts(options)?))
    }

    pub fn from_built(built: BuiltRepository) -> Self {
        let id = built.ids.clone();
        let crud_mapper = built.crud_mapper.clone();
        Self {
            inner: ReadonlyRepository::from_built(built),
            crud_mapper,
            ids: id.iter().cloned().collect(),
            id,
        }
    }

    pub fn add_middleware(&mut self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware);
    }

    fn params(&self, record: &Row) -> Vec<Parameter> {
        let mut pairs: Vec<_> = record
            .iter()
            .filter(|(key, _)| !self.ids.contains(*key))
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        pairs.sort_by(|left, right| left.0.cmp(&right.0));
        pairs
            .into_iter()
            .map(|(name, value)| Parameter::new(name, value))
            .collect()
    }

    fn id_where(&self, record: &Row) -> Result<WhereClause> {
        let children = self
            .id
            .iter()
            .map(|col| {
                let value = record.get(col).ok_or_else(|| {
                    DatasourceError::message(format!("Missing Id value \"{col}\""))
                })?;
                Ok(Expression::from(Operator::equals(col.clone(), value.clone())))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(WhereClause::new(and_all(children)))
    }
}

impl IRepository for CRUDRepository {
    fn entity_type(&self) -> TypeId {
        self.inner.entity_type()
    }
}

#[async_trait]
impl IReadonlyRepository for CRUDRepository {
    async fn read_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>> {
        self.inner.read_single(query).await
    }

    async fn read(&self, query: Option<&dyn Query>) -> Result<Vec<Row>> {
        self.inner.read(query).await
    }

    async fn read_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        self.inner.read_single_where(where_clause).await
    }

    async fn read_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        self.inner.read_where(where_clause).await
    }
}

#[async_trait]
impl ICRUDRepository for CRUDRepository {
    async fn create(&self, item: &mut Row) -> Result<Response> {
        let record = self.inner.reverse_convert(item)?;
        let fields = self.params(&record);
        let statement = self.inner.statements.insert(&fields);
        let response = self
            .inner
            .command()
            .execute_statement(&statement, Some(&fields))
            .await?;
        if response.result != Value::Null {
            apply(item, Mapper::map(&self.crud_mapper.create, response.result.clone()));
        }
        Ok(response)
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        let record = self.inner.reverse_convert(item)?;
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
        let fields = self.params(&record);
        if fields.is_empty() {
            return Err(DatasourceError::message(
                "Update requires at least one non-Id field",
            ));
        }
        let statement = self.inner.statements.update(&fields, predicate);
        let mut parameters = fields;
        parameters.extend(predicate.parameters());
        self.inner
            .command()
            .execute_statement(&statement, Some(&parameters))
            .await
    }

    async fn delete(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        let record = self.inner.reverse_convert(item)?;
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
        let statement = self.inner.statements.delete(predicate);
        let parameters = predicate.parameters();
        let parameters = if parameters.is_empty() {
            record
                .iter()
                .map(|(name, value)| Parameter::new(name.clone(), value.clone()))
                .collect()
        } else {
            parameters
        };
        self.inner
            .command()
            .execute_statement(&statement, Some(&parameters))
            .await
    }
}

fn apply(item: &mut Row, identity: Row) {
    for (key, value) in identity {
        if value != Value::Null {
            item.insert(key, value);
        }
    }
}

fn and_all(parts: Vec<Expression>) -> Expression {
    parts
        .into_iter()
        .reduce(|left, right| Operator::and(left, right).into())
        .unwrap_or_else(|| {
            Operator::Equals {
                left: Box::new(Expression::Literal(Literal::new(1_i64))),
                right: Box::new(Expression::Literal(Literal::new(1_i64))),
            }
            .into()
        })
}
