use crate::bulk_crud_repository::BulkCRUDRepository;
use crate::repository::{as_readonly, ICRUDRepository, IReadonlyRepository};
use std::any::Any;
use std::sync::Arc;

pub trait EnrichmentVia {
    fn read_via(self) -> Arc<dyn IReadonlyRepository>
    where
        Self: Sized,
    {
        self.into_repos().0
    }

    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository>,
        Option<Arc<dyn ICRUDRepository>>,
    );

    fn bulk_repo(&self) -> Option<Arc<BulkCRUDRepository>> {
        None
    }
}

impl<T> EnrichmentVia for Arc<T>
where
    T: IReadonlyRepository + Any + 'static,
{
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository>,
        Option<Arc<dyn ICRUDRepository>>,
    ) {
        let write = self.clone().to_crud();
        (self, write)
    }

    fn bulk_repo(&self) -> Option<Arc<BulkCRUDRepository>> {
        let any: Arc<dyn Any + Send + Sync> = self.clone();
        any.downcast::<BulkCRUDRepository>().ok()
    }
}

impl EnrichmentVia for Arc<dyn IReadonlyRepository> {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository>,
        Option<Arc<dyn ICRUDRepository>>,
    ) {
        (self, None)
    }
}

impl EnrichmentVia for Arc<dyn ICRUDRepository> {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository>,
        Option<Arc<dyn ICRUDRepository>>,
    ) {
        let write = self.clone();
        (as_readonly(self), Some(write))
    }
}
