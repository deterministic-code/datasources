use crate::default_write_enrichment_strategy::{
    decode_rows, encode_rows, related_ids, required, row_key,
};
use crate::delete_enrichment_strategy::load;
use crate::enrichment::Enrichment;
use crate::repository::ICRUDRepository;
use crate::result::Result;
use crate::row::Row;
use crate::update_enrichment_strategy::UpdateEnrichmentStrategy;
use crate::value::Value;
use crate::{Operator, Where};
use async_trait::async_trait;
use std::collections::HashSet;
use std::sync::Arc;

pub struct DefaultUpdateEnrichmentStrategy;

#[async_trait]
impl UpdateEnrichmentStrategy for DefaultUpdateEnrichmentStrategy {
    async fn update_owned(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &mut Row,
    ) -> Result<()> {
        let existing = load(writer.as_ref(), enrichment, parent).await?;
        let mut related = decode_rows(parent.get(&enrichment.destination_property));
        if let Some(key) = parent.get(&enrichment.source_property).cloned() {
            for row in &mut related {
                row.insert(enrichment.related_key.clone(), key.clone());
            }
        }
        let ids = writer.id_names();
        let keep: HashSet<String> = related
            .iter()
            .filter_map(|row| row_key(row, &ids))
            .collect();
        for row in existing.iter().filter(|row| {
            row_key(row, &ids)
                .map(|key| !keep.contains(&key))
                .unwrap_or(true)
        }) {
            writer.delete(row, None).await?;
        }
        for row in &mut related {
            match row_key(row, &ids) {
                Some(key)
                    if existing
                        .iter()
                        .any(|stored| row_key(stored, &ids) == Some(key.clone())) =>
                {
                    writer.update(row, None).await?;
                }
                _ => {
                    writer.create(row).await?;
                }
            }
        }
        parent.insert(
            enrichment.destination_property.clone(),
            encode_rows(&related),
        );
        Ok(())
    }

    async fn update_linked(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &mut Row,
    ) -> Result<()> {
        let association = enrichment
            .association
            .as_ref()
            .ok_or_else(|| crate::DatasourceError::message("Missing association"))?;
        let parent_key = required(
            parent,
            &enrichment.source_property,
            &enrichment.destination_property,
        )?;
        let existing = writer
            .read(
                Some(&Where::new(Operator::equals(
                    association.parent.clone(),
                    parent_key.clone(),
                ))),
                None,
            )
            .await?;
        let wanted = related_ids(parent, enrichment)?;
        let keep: HashSet<String> = wanted.iter().map(Value::wire_text).collect();
        for row in &existing {
            let id = row.get(&association.related);
            if id
                .map(Value::wire_text)
                .is_some_and(|key| keep.contains(&key))
            {
                continue;
            }
            let clause = Where::new(Operator::and(
                Operator::equals(association.parent.clone(), parent_key.clone()),
                Operator::equals(
                    association.related.clone(),
                    id.cloned().unwrap_or(Value::Null),
                ),
            ));
            writer.delete(row, Some(&clause)).await?;
        }
        for id in wanted {
            if existing.iter().any(|row| {
                row.get(&association.related).map(Value::wire_text) == Some(id.wire_text())
            }) {
                continue;
            }
            let mut row = Row::new();
            row.insert(association.parent.clone(), parent_key.clone());
            row.insert(association.related.clone(), id);
            writer.create(&mut row).await?;
        }
        Ok(())
    }
}
