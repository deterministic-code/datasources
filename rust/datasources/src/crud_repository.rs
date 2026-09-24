use crate::built_repository::BuiltRepository;
use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::converters::{ConvertSchema, RowTypeConverter};
use crate::create_repository_options::create_repository_options;
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::datasource_error::DatasourceError;
use crate::i_repository::IRepository;
use crate::id_key::IdKey;
use crate::mappers::{Mapper, TransformMapper};
use crate::parameter::Parameter;
use crate::read_input::ReadInput;
use crate::readonly_repository::ReadonlyRepository;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::table_repository_builder::RepositoryBuilder;
use crate::value::Value;
use crate::where_trait::Where;
use crate::{Expression, Literal, Operator, WhereClause};
use async_trait::async_trait;
use std::any::TypeId;
use std::collections::HashSet;
use std::sync::Arc;

pub struct CRUDRepository {
    inner: ReadonlyRepository,
    create: TransformMapper<Value, Row>,
    id: Vec<String>,
    ids: HashSet<String>,
}

impl RepositoryBuilder {
    pub fn build(
        connection: Arc<dyn Connection>,
        options: CRUDRepositoryOptions,
    ) -> Result<CRUDRepository> {
        CRUDRepository::new(connection, options)
    }
}

impl CRUDRepository {
    pub fn from_type<T: ConvertSchema + 'static>(connection: Arc<dyn Connection>) -> Result<Self> {
        let mut options = create_repository_options::<T>();
        options.row_converter = Some(RowTypeConverter::of::<T>());
        let mut repo = Self::new(connection, options)?;
        repo.inner.entity_type = TypeId::of::<T>();
        Ok(repo)
    }

    pub fn new(connection: Arc<dyn Connection>, options: CRUDRepositoryOptions) -> Result<Self> {
        Ok(Self::from_built(RepositoryBuilder::parts_crud(
            connection, options,
        )?))
    }

    pub fn from_built(mut built: BuiltRepository) -> Self {
        let id = built.ids.clone();
        let create = built
            .create
            .take()
            .unwrap_or_else(|| CRUDRepositoryOptions::default().create);
        Self {
            inner: ReadonlyRepository::from_built(built),
            create,
            ids: id.iter().cloned().collect(),
            id,
        }
    }

    pub fn add_middleware(&mut self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware);
    }

    fn params(&self, record: &Row, include_ids: bool) -> Result<Vec<Parameter>> {
        let mut pairs: Vec<_> = record
            .iter()
            .filter(|(key, _)| include_ids || !self.ids.contains(*key))
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        pairs.sort_by(|left, right| left.0.cmp(&right.0));
        pairs
            .into_iter()
            .map(|(name, value)| {
                Ok(Parameter::new(
                    self.inner.column(&name),
                    self.inner.cell(&name, &value, true)?,
                ))
            })
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
                Ok(Expression::from(Operator::equals(
                    col.clone(),
                    value.clone(),
                )))
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
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository>> {
        Some(self)
    }

    async fn read_single(&self, query: Option<&dyn ReadInput>) -> Result<Option<Row>> {
        self.inner.read_single(query).await
    }

    async fn read(&self, query: Option<&dyn ReadInput>) -> Result<Vec<Row>> {
        self.inner.read(query).await
    }
}

#[async_trait]
impl ICRUDRepository for CRUDRepository {
    async fn create(&self, item: &mut Row) -> Result<Response> {
        let record = self.inner.reverse_convert(item)?;
        let fields = self.params(&record, false)?;
        let statement = self.inner.statements.insert(&fields);
        let response = self
            .inner
            .command()
            .execute_statement(&statement, Some(&fields))
            .await?;
        if response.result != Value::Null {
            apply(item, Mapper::map(&self.create, response.result.clone()));
        }
        Ok(response)
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        let record = self.inner.reverse_convert(item)?;
        let fields = self.params(&record, false)?;
        if fields.is_empty() {
            return Err(DatasourceError::message(
                "Update requires at least one non-Id field",
            ));
        }
        let (owned, raw) = self.stored_predicate(&record, by_where)?;
        let predicate: &dyn Where = match (owned.as_ref(), raw) {
            (Some(clause), _) => clause,
            (None, Some(clause)) => clause,
            _ => unreachable!(),
        };
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
        let (owned, raw) = self.stored_predicate(&record, by_where)?;
        let predicate: &dyn Where = match (owned.as_ref(), raw) {
            (Some(clause), _) => clause,
            (None, Some(clause)) => clause,
            _ => unreachable!(),
        };
        let statement = self.inner.statements.delete(predicate);
        let parameters = predicate.parameters();
        let parameters = if parameters.is_empty() {
            self.params(&record, true)?
        } else {
            parameters
        };
        self.inner
            .command()
            .execute_statement(&statement, Some(&parameters))
            .await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Row>> {
        let record = self.id_record(id)?;
        let clause = self.id_where(&record)?;
        self.read_single(Some(&clause)).await
    }
}

impl CRUDRepository {
    fn stored_predicate<'a>(
        &'a self,
        record: &Row,
        by_where: Option<&'a dyn Where>,
    ) -> Result<(Option<WhereClause>, Option<&'a dyn Where>)> {
        match by_where {
            Some(clause) => match self.inner.store_where(clause, false)? {
                Some(stored) => Ok((Some(stored), None)),
                None => Ok((None, Some(clause))),
            },
            None => Ok((
                Some(self.inner.store_clause(&self.id_where(record)?, true)?),
                None,
            )),
        }
    }

    fn id_record(&self, id: &dyn IdKey) -> Result<Row> {
        if self.id.is_empty() {
            return Err(DatasourceError::message(
                "ReadSingleById requires an Id column",
            ));
        }
        match (id.as_value(), id.as_row()) {
            (Some(value), _) if self.id.len() == 1 => {
                let mut row = Row::new();
                row.insert(self.id[0].clone(), value.clone());
                Ok(row)
            }
            (Some(_), _) => Err(DatasourceError::message(
                "Composite id requires a row keyed by each Id column",
            )),
            (_, Some(row)) => Ok(row.clone()),
            _ => Err(DatasourceError::message(
                "ReadSingleById requires an Id column",
            )),
        }
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
