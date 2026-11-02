use crate::bulk_crud_repository::BulkCRUDRepository;
use crate::entity::Entity;
use crate::repository::{as_readonly, ICRUDRepository, IReadonlyRepository};
use crate::row::Row;
use std::any::Any;
use std::sync::Arc;

pub trait EnrichmentVia<TRelated: Entity = Row> {
    fn read_via(self) -> Arc<dyn IReadonlyRepository<TRelated>>
    where
        Self: Sized,
    {
        self.into_repos().0
    }

    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<TRelated>>,
        Option<Arc<dyn ICRUDRepository<TRelated>>>,
    );

    fn bulk_repo(&self) -> Option<Arc<BulkCRUDRepository>> {
        None
    }
}

impl<T, TRelated> EnrichmentVia<TRelated> for Arc<T>
where
    T: IReadonlyRepository<TRelated> + Any + 'static,
    TRelated: Entity,
{
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<TRelated>>,
        Option<Arc<dyn ICRUDRepository<TRelated>>>,
    ) {
        let write = self.clone().to_crud();
        (self, write)
    }

    fn bulk_repo(&self) -> Option<Arc<BulkCRUDRepository>> {
        let any: Arc<dyn Any + Send + Sync> = self.clone();
        any.downcast::<BulkCRUDRepository>().ok()
    }
}

impl<TRelated: Entity> EnrichmentVia<TRelated> for Arc<dyn IReadonlyRepository<TRelated>> {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<TRelated>>,
        Option<Arc<dyn ICRUDRepository<TRelated>>>,
    ) {
        (self, None)
    }
}

impl<TRelated: Entity> EnrichmentVia<TRelated> for Arc<dyn ICRUDRepository<TRelated>> {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<TRelated>>,
        Option<Arc<dyn ICRUDRepository<TRelated>>>,
    ) {
        let write = self.clone();
        (as_readonly(self), Some(write))
    }
}
