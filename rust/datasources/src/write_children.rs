use futures_util::TryStreamExt;
use crate::child_writes::ChildWrites;
use crate::datasource_error::DatasourceError;
use crate::default_write_enrichment_strategy::{
    decode_rows, encode_rows, field_row, related_ids, required_field, row_key,
};
use crate::enrichment::Enrichment;
use crate::from_schema::FromSchema;
use crate::repository::{map_row, ICRUDRepository};
use crate::result::Result;
use crate::row::Row;
use crate::schema::Schema;
use crate::value::Value;
use crate::{Operator, Where};
use async_trait::async_trait;
use std::collections::HashSet;
use std::sync::Arc;

struct TypedChildWrites<TChild> {
    repo: Arc<dyn ICRUDRepository<TChild>>,
}

pub(crate) fn child_writes<T, TChild>(
    repo: Arc<dyn ICRUDRepository<TChild>>,
) -> Arc<dyn ChildWrites<T>>
where
    T: FromSchema + 'static,
    TChild: FromSchema + 'static,
{
    Arc::new(TypedChildWrites { repo })
}

#[async_trait]
impl<T, TChild> ChildWrites<T> for TypedChildWrites<TChild>
where
    T: FromSchema,
    TChild: FromSchema,
{
    async fn create_children(&self, enrichment: &Enrichment, parent: &mut T) -> Result<()> {
        create_children(self.repo.as_ref(), enrichment, parent).await
    }

    async fn update_children(&self, enrichment: &Enrichment, parent: &mut T) -> Result<()> {
        update_children(self.repo.as_ref(), enrichment, parent).await
    }

    async fn delete_children(&self, enrichment: &Enrichment, parent: &T) -> Result<()> {
        delete_children(self.repo.as_ref(), enrichment, parent).await
    }

    async fn create_linked(&self, enrichment: &Enrichment, parent: &T) -> Result<()> {
        create_links(self.repo.as_ref(), enrichment, parent).await
    }

    async fn update_linked(&self, enrichment: &Enrichment, parent: &T) -> Result<()> {
        update_links(self.repo.as_ref(), enrichment, parent).await
    }

    async fn delete_linked(&self, enrichment: &Enrichment, parent: &T) -> Result<()> {
        delete_links(self.repo.as_ref(), enrichment, parent).await
    }

    async fn write_collected(&self, children: &mut [Row], mode: crate::write_mode::WriteMode) -> Result<bool> {
        match mode {
            crate::write_mode::WriteMode::Create => {
                if let Some(write) = self.repo.bulk_create(children) {
                    write.await?;
                    return Ok(true);
                }
            }
            crate::write_mode::WriteMode::Update => {
                if let Some(write) = self.repo.bulk_update(children) {
                    write.await?;
                    return Ok(true);
                }
            }
            crate::write_mode::WriteMode::Delete => {}
        }
        Ok(false)
    }
}

pub(crate) fn stamp<T: FromSchema>(item: &mut T, name: &str, value: &Value) -> Result<()> {
    let mut row = field_row(item)?;
    row.insert(name.to_string(), value.clone());
    *item = map_row(row, &Schema::default())?;
    Ok(())
}

fn read_children<T: FromSchema, TChild: FromSchema>(parent: &T, name: &str) -> Result<Vec<TChild>> {
    let row = field_row(parent)?;
    decode_rows(row.get(name))
        .into_iter()
        .map(|child| map_row(child, &Schema::default()))
        .collect()
}

fn write_children<T: FromSchema, TChild: FromSchema>(
    parent: &mut T,
    name: &str,
    children: &[TChild],
) -> Result<()> {
    let rows = children
        .iter()
        .map(|child| child.to_schema(&Schema::default()))
        .collect::<Result<Vec<_>>>()?;
    let mut row = field_row(parent)?;
    row.insert(name.to_string(), encode_rows(&rows));
    *parent = map_row(row, &Schema::default())?;
    Ok(())
}

fn dict_key(row: &Row) -> String {
    row.get("Key")
        .or_else(|| row.get("key"))
        .map(Value::wire_text)
        .unwrap_or_default()
}

async fn create_children<T: FromSchema, TChild: FromSchema>(
    writer: &dyn ICRUDRepository<TChild>,
    enrichment: &Enrichment,
    parent: &mut T,
) -> Result<()> {
    let mut related = read_children::<T, TChild>(parent, &enrichment.destination_property)?;
    if enrichment.r#type == crate::EnrichmentPropertyType::Dictionary {
        let mut keyed = Vec::new();
        for child in related {
            let key = dict_key(&child.to_schema(&Schema::default())?);
            keyed.push((key, child));
        }
        keyed.sort_by(|left, right| left.0.cmp(&right.0));
        related = keyed.into_iter().map(|(_, child)| child).collect();
    }
    let key = field_row(parent)?.get(&enrichment.source_property).cloned();
    for child in &mut related {
        if let Some(key) = &key {
            stamp(child, &enrichment.related_key, key)?;
        }
        *child = writer.create(child.clone()).await?;
    }
    write_children(parent, &enrichment.destination_property, &related)
}

async fn create_links<T: FromSchema, TLink: FromSchema>(
    writer: &dyn ICRUDRepository<TLink>,
    enrichment: &Enrichment,
    parent: &T,
) -> Result<()> {
    let association = enrichment
        .association
        .as_ref()
        .ok_or_else(|| DatasourceError::message("Missing association"))?;
    let parent_key = required_field(
        parent,
        &enrichment.source_property,
        &enrichment.destination_property,
    )?;
    for id in related_ids(&field_row(parent)?, enrichment)? {
        let mut link = TLink::default();
        stamp(&mut link, &association.parent, &parent_key)?;
        stamp(&mut link, &association.related, &id)?;
        writer.create(link).await?;
    }
    Ok(())
}

pub(crate) async fn load_children<T: FromSchema, TChild: FromSchema>(
    writer: &dyn ICRUDRepository<TChild>,
    enrichment: &Enrichment,
    parent: &T,
) -> Result<Vec<TChild>> {
    let Some(key) = field_row(parent)?.get(&enrichment.source_property).cloned() else {
        return Ok(Vec::new());
    };
    writer
        .read(&Where::new(Operator::equals(enrichment.related_key.clone(), key)))
        .try_collect::<Vec<_>>()
        .await
}

async fn update_children<T: FromSchema, TChild: FromSchema>(
    writer: &dyn ICRUDRepository<TChild>,
    enrichment: &Enrichment,
    parent: &mut T,
) -> Result<()> {
    let existing = load_children(writer, enrichment, parent).await?;
    let mut related = read_children::<T, TChild>(parent, &enrichment.destination_property)?;
    if enrichment.r#type == crate::EnrichmentPropertyType::Dictionary {
        let mut keyed = Vec::new();
        for child in related {
            let key = dict_key(&child.to_schema(&Schema::default())?);
            keyed.push((key, child));
        }
        keyed.sort_by(|left, right| left.0.cmp(&right.0));
        related = keyed.into_iter().map(|(_, child)| child).collect();
    }
    if let Some(key) = field_row(parent)?.get(&enrichment.source_property).cloned() {
        for child in &mut related {
            stamp(child, &enrichment.related_key, &key)?;
        }
    }
    let ids = writer.id_names();
    let keep: HashSet<String> = related
        .iter()
        .filter_map(|child| row_key(&field_row(child).ok()?, &ids))
        .collect();
    for child in existing.iter().filter(|child| {
        field_row(*child)
            .ok()
            .and_then(|row| row_key(&row, &ids))
            .map(|key| !keep.contains(&key))
            .unwrap_or(true)
    }) {
        writer.delete(child.clone(), None).await?;
    }
    for child in &mut related {
        let key = field_row(child).ok().and_then(|row| row_key(&row, &ids));
        let stored = key.as_ref().is_some_and(|key| {
            existing.iter().any(|child| {
                field_row(child)
                    .ok()
                    .and_then(|row| row_key(&row, &ids))
                    .as_ref()
                    == Some(key)
            })
        });
        if stored {
            writer.update(child.clone(), None).await?;
        } else {
            *child = writer.create(child.clone()).await?;
        }
    }
    write_children(parent, &enrichment.destination_property, &related)
}

async fn delete_children<T: FromSchema, TChild: FromSchema>(
    writer: &dyn ICRUDRepository<TChild>,
    enrichment: &Enrichment,
    parent: &T,
) -> Result<()> {
    let existing = load_children(writer, enrichment, parent).await?;
    for child in existing {
        writer.delete(child, None).await?;
    }
    Ok(())
}

async fn update_links<T: FromSchema, TLink: FromSchema>(
    writer: &dyn ICRUDRepository<TLink>,
    enrichment: &Enrichment,
    parent: &T,
) -> Result<()> {
    let association = enrichment
        .association
        .as_ref()
        .ok_or_else(|| DatasourceError::message("Missing association"))?;
    let parent_key = required_field(
        parent,
        &enrichment.source_property,
        &enrichment.destination_property,
    )?;
    let existing = writer
        .read(&Where::new(Operator::equals(
            association.parent.clone(),
            parent_key.clone(),
        )))
        .try_collect::<Vec<_>>()
        .await?;
    let wanted = related_ids(&field_row(parent)?, enrichment)?;
    let keep: HashSet<String> = wanted.iter().map(Value::wire_text).collect();
    for row in &existing {
        let stored = field_row(row)?;
        let id = stored.get(&association.related);
        if id.map(Value::wire_text).is_some_and(|key| keep.contains(&key)) {
            continue;
        }
        writer.delete(row.clone(), None).await?;
    }
    for id in wanted {
        if existing.iter().any(|row| {
            field_row(row)
                .ok()
                .and_then(|stored| stored.get(&association.related).map(Value::wire_text))
                == Some(id.wire_text())
        }) {
            continue;
        }
        let mut link = TLink::default();
        stamp(&mut link, &association.parent, &parent_key)?;
        stamp(&mut link, &association.related, &id)?;
        writer.create(link).await?;
    }
    Ok(())
}

async fn delete_links<T: FromSchema, TLink: FromSchema>(
    writer: &dyn ICRUDRepository<TLink>,
    enrichment: &Enrichment,
    parent: &T,
) -> Result<()> {
    let association = enrichment
        .association
        .clone()
        .ok_or_else(|| DatasourceError::message("Missing association"))?;
    let Some(parent_key) = field_row(parent)?.get(&enrichment.source_property).cloned() else {
        return Ok(());
    };
    if parent_key == Value::Null {
        return Ok(());
    }
    let existing = writer
        .read(&Where::new(Operator::equals(
            association.parent.clone(),
            parent_key,
        )))
        .try_collect::<Vec<_>>()
        .await?;
    for row in existing {
        let stored = field_row(&row)?;
        if stored.get(&association.related).is_none() {
            continue;
        }
        writer.delete(row, None).await?;
    }
    Ok(())
}
