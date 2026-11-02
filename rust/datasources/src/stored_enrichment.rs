use crate::child_writes::ChildWrites;
use crate::enrichment::Enrichment;
use crate::from_schema::FromSchema;
use crate::readonly_enrichment_strategy::{apply_linked, ReadonlyEnrichmentStrategy};
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

type EnrichApply<T> = Arc<
    dyn for<'a> Fn(
            &'a Enrichment,
            &'a mut Vec<T>,
            Option<&'a [Option<Row>]>,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>
        + Send
        + Sync,
>;

pub(crate) type LookupFn<T> = Arc<
    dyn for<'a> Fn(&'a mut T) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> + Send + Sync,
>;

pub(crate) struct WriteEnrichment<T: FromSchema = Row> {
    pub enrichment: Enrichment,
    pub persist: Option<Arc<dyn ChildWrites<T>>>,
    pub link: Option<Arc<dyn ChildWrites<T>>>,
    pub lookup: Option<LookupFn<T>>,
}

impl<T: FromSchema> Clone for WriteEnrichment<T> {
    fn clone(&self) -> Self {
        Self {
            enrichment: self.enrichment.clone(),
            persist: self.persist.clone(),
            link: self.link.clone(),
            lookup: self.lookup.clone(),
        }
    }
}

pub(crate) struct StoredEnrichment<T: FromSchema> {
    pub enrichment: Enrichment,
    apply: EnrichApply<T>,
}

impl<T: FromSchema> StoredEnrichment<T> {
    pub(crate) fn boxed_apply<TRelated, S>(
        via: Arc<dyn IReadonlyRepository<TRelated>>,
        strategy: Arc<S>,
    ) -> EnrichApply<T>
    where
        TRelated: FromSchema,
        S: ReadonlyEnrichmentStrategy<T, TRelated> + ?Sized + 'static,
    {
        Arc::new(move |enrichment, parents, sources| {
            let via = via.clone();
            let strategy = strategy.clone();
            Box::pin(async move { strategy.apply(via, enrichment, parents, sources).await })
        })
    }

    pub(crate) fn boxed_linked<TRelated, TLink>(
        via: Arc<dyn IReadonlyRepository<TRelated>>,
        link: Arc<dyn IReadonlyRepository<TLink>>,
    ) -> EnrichApply<T>
    where
        TRelated: FromSchema,
        TLink: FromSchema,
    {
        Arc::new(move |enrichment, parents, sources| {
            let via = via.clone();
            let link = link.clone();
            Box::pin(async move { apply_linked(via, link, enrichment, parents, sources).await })
        })
    }

    pub(crate) fn new(enrichment: Enrichment, apply: EnrichApply<T>) -> Self {
        Self { enrichment, apply }
    }

    pub(crate) async fn apply(
        &self,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        (self.apply)(&self.enrichment, parents, sources).await
    }
}

impl<T: FromSchema> Clone for StoredEnrichment<T> {
    fn clone(&self) -> Self {
        Self {
            enrichment: self.enrichment.clone(),
            apply: self.apply.clone(),
        }
    }
}
