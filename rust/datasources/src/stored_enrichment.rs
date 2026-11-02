use crate::bulk_crud_repository::BulkCRUDRepository;
use crate::cascade_delete_enrichment_strategy::{delete_children, delete_linked};
use crate::default_create_enrichment_strategy::{create_children, create_links};
use crate::default_update_enrichment_strategy::{update_children, update_linked};
use crate::enrichment::Enrichment;
use crate::entity::Entity;
use crate::readonly_enrichment_strategy::{apply_linked, ReadonlyEnrichmentStrategy};
use crate::repository::{as_readonly, ICRUDRepository, IReadonlyRepository};
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use crate::write_mode::WriteMode;
use crate::{Operator, Where};
use async_trait::async_trait;
use std::any::TypeId;
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
            .read_single(&Where::new(Operator::equals(property, name)))
            .await?;
        Ok(found.and_then(|item| item.cell(key)))
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
        let _ = &self.strategy;
        apply_linked(
            self.via.clone(),
            self.link.clone(),
            enrichment,
            parents,
            sources,
        )
        .await
    }

    async fn lookup(&self, property: &str, name: Value, key: &str) -> Result<Option<Value>> {
        let found = self
            .via
            .read_single(&Where::new(Operator::equals(property, name)))
            .await?;
        Ok(found.and_then(|item| item.cell(key)))
    }
}

#[async_trait]
pub(crate) trait AppliedPersist<T: Entity>: Send + Sync {
    async fn write(&self, enrichment: &Enrichment, parent: &mut T, mode: WriteMode) -> Result<()>;

    #[allow(dead_code)]
    async fn write_bulk(
        &self,
        enrichment: &Enrichment,
        parents: &mut [T],
        mode: WriteMode,
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

    async fn write_bulk(
        &self,
        enrichment: &Enrichment,
        parents: &mut [T],
        mode: WriteMode,
    ) -> Result<()> {
        for parent in parents {
            self.write(enrichment, parent, mode).await?;
        }
        Ok(())
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

    async fn write_bulk(
        &self,
        enrichment: &Enrichment,
        parents: &mut [T],
        mode: WriteMode,
    ) -> Result<()> {
        for parent in parents {
            self.write(enrichment, parent, mode).await?;
        }
        Ok(())
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
    linked: bool,
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
            linked: false,
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
            linked: true,
        }
    }

    pub(crate) async fn apply(
        &self,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
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
}

impl<T: Entity> Clone for StoredEnrichment<T> {
    fn clone(&self) -> Self {
        Self {
            enrichment: self.enrichment.clone(),
            read: self.read.clone(),
            persist: self.persist.clone(),
            link: self.link.clone(),
            linked: self.linked,
        }
    }
}
