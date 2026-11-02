use datasources::*;
use datasources_memory::MemoryDriver;
use futures_util::TryStreamExt;
use std::sync::Arc;

#[derive(Clone, Debug, Default, PartialEq, Facet)]
#[facet(crate = ::datasources::facet)]
struct Parent {
    id: i64,
    label: String,
    children: Vec<Child>,
}

#[derive(Clone, Debug, Default, PartialEq, Facet)]
#[facet(crate = ::datasources::facet)]
struct Child {
    id: i64,
    parent_id: i64,
    label: String,
}

impl SchemaObject for Parent {}
impl SchemaObject for Child {}

fn parent_options() -> CRUDRepositoryOptions {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "Parent".into();
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "label".into()];
    options.create_properties = Some(vec!["label".into()]);
    options.schema = Schema::new(std::collections::HashMap::from([
        ("id".into(), SchemaColumn {
            type_name: "integer".into(),
            nullable: false,
            has_default: true,
        }),
        ("label".into(), SchemaColumn {
            type_name: "string".into(),
            nullable: false,
            has_default: false,
        }),
    ]));
    options
}

fn child_repo(connection: Arc<dyn Connection>) -> Arc<CRUDRepository> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "Child".into();
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "parent_id".into(), "label".into()];
    options.create_properties = Some(vec!["parent_id".into(), "label".into()]);
    Arc::new(CRUDRepository::<Row>::new(connection, options).unwrap())
}

#[test]
fn typed_parent_payload_reads_children() {
    let parent = Parent {
        label: "p".into(),
        children: vec![Child {
            label: "a".into(),
            ..Default::default()
        }],
        ..Default::default()
    };
    let rows = parent.payload("children");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get("label"), Some(&Value::from("a")));
    let mut empty = Parent::default();
    empty.set_payload("children", rows).unwrap();
    assert_eq!(empty.children[0].label, "a");
}

#[tokio::test]
async fn typed_create_persists_and_reads_list_children() {
    let connection = MemoryDriver.connection(None);
    connection.open().await.unwrap();
    let mut parents = CRUDRepository::<Parent>::new(Arc::clone(&connection), parent_options()).unwrap();
    let children = child_repo(connection);
    parents.add_enriched(
        Enrichment::new(
            "children",
            "id",
            "parent_id",
            None,
            EnrichmentPropertyType::List,
        ),
        children.clone(),
        None,
    );
    let created = parents
        .create(Parent {
            label: "p".into(),
            children: vec![Child {
                label: "a".into(),
                ..Default::default()
            }],
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(created.id > 0);
    assert_eq!(created.children.len(), 1);
    assert_eq!(created.children[0].label, "a");
    assert_eq!(created.children[0].parent_id, created.id);
    let read = parents.read(&()).try_collect::<Vec<_>>().await.unwrap();
    assert_eq!(read.len(), 1);
    assert_eq!(read[0].children[0].label, "a");
}
