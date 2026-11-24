use crate::entity::Entity;
use crate::readonly_enrichment_strategy::{key_of, DEFAULT_FILL_BATCH_SIZE};
use crate::repository::ICRUDRepository;
use crate::result::Result;
use crate::value::Value;
use crate::{Operator, Where};
use futures_util::TryStreamExt;
use std::collections::{HashMap, HashSet};

/// Loads, with one `IN` read per chunk of keys, the rows of `writer` whose `column` equals the
/// `source` cell of each parent. The result lines up with `parents`; a parent with no key, or no
/// rows, gets an empty list, and rows keep the order the datasource returned them in.
pub(crate) async fn load_grouped<T: Entity, TItem: Entity>(
    writer: &dyn ICRUDRepository<TItem>,
    column: &str,
    source: &str,
    parents: &[&T],
) -> Result<Vec<Vec<TItem>>> {
    let keys: Vec<Option<Value>> = parents
        .iter()
        .map(|parent| parent.cell(source).filter(|value| *value != Value::Null))
        .collect();
    let mut distinct = Vec::new();
    let mut seen = HashSet::new();
    for key in keys.iter().flatten() {
        if seen.insert(key_of(key)) {
            distinct.push(key.clone());
        }
    }
    let mut groups: HashMap<String, Vec<TItem>> = HashMap::new();
    for chunk in distinct.chunks(DEFAULT_FILL_BATCH_SIZE) {
        let rows = writer
            .read_where(
                &Where::new(Operator::in_list(column.to_string(), chunk.to_vec())),
                None,
            )
            .try_collect::<Vec<_>>()
            .await?;
        for row in rows {
            if let Some(cell) = row.cell(column) {
                groups.entry(key_of(&cell)).or_default().push(row);
            }
        }
    }
    Ok(keys
        .iter()
        .map(|key| {
            key.as_ref()
                .and_then(|key| groups.get(&key_of(key)).cloned())
                .unwrap_or_default()
        })
        .collect())
}
