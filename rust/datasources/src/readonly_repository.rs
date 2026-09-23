use crate::built_repository::BuiltRepository;
use crate::command::Command;
use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::converters::{ConvertSchema, RowTypeConverter};
use crate::create_repository_options::create_repository_options;
use crate::i_repository::IRepository;
use crate::mappers::Mapper;
use crate::parameter::Parameter;
use crate::query::Query;
use crate::readonly_repository_options::ReadonlyRepositoryOptions;
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use crate::statement::Statement;
use crate::statements::{ArrayExpression, Expression, Identifier, Literal, Operator, WhereClause};
use crate::table_repository_builder::RepositoryBuilder;
use crate::table_repository_statements::RepositoryStatements;
use crate::text_query::QueryStatement;
use crate::value::Value;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::any::TypeId;
use std::sync::Arc;

pub struct ReadonlyRepository {
    pub(crate) connection: Arc<dyn Connection>,
    pub(crate) middlewares: Vec<Arc<dyn CommandMiddleware>>,
    pub(crate) row_converter: RowTypeConverter,
    pub(crate) rows_mapper: Box<dyn Mapper<Row, Row> + Send + Sync>,
    pub(crate) column_mappings: Box<dyn Mapper<String, String> + Send + Sync>,
    pub(crate) statements: RepositoryStatements,
    pub(crate) entity_type: TypeId,
}

impl ReadonlyRepository {
    pub fn from_type<T: ConvertSchema + 'static>(connection: Arc<dyn Connection>) -> Result<Self> {
        let mut options = create_repository_options::<T>();
        options.row_converter = Some(RowTypeConverter::of::<T>());
        let mut repo = Self::new(connection, options.read)?;
        repo.entity_type = TypeId::of::<T>();
        Ok(repo)
    }

    pub fn new(
        connection: Arc<dyn Connection>,
        options: ReadonlyRepositoryOptions,
    ) -> Result<Self> {
        Ok(Self::from_built(RepositoryBuilder::parts(
            connection, options,
        )?))
    }

    pub fn from_built(built: BuiltRepository) -> Self {
        Self {
            connection: built.connection,
            middlewares: Vec::new(),
            row_converter: built.row_converter,
            rows_mapper: built.rows_mapper,
            column_mappings: built.column_mappings,
            statements: built.statements,
            entity_type: TypeId::of::<Row>(),
        }
    }

    pub fn add_middleware(&mut self, middleware: Arc<dyn CommandMiddleware>) {
        self.middlewares.push(middleware);
    }

    pub(crate) fn command(&self) -> Arc<dyn Command> {
        let command = self.connection.create_command();
        for middleware in &self.middlewares {
            command.add_middleware(middleware.clone());
        }
        command
    }

    pub(crate) fn query_of(&self, query: Option<&dyn Query>) -> (QueryStatement, Vec<Parameter>) {
        if let Some(query) = query {
            let params = query
                .where_clause()
                .map(|predicate| predicate.parameters())
                .unwrap_or_else(|| query.parameters());
            let statement = match query.where_clause() {
                Some(predicate) => QueryStatement::with_where(
                    query.get_text(),
                    crate::text_where::TextWhere::new(predicate.get_text()),
                ),
                None => QueryStatement::text(query.get_text()),
            };
            (statement, params)
        } else {
            let statement = self.statements.query(None);
            let params = statement.parameters();
            (statement, params)
        }
    }

    pub(crate) async fn fetch_one(
        &self,
        statement: &dyn crate::statement::Statement,
        params: Vec<Parameter>,
    ) -> Result<Option<Row>> {
        let reader = self
            .command()
            .execute_reader_statement(statement, Some(&params))
            .await?;
        if !reader.next().await? {
            return Ok(None);
        }
        reader
            .current()
            .map(|row| {
                self.row_converter
                    .convert(&self.renamed(row))
                    .map(|converted| Mapper::map(&self.rows_mapper, converted))
            })
            .transpose()
    }

    pub(crate) async fn fetch_all(
        &self,
        statement: &dyn crate::statement::Statement,
        params: Vec<Parameter>,
    ) -> Result<Vec<Row>> {
        let reader = self
            .command()
            .execute_reader_statement(statement, Some(&params))
            .await?;
        let mut items = Vec::new();
        while reader.next().await? {
            if let Some(row) = reader.current() {
                items.push(Mapper::map(
                    &self.rows_mapper,
                    self.row_converter.convert(&self.renamed(row))?,
                ));
            }
        }
        Ok(items)
    }

    fn renamed(&self, row: Row) -> Row {
        row.into_iter()
            .map(|(key, value)| (Mapper::reverse_map(&self.column_mappings, key), value))
            .collect()
    }

    pub(crate) fn reverse_convert(&self, item: &Row) -> Result<Row> {
        self.row_converter
            .reverse_convert(&Mapper::reverse_map(&self.rows_mapper, item.clone()))
    }

    fn stored_query(&self, where_clause: &dyn Where) -> Result<(QueryStatement, Vec<Parameter>)> {
        match self.store_where(where_clause, false)? {
            Some(stored) => Ok((self.statements.query(Some(&stored)), stored.parameters())),
            None => Ok((
                self.statements.query(Some(where_clause)),
                where_clause.parameters(),
            )),
        }
    }

    pub(crate) fn store_where(
        &self,
        where_clause: &dyn Where,
        fields_applied: bool,
    ) -> Result<Option<WhereClause>> {
        match where_clause.as_any().downcast_ref::<WhereClause>() {
            Some(clause) => Ok(Some(self.store_clause(clause, fields_applied)?)),
            None => Ok(None),
        }
    }

    pub(crate) fn store_clause(
        &self,
        clause: &WhereClause,
        fields_applied: bool,
    ) -> Result<WhereClause> {
        Ok(WhereClause::new(
            self.rewrite(&clause.condition, fields_applied)?,
        ))
    }

    pub(crate) fn column(&self, property: &str) -> String {
        Mapper::map(&self.column_mappings, property.to_string())
    }

    pub(crate) fn cell(
        &self,
        property: &str,
        value: &Value,
        fields_applied: bool,
    ) -> Result<Value> {
        if let Some(field) = self.row_converter.fields.get(property) {
            return if fields_applied {
                Ok(value.clone())
            } else {
                field.reverse_convert(value)
            };
        }
        let kind = match value {
            Value::Bool(_) => Some("bool"),
            Value::DateTime(_) => Some("datetime"),
            Value::Uuid(_) => Some("uuid"),
            Value::Decimal(_) => Some("decimal"),
            Value::Binary(_) => Some("binary"),
            _ => None,
        };
        match kind.and_then(|key| self.row_converter.types.get(key)) {
            Some(converter) => converter.reverse_convert(value),
            None => Ok(value.clone()),
        }
    }

    fn rewrite(&self, expression: &Expression, fields_applied: bool) -> Result<Expression> {
        Ok(match expression {
            Expression::Identifier(id) => {
                Expression::Identifier(Identifier::new(self.column(&id.name)))
            }
            Expression::Literal(lit) => Expression::Literal(lit.clone()),
            Expression::Array(arr) => Expression::Array(ArrayExpression::new(
                arr.elements
                    .iter()
                    .map(|element| self.rewrite(element, fields_applied))
                    .collect::<Result<_>>()?,
            )),
            Expression::Operator(op) => Expression::Operator(self.rewrite_op(op, fields_applied)?),
        })
    }

    fn rewrite_op(&self, op: &Operator, fields_applied: bool) -> Result<Operator> {
        Ok(match op {
            Operator::And { left, right } => Operator::and(
                self.rewrite(left, fields_applied)?,
                self.rewrite(right, fields_applied)?,
            ),
            Operator::Or { left, right } => Operator::or(
                self.rewrite(left, fields_applied)?,
                self.rewrite(right, fields_applied)?,
            ),
            Operator::Not { argument } => Operator::not(self.rewrite(argument, fields_applied)?),
            Operator::In { left, right } => Operator::In {
                left: Box::new(self.rewrite(left, fields_applied)?),
                right: Box::new(self.in_right(left, right, fields_applied)?),
            },
            Operator::Equals { left, right } => {
                self.binary(left, right, fields_applied, |left, right| {
                    Operator::Equals { left, right }
                })?
            }
            Operator::NotEquals { left, right } => {
                self.binary(left, right, fields_applied, |left, right| {
                    Operator::NotEquals { left, right }
                })?
            }
            Operator::GreaterThan { left, right } => {
                self.binary(left, right, fields_applied, |left, right| {
                    Operator::GreaterThan { left, right }
                })?
            }
            Operator::LessThan { left, right } => {
                self.binary(left, right, fields_applied, |left, right| {
                    Operator::LessThan { left, right }
                })?
            }
            Operator::GreaterThanOrEqual { left, right } => {
                self.binary(left, right, fields_applied, |left, right| {
                    Operator::GreaterThanOrEqual { left, right }
                })?
            }
            Operator::LessThanOrEqual { left, right } => {
                self.binary(left, right, fields_applied, |left, right| {
                    Operator::LessThanOrEqual { left, right }
                })?
            }
        })
    }

    fn binary(
        &self,
        left: &Expression,
        right: &Expression,
        fields_applied: bool,
        build: impl FnOnce(Box<Expression>, Box<Expression>) -> Operator,
    ) -> Result<Operator> {
        let (left, right) = self.pair(left, right, fields_applied)?;
        Ok(build(Box::new(left), Box::new(right)))
    }

    fn pair(
        &self,
        left: &Expression,
        right: &Expression,
        fields_applied: bool,
    ) -> Result<(Expression, Expression)> {
        match (left, right) {
            (Expression::Identifier(id), Expression::Literal(lit)) => Ok((
                Expression::Identifier(Identifier::new(self.column(&id.name))),
                Expression::Literal(Literal::new(self.cell(
                    &id.name,
                    &lit.value,
                    fields_applied,
                )?)),
            )),
            (Expression::Literal(lit), Expression::Identifier(id)) => Ok((
                Expression::Literal(Literal::new(self.cell(
                    &id.name,
                    &lit.value,
                    fields_applied,
                )?)),
                Expression::Identifier(Identifier::new(self.column(&id.name))),
            )),
            _ => Ok((
                self.rewrite(left, fields_applied)?,
                self.rewrite(right, fields_applied)?,
            )),
        }
    }

    fn in_right(
        &self,
        left: &Expression,
        right: &Expression,
        fields_applied: bool,
    ) -> Result<Expression> {
        match (left, right) {
            (Expression::Identifier(id), Expression::Array(arr)) => {
                let elements = arr
                    .elements
                    .iter()
                    .map(|element| match element {
                        Expression::Literal(lit) => Ok(Expression::Literal(Literal::new(
                            self.cell(&id.name, &lit.value, fields_applied)?,
                        ))),
                        other => self.rewrite(other, fields_applied),
                    })
                    .collect::<Result<Vec<_>>>()?;
                Ok(Expression::Array(ArrayExpression::new(elements)))
            }
            _ => self.rewrite(right, fields_applied),
        }
    }
}

impl IRepository for ReadonlyRepository {
    fn entity_type(&self) -> TypeId {
        self.entity_type
    }
}

#[async_trait]
impl IReadonlyRepository for ReadonlyRepository {
    async fn read_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>> {
        let (statement, params) = self.query_of(query);
        self.fetch_one(&statement, params).await
    }

    async fn read(&self, query: Option<&dyn Query>) -> Result<Vec<Row>> {
        let (statement, params) = self.query_of(query);
        self.fetch_all(&statement, params).await
    }

    async fn read_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        let (statement, params) = self.stored_query(where_clause)?;
        self.fetch_one(&statement, params).await
    }

    async fn read_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        let (statement, params) = self.stored_query(where_clause)?;
        self.fetch_all(&statement, params).await
    }
}
