use crate::default_readonly_enrichment_strategy::DefaultReadonlyEnrichmentStrategy;
use crate::enrichment::Enrichment;
use crate::enrichment_via::EnrichmentVia;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct RepositoryTreeNode {
    pub value: Arc<dyn IReadonlyRepository>,
    pub write: Option<Arc<dyn ICRUDRepository>>,
    pub property_name: String,
    pub items: Vec<RepositoryTreeNode>,
    pub enrichment: Option<Enrichment>,
    pub strategy: Arc<dyn ReadonlyEnrichmentStrategy>,
}

impl RepositoryTreeNode {
    pub fn new(value: Arc<dyn IReadonlyRepository>, property_name: impl Into<String>) -> Self {
        Self::with_strategy(
            value,
            property_name,
            Arc::new(DefaultReadonlyEnrichmentStrategy),
        )
    }

    pub fn with_strategy(
        value: Arc<dyn IReadonlyRepository>,
        property_name: impl Into<String>,
        strategy: Arc<dyn ReadonlyEnrichmentStrategy>,
    ) -> Self {
        Self {
            value,
            write: None,
            property_name: property_name.into(),
            items: Vec::new(),
            enrichment: None,
            strategy,
        }
    }

    pub fn include(
        &mut self,
        via: impl EnrichmentVia,
        into: impl Into<String>,
        from: impl Into<String>,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
    ) -> &mut Self {
        let into = into.into();
        let (value, write) = via.into_repos();
        let enrichment = Enrichment::new(into.clone(), from, by, select, many);
        self.items.push(Self {
            value,
            write,
            property_name: into,
            items: Vec::new(),
            enrichment: Some(enrichment),
            strategy: self.strategy.clone(),
        });
        self.items.last_mut().expect("include")
    }

    pub fn enrich_from<'a>(
        &'a self,
        parents: &'a mut [Row],
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move { self.strategy.apply(self, parents).await })
    }
}

pub fn encode_rows(rows: &[Row]) -> Value {
    Value::Text(
        rows.iter()
            .map(encode_row_text)
            .collect::<Vec<_>>()
            .join("|"),
    )
}

pub(crate) fn encode_row(row: &Row) -> Value {
    Value::Text(encode_row_text(row))
}

fn encode_row_text(row: &Row) -> String {
    let mut keys: Vec<_> = row.keys().cloned().collect();
    keys.sort();
    keys.into_iter()
        .filter_map(|key| {
            row.get(&key)
                .map(|value| format!("{key}={}", value.wire_text()))
        })
        .collect::<Vec<_>>()
        .join(",")
}
