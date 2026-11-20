use crate::create_enrichment_strategy::{children, missing, set, CreateEnrichmentStrategy};
use crate::datasource_error::DatasourceError;
use crate::default_write_enrichment_strategy::required;
use crate::enrichment::Enrichment;
use crate::entity::Entity;
use crate::repository::ICRUDRepository;
use crate::result::Result;
use crate::value::Value;
use async_trait::async_trait;
use std::any::Any;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct TreeCreateEnrichmentStrategy {
    inserted: Mutex<HashMap<String, Value>>,
    depth: AtomicUsize,
}

impl TreeCreateEnrichmentStrategy {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn create_typed<T: Entity, TChild: Entity>(
        &self,
        writer: Arc<dyn ICRUDRepository<TChild>>,
        enrichment: &Enrichment,
        parent: &mut T,
    ) -> Result<()> {
        self.depth.fetch_add(1, Ordering::SeqCst);
        let result = self
            .create_typed_inner(writer, enrichment, parent)
            .await;
        if self.depth.fetch_sub(1, Ordering::SeqCst) == 1 {
            self.inserted.lock().expect("inserted").clear();
        }
        result
    }

    async fn create_typed_inner<T: Entity, TChild: Entity>(
        &self,
        writer: Arc<dyn ICRUDRepository<TChild>>,
        enrichment: &Enrichment,
        parent: &mut T,
    ) -> Result<()> {
        let parent_id = required(parent, &enrichment.source_property, &enrichment.destination_property)?;
        self.inserted
            .lock()
            .expect("inserted")
            .insert(node_key(parent)?, parent_id);
        let related = children::<T, TChild>(enrichment, parent, writer.schema())?;
        for mut child in related {
            for (source, related_key) in enrichment.pairs() {
                let key = required(parent, &source, &enrichment.destination_property)?;
                set(&mut child, &related_key, key)?;
            }
            let key = node_key(&child)?;
            if self.inserted.lock().expect("inserted").contains_key(&key) {
                return Err(DatasourceError::message(format!(
                    "Tree create cycle at \"{key}\""
                )));
            }
            let created = writer.create(child).await?;
            let related_id = required(
                &created,
                &enrichment.source_property,
                &enrichment.destination_property,
            )?;
            self.inserted
                .lock()
                .expect("inserted")
                .insert(key, related_id);
            let mut nested = created;
            Box::pin(self.create_typed(Arc::clone(&writer), enrichment, &mut nested)).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl<T: Entity> CreateEnrichmentStrategy<T> for TreeCreateEnrichmentStrategy {
    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn create_children(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &mut T,
    ) -> Result<()> {
        self.create_typed(writer, enrichment, parent).await
    }

    async fn create_linked(
        &self,
        _writer: Arc<dyn ICRUDRepository>,
        _enrichment: &Enrichment,
        _parent: &mut T,
    ) -> Result<()> {
        Ok(())
    }
}

pub(crate) fn node_key<T: Entity>(item: &T) -> Result<String> {
    if let Some(name) = item
        .cell("Name")
        .or_else(|| item.cell("name"))
        .or_else(|| item.cell("Value"))
        .or_else(|| item.cell("value"))
        .filter(|value| !missing(value))
    {
        return Ok(name.wire_text());
    }
    if let Some(id) = item
        .cell("Id")
        .or_else(|| item.cell("id"))
        .filter(|value| !missing(value))
    {
        return Ok(id.wire_text());
    }
    let region = item.cell("RegionId").or_else(|| item.cell("region_id"));
    let category = item.cell("CategoryId").or_else(|| item.cell("category_id"));
    if let (Some(region), Some(category)) = (region, category) {
        if !missing(&region) && !missing(&category) {
            return Ok(format!("{}:{}", region.wire_text(), category.wire_text()));
        }
    }
    Err(DatasourceError::message(
        "Tree create requires Name, Value, Id, or RegionId/CategoryId",
    ))
}
