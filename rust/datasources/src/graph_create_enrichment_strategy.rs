use crate::create_enrichment_strategy::{children, set, CreateEnrichmentStrategy};
use crate::datasource_error::DatasourceError;
use crate::default_write_enrichment_strategy::required;
use crate::enrichment::Enrichment;
use crate::entity::Entity;
use crate::repository::ICRUDRepository;
use crate::result::Result;
use crate::tree_create_enrichment_strategy::node_key;
use crate::value::Value;
use async_trait::async_trait;
use std::any::Any;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

pub struct GraphCreateEnrichmentStrategy {
    insert_edge: Arc<
        dyn Fn(String, String, Value, Value) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>
            + Send
            + Sync,
    >,
    inserted: Mutex<HashMap<String, Value>>,
    depth: AtomicUsize,
}

impl GraphCreateEnrichmentStrategy {
    pub fn new<TLink: Entity>(link: Arc<dyn ICRUDRepository<TLink>>) -> Self {
        Self {
            insert_edge: Arc::new(move |parent_field, related_field, parent_id, related_id| {
                let link = Arc::clone(&link);
                Box::pin(async move {
                    let mut row = TLink::default();
                    set(&mut row, &parent_field, parent_id)?;
                    set(&mut row, &related_field, related_id)?;
                    link.create(row).await?;
                    Ok(())
                })
            }),
            inserted: Mutex::new(HashMap::new()),
            depth: AtomicUsize::new(0),
        }
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
        let association = enrichment
            .association
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Missing association"))?;
        let parent_id = required(parent, &enrichment.source_property, &enrichment.destination_property)?;
        self.inserted
            .lock()
            .expect("inserted")
            .insert(node_key(parent)?, parent_id.clone());
        let related = children::<T, TChild>(enrichment, parent, writer.schema())?;
        for child in related {
            let key = node_key(&child)?;
            let existing = {
                let guard = self.inserted.lock().expect("inserted");
                guard.get(&key).cloned()
            };
            let related_id = if let Some(existing) = existing {
                existing
            } else {
                let created = writer.create(child).await?;
                let related_id = required(
                    &created,
                    &enrichment.source_property,
                    &enrichment.destination_property,
                )?;
                self.inserted
                    .lock()
                    .expect("inserted")
                    .insert(key, related_id.clone());
                let mut nested = created;
                Box::pin(self.create_typed(Arc::clone(&writer), enrichment, &mut nested)).await?;
                related_id
            };
            (self.insert_edge)(
                association.parent.clone(),
                association.related.clone(),
                parent_id.clone(),
                related_id,
            )
            .await?;
        }
        Ok(())
    }
}

#[async_trait]
impl<T: Entity> CreateEnrichmentStrategy<T> for GraphCreateEnrichmentStrategy {
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
