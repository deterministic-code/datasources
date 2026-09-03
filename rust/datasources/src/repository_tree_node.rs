use crate::builders::WhereBuilder;
use crate::enrichment::Enrichment;
use crate::repository::ReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct RepositoryTreeNode {
    pub value: Arc<dyn ReadonlyRepository>,
    pub property_name: String,
    pub items: Vec<RepositoryTreeNode>,
    enrichment: Option<Enrichment>,
}

impl RepositoryTreeNode {
    pub fn new(value: Arc<dyn ReadonlyRepository>, property_name: impl Into<String>) -> Self {
        Self {
            value,
            property_name: property_name.into(),
            items: Vec::new(),
            enrichment: None,
        }
    }

    pub fn include(
        &mut self,
        via: Arc<dyn ReadonlyRepository>,
        into: impl Into<String>,
        from: impl Into<String>,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
    ) -> &mut Self {
        let into = into.into();
        let enrichment = Enrichment::new(into.clone(), from, by, select, many);
        self.items.push(Self {
            value: via,
            property_name: into,
            items: Vec::new(),
            enrichment: Some(enrichment),
        });
        self
    }

    pub fn enrich_from<'a>(
        &'a self,
        parents: &'a mut [Row],
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            if let Some(enrichment) = &self.enrichment {
                apply(self.value.as_ref(), enrichment, parents).await?;
            }
            for child in &self.items {
                child.enrich_from(parents).await?;
            }
            Ok(())
        })
    }
}

async fn apply(
    via: &dyn ReadonlyRepository,
    enrichment: &Enrichment,
    parents: &mut [Row],
) -> Result<()> {
    if enrichment.many {
        apply_many(via, enrichment, parents).await
    } else {
        apply_lookup(via, enrichment, parents).await
    }
}

async fn apply_lookup(
    via: &dyn ReadonlyRepository,
    enrichment: &Enrichment,
    parents: &mut [Row],
) -> Result<()> {
    let mut cache: HashMap<String, Option<Row>> = HashMap::new();
    for parent in parents.iter_mut() {
        let Some(key) = parent.get(&enrichment.from).cloned() else {
            continue;
        };
        let cache_key = key.wire_text();
        if !cache.contains_key(&cache_key) {
            let related = via
                .get_single_where(&WhereBuilder::equals(enrichment.by.clone(), key.clone()))
                .await?;
            cache.insert(cache_key.clone(), related);
        }
        let Some(related) = cache.get(&cache_key).and_then(|row| row.as_ref()) else {
            continue;
        };
        let value = enrichment
            .select
            .as_ref()
            .and_then(|select| related.get(select).cloned())
            .unwrap_or_else(|| encode_row(related));
        parent.insert(enrichment.into.clone(), value);
    }
    Ok(())
}

async fn apply_many(
    via: &dyn ReadonlyRepository,
    enrichment: &Enrichment,
    parents: &mut [Row],
) -> Result<()> {
    for parent in parents.iter_mut() {
        let Some(key) = parent.get(&enrichment.from).cloned() else {
            continue;
        };
        let related = via
            .get_where(&WhereBuilder::equals(enrichment.by.clone(), key))
            .await?;
        parent.insert(enrichment.into.clone(), encode_rows(&related));
    }
    Ok(())
}

fn encode_rows(rows: &[Row]) -> Value {
    Value::Text(
        rows.iter()
            .map(encode_row_text)
            .collect::<Vec<_>>()
            .join("|"),
    )
}

fn encode_row(row: &Row) -> Value {
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
