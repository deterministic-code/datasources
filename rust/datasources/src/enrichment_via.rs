use crate::repository::{as_readonly, ICRUDRepository, IReadonlyRepository};
use std::sync::Arc;

pub trait EnrichmentVia {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository>,
        Option<Arc<dyn ICRUDRepository>>,
    );
}

impl<T> EnrichmentVia for Arc<T>
where
    T: IReadonlyRepository + 'static,
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
