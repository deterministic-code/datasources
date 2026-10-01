use crate::builders::Join;
use crate::i_where::IWhere;
use crate::mappers::Mapper;
use crate::parameter::Parameter;
use crate::result::Result;
use crate::statements::statement::Statement;
use crate::statements::text::Text;
use crate::statements::{OrderBy, SelectStatementOptions, Where};
use crate::value::Value;
use crate::{DeleteStatement, InsertStatement, SelectInsertedIdStatement, SelectStatement, UpdateStatement};
use std::sync::Arc;

#[derive(Clone)]
pub struct StatementMappings {
    entity_mappers: Arc<dyn Mapper<String, String> + Send + Sync>,
    property_mappers: Arc<dyn Mapper<String, String> + Send + Sync>,
}

impl StatementMappings {
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
        let Some(dot) = last_dot(property) else {
            return Mapper::map(&self.property_mappers, property.to_string());
        };
        let qualifier = &property[..dot];
        let name = &property[dot + 1..];
        let mapped = Mapper::map(&self.property_mappers, name.to_string());
        if mapped != name && mapped.contains('.') {
            return mapped;
        }
        let table = self.entity(qualifier);
        if table == qualifier && mapped == name {
            property.to_string()
        } else {
            format!("{table}.{mapped}")
        }
    }

    pub(crate) fn property(&self, key: &str) -> String {
        let reversed = Mapper::reverse_map(&self.property_mappers, key.to_string());
        if reversed != key && Mapper::map(&self.property_mappers, reversed.clone()) == key {
            return reversed;
        }
        let Some(dot) = last_dot(key) else {
            return reversed;
        };
        let column = &key[dot + 1..];
        let name = Mapper::reverse_map(&self.property_mappers, column.to_string());
        if name != column && Mapper::map(&self.property_mappers, name.clone()) == column {
            name
        } else {
            reversed
        }
    }

    pub fn apply_select(
        &self,
        select: &SelectStatement,
        cell: &dyn Fn(&str, &Value) -> Result<Value>,
    ) -> Result<SelectStatement> {
        let table = self.entity(&select.table);
        let fields: Vec<String> = select.fields.iter().map(|field| self.column(field)).collect();
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
        let Some(table) = insert.table() else {
            return InsertStatement::text(insert.get_text());
        };
        let fields: Vec<Parameter> = insert
            .fields()
            .iter()
            .map(|field| Parameter {
                name: self.column(&field.name),
                value: field.value.clone(),
                direction: field.direction,
            })
            .collect();
        InsertStatement::new(self.entity(&table), &fields)
    }

    pub fn apply_update(
        &self,
        update: &UpdateStatement,
        cell: &dyn Fn(&str, &Value) -> Result<Value>,
    ) -> Result<UpdateStatement> {
        let Some(table) = update.table() else {
            return Ok(UpdateStatement::text(
                update.get_text(),
                crate::where_text::boxed(update.where_clause()),
            ));
        };
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
        Ok(UpdateStatement::new(self.entity(&table), &fields, clause))
    }

    pub fn apply_delete(
        &self,
        delete: &DeleteStatement,
        cell: &dyn Fn(&str, &Value) -> Result<Value>,
    ) -> Result<DeleteStatement> {
        let Some(table) = delete.table() else {
            return Ok(DeleteStatement::raw(
                delete.get_text(),
                crate::where_text::boxed(delete.where_clause()),
            ));
        };
        let mapped = self.apply_predicate(delete.where_clause(), cell)?;
        let clause: Option<&dyn IWhere> = match mapped.as_ref() {
            Some(where_clause) => Some(where_clause),
            None => delete.where_clause(),
        };
        Ok(DeleteStatement::new(self.entity(&table), clause))
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

fn map_on(on: &str, column: impl Fn(&str) -> String) -> String {
    match on.split_once(" = ") {
        Some((left, right)) if !left.contains(' ') && !right.contains(' ') => {
            format!("{} = {}", column(left), column(right))
        }
        _ => on.to_string(),
    }
}

fn last_dot(name: &str) -> Option<usize> {
    let mut quoted = false;
    let mut dot = None;
    for (index, ch) in name.char_indices() {
        if ch == '"' {
            quoted = !quoted;
        }
        if ch == '.' && !quoted {
            dot = Some(index);
        }
    }
    dot
}
