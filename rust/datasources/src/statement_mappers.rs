use crate::builders::Join;
use crate::statements::select_field_aliases::{
    apply_select_field_alias, as_index, function_of, select_field_aliases, segments,
};
use crate::IWhere;
use crate::mappers::Mapper;
use crate::parameter::Parameter;
use crate::result::Result;
use crate::statements::statement::Statement;
use crate::statements::{OrderBy, SelectStatementOptions, Where};
use crate::value::Value;
use crate::{DeleteStatement, InsertStatement, SelectInsertedIdStatement, SelectStatement, UpdateStatement};
use std::sync::Arc;

#[derive(Clone)]
pub struct StatementMappers {
    entity_mappers: Arc<dyn Mapper<String, String> + Send + Sync>,
    property_mappers: Arc<dyn Mapper<String, String> + Send + Sync>,
}

impl StatementMappers {
    pub fn new(
        entity_mappers: Arc<dyn Mapper<String, String> + Send + Sync>,
        property_mappers: Arc<dyn Mapper<String, String> + Send + Sync>,
    ) -> Self {
        Self {
            entity_mappers,
            property_mappers,
        }
    }

    pub(crate) fn column(&self, property: &str) -> String {
        Mapper::map(self, property.to_string())
    }

    pub(crate) fn property(&self, key: &str) -> String {
        Mapper::reverse_map(self, key.to_string())
    }

    fn map_select_fields(
        &self,
        table: &str,
        fields: &[String],
        aliases: &[Option<String>],
    ) -> Result<Vec<String>> {
        let lookup = select_field_aliases(table, fields, aliases);
        fields
            .iter()
            .enumerate()
            .map(|(index, field)| self.map_select_field(field, lookup[index].as_deref()))
            .collect()
    }

    fn map_select_field(&self, field: &str, alias: Option<&str>) -> Result<String> {
        if as_index(field).is_some() {
            return Err(crate::DatasourceError::message(format!(
                "Select field \"{field}\" must be a path"
            )));
        }
        let mapped = match function_of(field) {
            Some((name, inner)) => format!("{name}({})", self.column(inner)),
            None => self.column(field),
        };
        Ok(apply_select_field_alias(&mapped, alias))
    }

    pub fn apply_select(
        &self,
        select: &SelectStatement,
        cell: &dyn Fn(&str, &Value) -> Result<Value>,
    ) -> Result<SelectStatement> {
        let table = self.entity(&select.table);
        let fields = self.map_select_fields(&select.table, &select.fields, &select.aliases)?;
        let field_refs: Vec<&str> = fields.iter().map(String::as_str).collect();
        let group: Vec<String> = select.group_by.iter().map(|column| self.column(column)).collect();
        let group_refs: Vec<&str> = group.iter().map(String::as_str).collect();
        let order: Vec<OrderBy> = select
            .order_by
            .iter()
            .map(|item| OrderBy {
                column: self.column(&item.column),
                direction: item.direction,
            })
            .collect();
        let joins: Vec<Join> = select
            .joins
            .iter()
            .map(|join| Join {
                kind: join.kind,
                table: self.entity(&join.table),
                on: join.on.as_ref().map(|on| map_on(on, |name| self.column(name))),
            })
            .collect();
        let mapped_where = self.apply_predicate(select.predicate(), cell)?;
        let mapped_having = self.apply_predicate(select.having.as_deref(), cell)?;
        let where_clause = mapped_where
            .as_ref()
            .map(|clause| clause as &dyn IWhere)
            .or(select.predicate());
        let having = mapped_having
            .as_ref()
            .map(|clause| clause as &dyn IWhere)
            .or(select.having.as_deref());
        Ok(SelectStatement::new(SelectStatementOptions {
            table: &table,
            fields: &field_refs,
            aliases: &[],
            where_clause,
            joins: &joins,
            group_by: &group_refs,
            order_by: &order,
            limit: select.limit,
            distinct: select.distinct,
            having,
            offset: select.offset,
        }))
    }

    pub fn apply_where(
        &self,
        clause: &Where,
        cell: &dyn Fn(&str, &Value) -> Result<Value>,
    ) -> Result<Where> {
        clause.map(|name| self.column(name), cell)
    }

    pub fn apply_insert(&self, insert: &InsertStatement) -> InsertStatement {
        let fields: Vec<Parameter> = insert
            .fields()
            .iter()
            .map(|field| Parameter {
                name: self.column(&field.name),
                value: field.value.clone(),
                direction: field.direction,
            })
            .collect();
        InsertStatement::new(self.entity(insert.table()), &fields)
    }

    pub fn apply_update(
        &self,
        update: &UpdateStatement,
        cell: &dyn Fn(&str, &Value) -> Result<Value>,
    ) -> Result<UpdateStatement> {
        let fields: Vec<Parameter> = update
            .fields()
            .iter()
            .map(|field| Parameter {
                name: self.column(&field.name),
                value: field.value.clone(),
                direction: field.direction,
            })
            .collect();
        let mapped = self.apply_predicate(update.where_clause(), cell)?;
        let clause: Option<&dyn IWhere> = match mapped.as_ref() {
            Some(where_clause) => Some(where_clause),
            None => update.where_clause(),
        };
        Ok(UpdateStatement::new(self.entity(update.table()), &fields, clause))
    }

    pub fn apply_delete(
        &self,
        delete: &DeleteStatement,
        cell: &dyn Fn(&str, &Value) -> Result<Value>,
    ) -> Result<DeleteStatement> {
        let mapped = self.apply_predicate(delete.where_clause(), cell)?;
        let clause: Option<&dyn IWhere> = match mapped.as_ref() {
            Some(where_clause) => Some(where_clause),
            None => delete.where_clause(),
        };
        Ok(DeleteStatement::new(self.entity(delete.table()), clause))
    }

    pub fn apply_inserted(&self, inserted: &SelectInsertedIdStatement) -> SelectInsertedIdStatement {
        SelectInsertedIdStatement::new(self.entity(&inserted.table), self.column(&inserted.column))
    }

    fn apply_predicate(
        &self,
        predicate: Option<&dyn IWhere>,
        cell: &dyn Fn(&str, &Value) -> Result<Value>,
    ) -> Result<Option<Where>> {
        match predicate.and_then(|item| item.as_any().downcast_ref::<Where>()) {
            Some(clause) => Ok(Some(self.apply_where(clause, cell)?)),
            None => Ok(None),
        }
    }

    fn entity(&self, name: &str) -> String {
        Mapper::map(&self.entity_mappers, name.to_string())
    }
}

impl Mapper<String, String> for StatementMappers {
    fn map(&self, property: String) -> String {
        let parts = segments(&property);
        if parts.len() < 2 {
            return Mapper::map(&self.property_mappers, property);
        }
        let qualifier = parts[parts.len() - 2];
        let name = parts[parts.len() - 1];
        let mapped = Mapper::map(&self.property_mappers, name.to_string());
        if mapped != name && mapped.contains('.') {
            return mapped;
        }
        let table = self.entity(qualifier);
        if table == qualifier && mapped == name {
            format!("{qualifier}.{name}")
        } else {
            format!("{table}.{mapped}")
        }
    }

    fn reverse_map(&self, key: String) -> String {
        let reversed = Mapper::reverse_map(&self.property_mappers, key.clone());
        if reversed != key && Mapper::map(&self.property_mappers, reversed.clone()) == key {
            return reversed;
        }
        let parts = segments(&key);
        if parts.len() < 2 {
            return reversed;
        }
        let column = parts[parts.len() - 1];
        let name = Mapper::reverse_map(&self.property_mappers, column.to_string());
        if name != column && Mapper::map(&self.property_mappers, name.clone()) == column {
            name
        } else {
            reversed
        }
    }
}

fn map_on(on: &str, column: impl Fn(&str) -> String) -> String {
    match on.split_once(" = ") {
        Some((left, right)) if !left.contains(' ') && !right.contains(' ') => {
            format!("{} = {}", column(left), column(right))
        }
        _ => on.to_string(),
    }
}

