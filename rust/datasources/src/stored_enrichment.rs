use crate::bulk_crud_repository::BulkCRUDRepository;
use crate::cascade_delete_enrichment_strategy::{delete_children, delete_linked};
use crate::create_enrichment_strategy::CreateEnrichmentStrategy;
use crate::default_create_enrichment_strategy::{create_children, create_links};
use crate::graph_create_enrichment_strategy::GraphCreateEnrichmentStrategy;
use crate::tree_create_enrichment_strategy::TreeCreateEnrichmentStrategy;
use crate::default_update_enrichment_strategy::{update_children, update_linked};
use crate::enrichment::Enrichment;
use crate::enrichment_selection::EnrichmentSelection;
use crate::entity::Entity;
use crate::load_level_order_readonly_enrichment_strategy::load_level_order_apply_linked;
use crate::readonly_enrichment_strategy::{
    key_of, load_by_keys, ReadonlyEnrichmentStrategy, DEFAULT_FILL_BATCH_SIZE,
};
use crate::repository::{as_readonly, ICRUDRepository, IReadonlyRepository};
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use crate::write_mode::WriteMode;
use crate::{Operator, Where};
use async_trait::async_trait;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait]
pub(crate) trait AppliedRead<T: Entity>: Send + Sync {
    async fn apply(
        &self,
        enrichment: &Enrichment,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()>;

    async fn lookup(&self, property: &str, name: Value, key: &str) -> Result<Option<Value>>;

    /// Resolves every name in one batched read; names with no match are absent from the map.
    async fn lookup_batch(
        &self,
        property: &str,
        names: &[Value],
        key: &str,
    ) -> Result<HashMap<String, Value>> {
        let mut found = HashMap::new();
        for name in names {
            if let Some(value) = self.lookup(property, name.clone(), key).await? {
                found.insert(key_of(name), value);
            }
        }
        Ok(found)
    }
}

/// One `IN` read per chunk of names; the first row per name wins, as in a single-row lookup.
/// A name the `IN` read does not return (for example a case-insensitive collation) falls back
/// to the single-name read, so results match the per-name path.
async fn lookup_batch_via<TRelated: Entity>(
    via: &Arc<dyn IReadonlyRepository<TRelated>>,
    property: &str,
    names: &[Value],
    key: &str,
) -> Result<HashMap<String, Value>> {
    let loaded = load_by_keys(via.clone(), property, names, DEFAULT_FILL_BATCH_SIZE).await?;
    let mut found = HashMap::new();
    for item in loaded {
        let (Some(name), Some(value)) = (item.cell(property), item.cell(key)) else {
            continue;
        };
        found.entry(key_of(&name)).or_insert(value);
    }
    for name in names {
        if !found.contains_key(&key_of(name)) {
            let row = via
                .read_single_where(&Where::new(Operator::equals(property, name.clone())), None)
                .await?;
            if let Some(value) = row.and_then(|item| item.cell(key)) {
                found.insert(key_of(name), value);
            }
        }
    }
    Ok(found)
}

struct AppliedReadImpl<T, TRelated> {
    via: Arc<dyn IReadonlyRepository<TRelated>>,
    strategy: Arc<dyn ReadonlyEnrichmentStrategy<T, TRelated>>,
}

#[async_trait]
impl<T: Entity, TRelated: Entity> AppliedRead<T> for AppliedReadImpl<T, TRelated> {
    async fn apply(
        &self,
        enrichment: &Enrichment,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        self.strategy
            .apply(self.via.clone(), enrichment, parents, sources)
            .await
    }

    async fn lookup(&self, property: &str, name: Value, key: &str) -> Result<Option<Value>> {
        let found = self
            .via
            .read_single_where(&Where::new(Operator::equals(property, name)), None)
            .await?;
        Ok(found.and_then(|item| item.cell(key)))
    }

    async fn lookup_batch(
        &self,
        property: &str,
        names: &[Value],
        key: &str,
    ) -> Result<HashMap<String, Value>> {
        lookup_batch_via(&self.via, property, names, key).await
    }
}

struct AppliedLinkedImpl<T, TRelated, TLink> {
    via: Arc<dyn IReadonlyRepository<TRelated>>,
    link: Arc<dyn IReadonlyRepository<TLink>>,
    strategy: Arc<dyn ReadonlyEnrichmentStrategy<T, TRelated>>,
}

#[async_trait]
impl<T: Entity, TRelated: Entity, TLink: Entity> AppliedRead<T>
    for AppliedLinkedImpl<T, TRelated, TLink>
{
    async fn apply(
        &self,
        enrichment: &Enrichment,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        load_level_order_apply_linked(
            self.via.clone(),
            self.link.clone(),
            enrichment,
            parents,
            sources,
            self.strategy
                .linked_batch_size()
                .unwrap_or(DEFAULT_FILL_BATCH_SIZE),
        )
        .await
    }

    async fn lookup(&self, property: &str, name: Value, key: &str) -> Result<Option<Value>> {
        let found = self
            .via
            .read_single_where(&Where::new(Operator::equals(property, name)), None)
            .await?;
        Ok(found.and_then(|item| item.cell(key)))
    }

    async fn lookup_batch(
        &self,
        property: &str,
        names: &[Value],
        key: &str,
    ) -> Result<HashMap<String, Value>> {
        lookup_batch_via(&self.via, property, names, key).await
    }
}

#[async_trait]
pub(crate) trait AppliedPersist<T: Entity>: Send + Sync {
    async fn write(&self, enrichment: &Enrichment, parent: &mut T, mode: WriteMode) -> Result<()>;

    async fn write_create(
        &self,
        create: &dyn CreateEnrichmentStrategy<T>,
        enrichment: &Enrichment,
        parent: &mut T,
    ) -> Result<()>;

    fn persist_bulk(&self) -> Option<Arc<BulkCRUDRepository>> {
        None
    }

    fn row_writer(&self) -> Option<Arc<dyn ICRUDRepository>> {
        None
    }
}

struct AppliedPersistImpl<T, TChild> {
    repository: Arc<dyn ICRUDRepository<TChild>>,
    persist_bulk: Option<Arc<BulkCRUDRepository>>,
    entity: std::marker::PhantomData<T>,
}

#[async_trait]
impl<T: Entity, TChild: Entity> AppliedPersist<T> for AppliedPersistImpl<T, TChild> {
    async fn write(&self, enrichment: &Enrichment, parent: &mut T, mode: WriteMode) -> Result<()> {
        match mode {
            WriteMode::Create => {
                create_children(self.repository.as_ref(), enrichment, parent).await
            }
            WriteMode::Update => {
                update_children(self.repository.as_ref(), enrichment, parent).await
            }
            WriteMode::Delete => {
                delete_children(self.repository.as_ref(), enrichment, parent, &NoNested).await
            }
        }
    }

    async fn write_create(
        &self,
        create: &dyn CreateEnrichmentStrategy<T>,
        enrichment: &Enrichment,
        parent: &mut T,
    ) -> Result<()> {
        if let Some(tree) = create.as_any().downcast_ref::<TreeCreateEnrichmentStrategy>() {
            return tree
                .create_typed(self.repository.clone(), enrichment, parent)
                .await;
        }
        if let Some(graph) = create.as_any().downcast_ref::<GraphCreateEnrichmentStrategy>() {
            return graph
                .create_typed(self.repository.clone(), enrichment, parent)
                .await;
        }
        create_children(self.repository.as_ref(), enrichment, parent).await
    }

    fn persist_bulk(&self) -> Option<Arc<BulkCRUDRepository>> {
        self.persist_bulk.clone()
    }

    fn row_writer(&self) -> Option<Arc<dyn ICRUDRepository>> {
        as_row_crud(self.repository.clone())
    }
}

fn as_row_crud<TChild: Entity>(
    repository: Arc<dyn ICRUDRepository<TChild>>,
) -> Option<Arc<dyn ICRUDRepository>> {
    if TypeId::of::<TChild>() != TypeId::of::<Row>() {
        return None;
    }
    Some(unsafe {
        std::mem::transmute::<Arc<dyn ICRUDRepository<TChild>>, Arc<dyn ICRUDRepository>>(repository)
    })
}

struct AppliedLinkPersist<T, TLink> {
    repository: Arc<dyn ICRUDRepository<TLink>>,
    entity: std::marker::PhantomData<T>,
}

#[async_trait]
impl<T: Entity, TLink: Entity> AppliedPersist<T> for AppliedLinkPersist<T, TLink> {
    async fn write(&self, enrichment: &Enrichment, parent: &mut T, mode: WriteMode) -> Result<()> {
        match mode {
            WriteMode::Create => create_links(self.repository.as_ref(), enrichment, parent).await,
            WriteMode::Update => update_linked(self.repository.as_ref(), enrichment, parent).await,
            WriteMode::Delete => delete_linked(self.repository.as_ref(), enrichment, parent).await,
        }
    }

    async fn write_create(
        &self,
        _create: &dyn CreateEnrichmentStrategy<T>,
        enrichment: &Enrichment,
        parent: &mut T,
    ) -> Result<()> {
        create_links(self.repository.as_ref(), enrichment, parent).await
    }

    fn row_writer(&self) -> Option<Arc<dyn ICRUDRepository>> {
        as_row_crud(self.repository.clone())
    }
}

struct NoNested;

#[async_trait]
impl crate::delete_nested::DeleteNested for NoNested {
    async fn delete_nested(&self, _rows: &mut [Row]) -> Result<()> {
        Ok(())
    }
}

pub(crate) struct StoredEnrichment<T: Entity = Row> {
    pub enrichment: Enrichment,
    pub read: Arc<dyn AppliedRead<T>>,
    pub persist: Option<Arc<dyn AppliedPersist<T>>>,
    pub link: Option<Arc<dyn AppliedPersist<T>>>,
}

impl<T: Entity> StoredEnrichment<T> {
    pub(crate) fn direct<TRelated: Entity>(
        enrichment: Enrichment,
        via: Arc<dyn IReadonlyRepository<TRelated>>,
        persist: Option<Arc<dyn ICRUDRepository<TRelated>>>,
        strategy: Arc<dyn ReadonlyEnrichmentStrategy<T, TRelated>>,
    ) -> Self {
        Self::direct_bulk(enrichment, via, persist, None, strategy)
    }

    pub(crate) fn direct_bulk<TRelated: Entity>(
        enrichment: Enrichment,
        via: Arc<dyn IReadonlyRepository<TRelated>>,
        persist: Option<Arc<dyn ICRUDRepository<TRelated>>>,
        persist_bulk: Option<Arc<BulkCRUDRepository>>,
        strategy: Arc<dyn ReadonlyEnrichmentStrategy<T, TRelated>>,
    ) -> Self {
        let persist = persist.map(|repository| {
            Arc::new(AppliedPersistImpl {
                repository,
                persist_bulk: persist_bulk.clone(),
                entity: std::marker::PhantomData,
            }) as Arc<dyn AppliedPersist<T>>
        });
        Self {
            enrichment,
            read: Arc::new(AppliedReadImpl { via, strategy }),
            persist,
            link: None,
        }
    }

    pub(crate) fn linked<TRelated: Entity, TLink: Entity>(
        enrichment: Enrichment,
        via: Arc<dyn IReadonlyRepository<TRelated>>,
        link: Arc<dyn ICRUDRepository<TLink>>,
        strategy: Arc<dyn ReadonlyEnrichmentStrategy<T, TRelated>>,
    ) -> Self {
        Self {
            enrichment,
            read: Arc::new(AppliedLinkedImpl {
                via,
                link: as_readonly(link.clone()),
                strategy,
            }),
            persist: None,
            link: Some(Arc::new(AppliedLinkPersist {
                repository: link,
                entity: std::marker::PhantomData,
            })),
        }
    }

    pub(crate) fn selected(&self, selection: Option<&EnrichmentSelection>) -> bool {
        selection
            .map(|chosen| chosen.includes(&self.enrichment.destination_property))
            .unwrap_or(true)
    }

    pub(crate) async fn apply_selected(
        &self,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
        selection: Option<&EnrichmentSelection>,
    ) -> Result<()> {
        if !self.selected(selection) {
            return Ok(());
        }
        self.read.apply(&self.enrichment, parents, sources).await
    }

    pub(crate) async fn lookup(
        &self,
        property: &str,
        name: Value,
        key: &str,
    ) -> Result<Option<Value>> {
        self.read.lookup(property, name, key).await
    }

    pub(crate) async fn lookup_batch(
        &self,
        property: &str,
        names: &[Value],
        key: &str,
    ) -> Result<HashMap<String, Value>> {
        self.read.lookup_batch(property, names, key).await
    }
}

impl<T: Entity> Clone for StoredEnrichment<T> {
    fn clone(&self) -> Self {
        Self {
            enrichment: self.enrichment.clone(),
            read: self.read.clone(),
            persist: self.persist.clone(),
            link: self.link.clone(),
        }
    }
}
