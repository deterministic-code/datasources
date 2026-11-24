use async_trait::async_trait;
use datasources::*;
use datasources_memory::MemoryDriver;
use futures_core::Stream;
use futures_util::TryStreamExt;
use std::any::TypeId;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

fn record(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), value.clone()))
        .collect()
}

async fn setup() -> Arc<dyn Connection> {
    let connection = MemoryDriver.connection(None);
    connection.open().await.unwrap();
    connection
}

fn num(row: &Row, key: &str) -> i64 {
    match row.get(key) {
        Some(Value::Int(id)) => *id,
        other => panic!("{key}: {other:?}"),
    }
}

fn text(row: &Row, key: &str) -> String {
    match row.get(key) {
        Some(Value::Text(value)) => value.clone(),
        _ => String::new(),
    }
}

fn crud(connection: Arc<dyn Connection>, entity: &str) -> CRUDRepository {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = entity.into();
    options.ids = vec!["id".into()];
    CRUDRepository::<Row>::new(connection, options).unwrap()
}

struct CountingCrud<T: Entity> {
    inner: Arc<dyn ICRUDRepository<T>>,
    reads: Arc<Mutex<usize>>,
}

impl<T: Entity> IRepository for CountingCrud<T> {
    fn entity_type(&self) -> TypeId {
        self.inner.entity_type()
    }

    fn schema(&self) -> &Schema {
        self.inner.schema()
    }
}

#[async_trait]
impl<T: Entity> IReadonlyRepository<T> for CountingCrud<T> {
    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<T>> {
        *self.reads.lock().unwrap() += 1;
        self.inner.read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>> {
        *self.reads.lock().unwrap() += 1;
        self.inner.read(query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<T>> {
        *self.reads.lock().unwrap() += 1;
        self.inner.read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>> {
        *self.reads.lock().unwrap() += 1;
        self.inner.read_where(clause, options)
    }
}

#[async_trait]
impl<T: Entity> ICRUDRepository<T> for CountingCrud<T> {
    async fn create(&self, item: T) -> Result<T> {
        self.inner.create(item).await
    }

    async fn update(
        &self,
        item: T,
        by_where: Option<&dyn IWhere>,
    ) -> Result<Response> {
        self.inner.update(item, by_where).await
    }

    async fn delete(
        &self,
        item: T,
        by_where: Option<&dyn IWhere>,
    ) -> Result<Response> {
        self.inner.delete(item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<T>> {
        self.inner.read_single_by_id(id).await
    }
}

const PARENTS: i64 = 100;

struct NoNested;

#[async_trait]
impl DeleteNested for NoNested {
    async fn delete_nested(&self, _rows: &mut [Row]) -> Result<()> {
        Ok(())
    }
}

fn children_fill() -> Enrichment {
    Enrichment::new("children", "id", "parent_id", None, EnrichmentPropertyType::List)
}

fn links_fill() -> Enrichment {
    Enrichment::new("tags", "id", "id", None, EnrichmentPropertyType::List)
        .linked(Association::new("parent_id", "tag_id"))
}

/// `PARENTS` stored parents, each with one child, and the child repository wrapped to count reads.
struct Fixture {
    parents: Vec<Row>,
    children: Arc<CRUDRepository>,
    counted: Arc<dyn ICRUDRepository<Row>>,
    reads: Arc<Mutex<usize>>,
}

async fn fixture(child_entity: &str, owner_column: &str) -> Fixture {
    let connection = setup().await;
    let parents_repo = crud(connection.clone(), "Parent");
    let children = Arc::new(crud(connection, child_entity));
    let mut parents = Vec::new();
    for i in 0..PARENTS {
        let parent = parents_repo
            .create(record(&[("label", Value::from(format!("p{i}")))]))
            .await
            .unwrap();
        children
            .create(record(&[
                (owner_column, parent.get("id").unwrap().clone()),
                ("label", Value::from(format!("child{i}"))),
            ]))
            .await
            .unwrap();
        parents.push(parent);
    }
    let reads = Arc::new(Mutex::new(0));
    let counted: Arc<dyn ICRUDRepository<Row>> = Arc::new(CountingCrud {
        inner: children.clone(),
        reads: reads.clone(),
    });
    Fixture { parents, children, counted, reads }
}

async fn stored(children: &CRUDRepository) -> Vec<Row> {
    children.read(None, None).try_collect::<Vec<_>>().await.unwrap()
}

fn with_changed_children(f: &Fixture, rows: &[Row]) -> Vec<Row> {
    f.parents
        .iter()
        .map(|parent| {
            let child = rows
                .iter()
                .find(|row| row.get("parent_id") == parent.get("id"))
                .unwrap();
            let mut changed = child.clone();
            changed.insert("label".into(), Value::from(format!("{}!", text(child, "label"))));
            let mut row = parent.clone();
            row.insert("children".into(), encode_rows(&[changed]));
            row
        })
        .collect()
}

fn labels(rows: &[Row]) -> Vec<String> {
    let mut labels: Vec<String> = rows.iter().map(|row| text(row, "label")).collect();
    labels.sort();
    labels
}

#[tokio::test]
async fn update_children_batch_reads_once_and_matches_the_default_strategy() {
    let batch = fixture("Child", "parent_id").await;
    let default = fixture("Child", "parent_id").await;
    let mut batch_rows = with_changed_children(&batch, &stored(&batch.children).await);
    let mut default_rows = with_changed_children(&default, &stored(&default.children).await);
    *batch.reads.lock().unwrap() = 0;
    let mut targets: Vec<&mut Row> = batch_rows.iter_mut().collect();
    BatchUpdateEnrichmentStrategy
        .update_children_batch(batch.counted.clone(), &children_fill(), &mut targets)
        .await
        .unwrap();
    assert_eq!(*batch.reads.lock().unwrap(), 1);
    for row in default_rows.iter_mut() {
        DefaultUpdateEnrichmentStrategy
            .update_children(default.counted.clone(), &children_fill(), row)
            .await
            .unwrap();
    }
    assert_eq!(*default.reads.lock().unwrap() as i64, PARENTS);
    let after = stored(&batch.children).await;
    assert_eq!(after.len() as i64, PARENTS);
    assert_eq!(labels(&after), labels(&stored(&default.children).await));
    assert!(after.iter().all(|row| text(row, "label").ends_with('!')));
}

#[tokio::test]
async fn update_children_batch_deletes_children_dropped_from_a_parent() {
    let f = fixture("Child", "parent_id").await;
    let mut rows: Vec<Row> = f
        .parents
        .iter()
        .map(|parent| {
            let mut row = parent.clone();
            row.insert("children".into(), encode_rows(&[]));
            row
        })
        .collect();
    let mut targets: Vec<&mut Row> = rows.iter_mut().collect();
    BatchUpdateEnrichmentStrategy
        .update_children_batch(f.counted.clone(), &children_fill(), &mut targets)
        .await
        .unwrap();
    assert!(stored(&f.children).await.is_empty());
}

#[tokio::test]
async fn delete_children_batch_reads_once_and_matches_the_default_strategy() {
    let batch = fixture("Child", "parent_id").await;
    let default = fixture("Child", "parent_id").await;
    // keep one parent's child out of the delete so a wrong match would show
    let keep = batch.parents.len() - 1;
    let batch_parents: Vec<&Row> = batch.parents[..keep].iter().collect();
    *batch.reads.lock().unwrap() = 0;
    BatchDeleteEnrichmentStrategy
        .delete_children_batch(batch.counted.clone(), &children_fill(), &batch_parents, &NoNested)
        .await
        .unwrap();
    assert_eq!(*batch.reads.lock().unwrap(), 1);
    for parent in &default.parents[..keep] {
        CascadeDeleteEnrichmentStrategy
            .delete_children(default.counted.clone(), &children_fill(), parent, &NoNested)
            .await
            .unwrap();
    }
    assert_eq!(*default.reads.lock().unwrap(), keep);
    let left = stored(&batch.children).await;
    assert_eq!(left.len(), 1);
    assert_eq!(labels(&left), labels(&stored(&default.children).await));
}

#[tokio::test]
async fn delete_linked_batch_reads_once_and_matches_the_default_strategy() {
    let batch = fixture("ParentTag", "parent_id").await;
    let default = fixture("ParentTag", "parent_id").await;
    for f in [&batch, &default] {
        for (i, row) in stored(&f.children).await.into_iter().enumerate() {
            let mut linked = row.clone();
            linked.insert("tag_id".into(), Value::Int(i as i64 + 1));
            f.children.update(linked, None).await.unwrap();
        }
    }
    let keep = batch.parents.len() - 1;
    let batch_parents: Vec<&Row> = batch.parents[..keep].iter().collect();
    *batch.reads.lock().unwrap() = 0;
    BatchDeleteEnrichmentStrategy
        .delete_linked_batch(batch.counted.clone(), &links_fill(), &batch_parents)
        .await
        .unwrap();
    assert_eq!(*batch.reads.lock().unwrap(), 1);
    for parent in &default.parents[..keep] {
        CascadeDeleteEnrichmentStrategy
            .delete_linked(default.counted.clone(), &links_fill(), parent)
            .await
            .unwrap();
    }
    assert_eq!(*default.reads.lock().unwrap(), keep);
    assert_eq!(stored(&batch.children).await.len(), 1);
    assert_eq!(stored(&default.children).await.len(), 1);
}

#[tokio::test]
async fn update_linked_batch_reads_once_and_matches_the_default_strategy() {
    let batch = fixture("ParentTag", "parent_id").await;
    let default = fixture("ParentTag", "parent_id").await;
    for f in [&batch, &default] {
        for (i, row) in stored(&f.children).await.into_iter().enumerate() {
            let mut linked = row.clone();
            linked.insert("tag_id".into(), Value::Int(i as i64 + 1));
            f.children.update(linked, None).await.unwrap();
        }
    }
    // every parent now wants tag 1000 only: its old link is removed and the new one added
    let wanted = |f: &Fixture| -> Vec<Row> {
        f.parents
            .iter()
            .map(|parent| {
                let mut row = parent.clone();
                row.insert("tags".into(), encode_rows(&[record(&[("id", Value::Int(1000))])]));
                row
            })
            .collect()
    };
    let mut batch_rows = wanted(&batch);
    let mut default_rows = wanted(&default);
    *batch.reads.lock().unwrap() = 0;
    let mut targets: Vec<&mut Row> = batch_rows.iter_mut().collect();
    BatchUpdateEnrichmentStrategy
        .update_linked_batch(batch.counted.clone(), &links_fill(), &mut targets)
        .await
        .unwrap();
    assert_eq!(*batch.reads.lock().unwrap(), 1);
    for row in default_rows.iter_mut() {
        DefaultUpdateEnrichmentStrategy
            .update_linked(default.counted.clone(), &links_fill(), row)
            .await
            .unwrap();
    }
    assert_eq!(*default.reads.lock().unwrap() as i64, PARENTS);
    let tags = |rows: &[Row]| {
        let mut tags: Vec<i64> = rows.iter().map(|row| num(row, "tag_id")).collect();
        tags.sort();
        tags
    };
    assert_eq!(tags(&stored(&batch.children).await), tags(&stored(&default.children).await));
    assert!(tags(&stored(&batch.children).await).iter().all(|tag| *tag == 1000));
}
