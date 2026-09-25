use crate::association::Association;
use crate::default_readonly_enrichment_strategy::DefaultReadonlyEnrichmentStrategy;
use crate::enrichment::Enrichment;
use crate::enrichment_via::EnrichmentVia;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use crate::{Operator, Where};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct RepositoryTreeNode {
    pub value: Arc<dyn IReadonlyRepository>,
    pub write: Option<Arc<dyn ICRUDRepository>>,
    pub link: Option<Arc<dyn ICRUDRepository>>,
    pub property_name: String,
    pub items: Vec<RepositoryTreeNode>,
    pub enrichment: Option<Enrichment>,
    binder: Arc<dyn ReadonlyEnrichmentStrategy>,
}

impl RepositoryTreeNode {
    pub fn new(value: Arc<dyn IReadonlyRepository>, property_name: impl Into<String>) -> Self {
        Self::with_binder(
            value,
            property_name,
            Arc::new(DefaultReadonlyEnrichmentStrategy),
        )
    }

    pub fn with_binder(
        value: Arc<dyn IReadonlyRepository>,
        property_name: impl Into<String>,
        binder: Arc<dyn ReadonlyEnrichmentStrategy>,
    ) -> Self {
        Self {
            value,
            write: None,
            link: None,
            property_name: property_name.into(),
            items: Vec::new(),
            enrichment: None,
            binder,
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
        self.include_with(via, into, from, by, select, many, self.binder.clone())
    }

    pub(crate) fn include_with(
        &mut self,
        via: impl EnrichmentVia,
        into: impl Into<String>,
        from: impl Into<String>,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
        strategy: Arc<dyn ReadonlyEnrichmentStrategy>,
    ) -> &mut Self {
        let into = into.into();
        let (value, write) = via.into_repos();
        let enrichment = Enrichment::new(into.clone(), from, by, select, many);
        self.items.push(Self {
            value,
            write,
            link: None,
            property_name: into,
            items: Vec::new(),
            enrichment: Some(enrichment),
            binder: strategy,
        });
        self.items.last_mut().expect("include")
    }

    pub fn include_linked(
        &mut self,
        via: impl EnrichmentVia,
        into: impl Into<String>,
        from: impl Into<String>,
        by: impl Into<String>,
        link: Arc<dyn ICRUDRepository>,
        parent: impl Into<String>,
        related: impl Into<String>,
    ) -> &mut Self {
        self.include_linked_with(
            via,
            into,
            from,
            by,
            link,
            parent,
            related,
            self.binder.clone(),
        )
    }

    pub(crate) fn include_linked_with(
        &mut self,
        via: impl EnrichmentVia,
        into: impl Into<String>,
        from: impl Into<String>,
        by: impl Into<String>,
        link: Arc<dyn ICRUDRepository>,
        parent: impl Into<String>,
        related: impl Into<String>,
        strategy: Arc<dyn ReadonlyEnrichmentStrategy>,
    ) -> &mut Self {
        let into = into.into();
        let (value, _) = via.into_repos();
        let enrichment = Enrichment::new(into.clone(), from, by, None, true)
            .linked(Association {
                parent: parent.into(),
                related: related.into(),
            });
        self.items.push(Self {
            value,
            write: None,
            link: Some(link),
            property_name: into,
            items: Vec::new(),
            enrichment: Some(enrichment),
            binder: strategy,
        });
        self.items.last_mut().expect("include_linked")
    }

    pub fn enrich_from<'a>(
        &'a self,
        parents: &'a mut [Row],
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        self.enrich_from_sourced(parents, None)
    }

    pub(crate) fn enrich_from_sourced<'a>(
        &'a self,
        parents: &'a mut [Row],
        sources: Option<&'a [Option<Row>]>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            self.apply_node(parents, sources).await?;
            if self.property_name.is_empty() {
                for child in &self.items {
                    child.enrich_from_sourced(parents, sources).await?;
                }
            } else if !self.items.is_empty() {
                let mut next = related(parents, &self.property_name);
                for child in &self.items {
                    child.enrich_from_sourced(&mut next, None).await?;
                }
                write_related(parents, &self.property_name, &next);
            }
            Ok(())
        })
    }

    async fn apply_node(
        &self,
        parents: &mut [Row],
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let Some(enrichment) = &self.enrichment else {
            return Ok(());
        };
        if enrichment.association.is_some() {
            let link = self
                .link
                .clone()
                .expect("linked enrichment requires a link repository");
            return apply_linked(self.value.clone(), link, enrichment, parents, sources).await;
        }
        self.binder
            .apply(self.value.clone(), enrichment, parents, sources)
            .await
    }

    pub fn key_of(key: &Value) -> String {
        key.wire_text()
    }
}

async fn apply_linked(
    via: Arc<dyn IReadonlyRepository>,
    link: Arc<dyn ICRUDRepository>,
    enrichment: &Enrichment,
    parents: &mut [Row],
    sources: Option<&[Option<Row>]>,
) -> Result<()> {
    let association = enrichment.association.as_ref().expect("association");
    for (index, parent) in parents.iter_mut().enumerate() {
        let Some(key) = source_key(parent, &enrichment.from, at(sources, index)) else {
            continue;
        };
        let links = link
            .read(
                Some(&Where::new(Operator::equals(
                    association.parent.clone(),
                    key,
                ))),
                None,
            )
            .await?;
        let mut related_rows = Vec::new();
        for row in links {
            let Some(id) = row.get(&association.related).cloned() else {
                continue;
            };
            if let Some(entity) = via
                .read_single(
                    Some(&Where::new(Operator::equals(
                        enrichment.by.clone(),
                        id,
                    ))),
                    None,
                )
                .await?
            {
                related_rows.push(entity);
            }
        }
        parent.insert(enrichment.into.clone(), encode_rows(&related_rows));
    }
    Ok(())
}

pub fn source_key(parent: &Row, name: &str, source: Option<&Row>) -> Option<Value> {
    if let Some(source) = source {
        if let Some(cell) = source.get(name) {
            if *cell != Value::Null {
                return Some(cell.clone());
            }
        }
    }
    parent
        .get(name)
        .filter(|value| **value != Value::Null)
        .cloned()
}

fn at<'a>(sources: Option<&'a [Option<Row>]>, index: usize) -> Option<&'a Row> {
    sources
        .and_then(|rows| rows.get(index))
        .and_then(|row| row.as_ref())
}

fn related(parents: &[Row], property_name: &str) -> Vec<Row> {
    if property_name.is_empty() {
        return parents.to_vec();
    }
    let mut next = Vec::new();
    for parent in parents {
        match parent.get(property_name) {
            Some(Value::Text(text)) if !text.is_empty() => {
                for part in text.split('|') {
                    next.push(decode_row(part));
                }
            }
            _ => {}
        }
    }
    next
}

fn write_related(parents: &mut [Row], property_name: &str, rows: &[Row]) {
    let mut offset = 0;
    for parent in parents.iter_mut() {
        let count = match parent.get(property_name) {
            Some(Value::Text(text)) if !text.is_empty() => text.split('|').count(),
            _ => 0,
        };
        if count == 0 {
            continue;
        }
        let slice = &rows[offset..offset + count];
        offset += count;
        parent.insert(property_name.to_string(), encode_rows(slice));
    }
}

fn decode_row(text: &str) -> Row {
    text.split(',')
        .filter_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            Some((key.to_string(), parse_value(value)))
        })
        .collect()
}

fn parse_value(text: &str) -> Value {
    match text {
        "true" => Value::Bool(true),
        "false" => Value::Bool(false),
        _ => text
            .parse::<i64>()
            .map(Value::Int)
            .unwrap_or_else(|_| Value::Text(text.into())),
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
