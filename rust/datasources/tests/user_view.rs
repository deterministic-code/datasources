use datasources::*;
use datasources_memory::MemoryProvider;
use std::sync::Arc;

struct Seed {
    user_types: Arc<CRUDRepository>,
    roles: Arc<CRUDRepository>,
    groups: Arc<CRUDRepository>,
    user_roles: Arc<CRUDRepository>,
    user_groups: Arc<CRUDRepository>,
    admin_type: Row,
    admin_role: Row,
    editor_role: Row,
    engineering: Row,
    ops: Row,
}

fn record(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), value.clone()))
        .collect()
}

fn num(row: &Row, key: &str) -> i64 {
    match row.get(key) {
        Some(Value::Int(id)) => *id,
        _ => panic!("{key}"),
    }
}

fn ints(rows: &[Row], key: &str) -> Vec<i64> {
    let mut values: Vec<i64> = rows.iter().map(|row| num(row, key)).collect();
    values.sort();
    values
}

fn contains_id(row: &Row, into: &str, id: i64) -> bool {
    match row.get(into) {
        Some(Value::Text(text)) => text
            .split('|')
            .any(|part| part.split(',').any(|pair| pair == format!("id={id}"))),
        _ => false,
    }
}

async fn open(write: Option<Arc<dyn WriteEnrichmentStrategy>>) -> (CRUDRepository, Seed) {
    let connection = MemoryProvider.connection(None);
    connection.open().await.unwrap();
    let mut users_options = CRUDRepositoryOptions::default();
    users_options.entity = "users".into();
    users_options.write = write;
    let users = CRUDRepository::new(connection.clone(), users_options).unwrap();
    let repo = |entity: &str| {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = entity.into();
        Arc::new(CRUDRepository::new(connection.clone(), options).unwrap())
    };
    let user_types = repo("user_types");
    let roles = repo("roles");
    let groups = repo("groups");
    let user_roles = repo("user_roles");
    let user_groups = repo("user_groups");
    let admin_type = user_types
        .create(&mut record(&[("name", Value::from("admin"))]))
        .await
        .unwrap();
    let admin_role = roles
        .create(&mut record(&[("name", Value::from("admin"))]))
        .await
        .unwrap();
    let editor_role = roles
        .create(&mut record(&[("name", Value::from("editor"))]))
        .await
        .unwrap();
    let engineering = groups
        .create(&mut record(&[("name", Value::from("engineering"))]))
        .await
        .unwrap();
    let ops = groups
        .create(&mut record(&[("name", Value::from("ops"))]))
        .await
        .unwrap();
    (
        users,
        Seed {
            user_types,
            roles,
            groups,
            user_roles,
            user_groups,
            admin_type,
            admin_role,
            editor_role,
            engineering,
            ops,
        },
    )
}

fn ada(seeded: &Seed, roles: Value, groups: Value) -> Row {
    record(&[
        ("username", Value::from("ada")),
        ("is_enabled", Value::Bool(true)),
        ("user_type_id", seeded.admin_type.get("id").unwrap().clone()),
        ("roles", roles),
        ("groups", groups),
    ])
}

fn link(users: &mut CRUDRepository, seeded: &Seed) {
    users
        .add_enriched(
            Enrichment::new("user_type", "user_type_id", "id", None, false),
            seeded.user_types.clone(),
            None,
        )
        .add_linked(
            Enrichment::new("roles", "id", "id", None, false)
                .linked(Association::new("user_id", "role_id")),
            seeded.roles.clone(),
            seeded.user_roles.clone(),
            None,
        )
        .add_linked(
            Enrichment::new("groups", "id", "id", None, false)
                .linked(Association::new("user_id", "group_id")),
            seeded.groups.clone(),
            seeded.user_groups.clone(),
            None,
        );
}

async fn create_ada(users: &mut CRUDRepository, seeded: &Seed, roles: Value, groups: Value) -> Row {
    link(users, seeded);
    users.create(&mut ada(seeded, roles, groups)).await.unwrap()
}

#[tokio::test]
async fn create_update_and_delete_syncs_link_rows() {
    let (mut users, seeded) = open(None).await;
    users
        .add_enriched(
            Enrichment::new("user_type", "user_type_id", "id", None, false),
            seeded.user_types.clone(),
            None,
        )
        .add_linked(
            Enrichment::new("roles", "id", "id", None, false)
                .linked(Association::new("user_id", "role_id")),
            seeded.roles.clone(),
            seeded.user_roles.clone(),
            None,
        )
        .add_linked(
            Enrichment::new("groups", "id", "id", None, false)
                .linked(Association::new("user_id", "group_id")),
            seeded.groups.clone(),
            seeded.user_groups.clone(),
            None,
        );
    let created = users
        .create(&mut ada(
            &seeded,
            encode_rows(&[seeded.admin_role.clone(), seeded.editor_role.clone()]),
            encode_rows(&[seeded.engineering.clone()]),
        ))
        .await
        .unwrap();
    let mut next = ada(
        &seeded,
        encode_rows(&[seeded.editor_role.clone()]),
        encode_rows(&[seeded.engineering.clone(), seeded.ops.clone()]),
    );
    next.insert("id".into(), created.get("id").unwrap().clone());
    users.update(&next, None).await.unwrap();
    let updated = users
        .read_single_by_id(created.get("id").unwrap())
        .await
        .unwrap()
        .unwrap();
    users
        .delete(&record(&[("id", created.get("id").unwrap().clone())]), None)
        .await
        .unwrap();

    assert_eq!(num(&created, "id") > 0, true);
    assert_eq!(contains_id(&created, "user_type", num(&seeded.admin_type, "id")), true);
    assert_eq!(contains_id(&created, "roles", num(&seeded.admin_role, "id")), true);
    assert_eq!(contains_id(&created, "roles", num(&seeded.editor_role, "id")), true);
    assert_eq!(contains_id(&created, "groups", num(&seeded.engineering, "id")), true);
    assert_eq!(contains_id(&updated, "roles", num(&seeded.editor_role, "id")), true);
    assert_eq!(contains_id(&updated, "roles", num(&seeded.admin_role, "id")), false);
    assert_eq!(contains_id(&updated, "groups", num(&seeded.engineering, "id")), true);
    assert_eq!(contains_id(&updated, "groups", num(&seeded.ops, "id")), true);
    assert_eq!(users.read(None, None).await.unwrap().is_empty(), true);
    assert_eq!(seeded.user_roles.read(None, None).await.unwrap().is_empty(), true);
    assert_eq!(seeded.user_groups.read(None, None).await.unwrap().is_empty(), true);
    assert_eq!(seeded.roles.read(None, None).await.unwrap().len(), 2);
    assert_eq!(seeded.groups.read(None, None).await.unwrap().len(), 2);
    assert_eq!(
        seeded.user_types.read(None, None).await.unwrap()[0]["name"],
        Value::from("admin")
    );
}

#[tokio::test]
async fn null_list_clears_that_association() {
    let (mut users, seeded) = open(None).await;
    let roles = encode_rows(&[seeded.admin_role.clone(), seeded.editor_role.clone()]);
    let groups = encode_rows(&[seeded.engineering.clone()]);
    let created = create_ada(&mut users, &seeded, roles, groups).await;
    let next = record(&[
        ("id", created.get("id").unwrap().clone()),
        ("username", Value::from("ada")),
        ("is_enabled", Value::Bool(true)),
        ("user_type_id", seeded.admin_type.get("id").unwrap().clone()),
        ("groups", encode_rows(&[seeded.engineering.clone()])),
    ]);
    users.update(&next, None).await.unwrap();
    assert!(seeded.user_roles.read(None, None).await.unwrap().is_empty());
    assert_eq!(
        ints(&seeded.user_groups.read(None, None).await.unwrap(), "group_id"),
        vec![num(&seeded.engineering, "id")]
    );
}

#[tokio::test]
async fn empty_list_clears_that_association() {
    let (mut users, seeded) = open(None).await;
    let roles = encode_rows(&[seeded.admin_role.clone(), seeded.editor_role.clone()]);
    let groups = encode_rows(&[seeded.engineering.clone()]);
    let created = create_ada(&mut users, &seeded, roles, groups).await;
    let next = record(&[
        ("id", created.get("id").unwrap().clone()),
        ("username", Value::from("ada")),
        ("is_enabled", Value::Bool(true)),
        ("user_type_id", seeded.admin_type.get("id").unwrap().clone()),
        ("roles", encode_rows(&[])),
        ("groups", encode_rows(&[seeded.engineering.clone()])),
    ]);
    users.update(&next, None).await.unwrap();
    assert!(seeded.user_roles.read(None, None).await.unwrap().is_empty());
    assert_eq!(
        ints(&seeded.user_groups.read(None, None).await.unwrap(), "group_id"),
        vec![num(&seeded.engineering, "id")]
    );
}

#[tokio::test]
async fn duplicate_related_ids_insert_one_row() {
    let (mut users, seeded) = open(None).await;
    let roles = encode_rows(&[seeded.admin_role.clone(), seeded.admin_role.clone()]);
    let groups = encode_rows(&[seeded.engineering.clone()]);
    let created = create_ada(&mut users, &seeded, roles, groups).await;
    let links = seeded.user_roles.read(None, None).await.unwrap();
    assert_eq!(ints(&links, "user_id"), vec![num(&created, "id")]);
    assert_eq!(ints(&links, "role_id"), vec![num(&seeded.admin_role, "id")]);
}

#[tokio::test]
async fn missing_related_id_throws() {
    let (mut users, seeded) = open(None).await;
    link(&mut users, &seeded);
    let error = users
        .create(&mut ada(
            &seeded,
            encode_rows(&[record(&[("id", Value::Int(0)), ("name", Value::from("ghost"))])]),
            encode_rows(&[]),
        ))
        .await
        .unwrap_err();
    assert!(error.to_string().contains("requires \"id\""));
}

#[tokio::test]
async fn lazy_write_skips_unloaded_links() {
    let (mut users, seeded) = open(Some(Arc::new(LazyWriteEnrichmentStrategy::new(None)))).await;
    link(&mut users, &seeded);
    let mut creating = record(&[
        ("username", Value::from("ada")),
        ("is_enabled", Value::Bool(true)),
        ("user_type_id", seeded.admin_type.get("id").unwrap().clone()),
        ("groups", encode_rows(&[seeded.engineering.clone()])),
    ]);
    let created = users.create(&mut creating).await.unwrap();
    let next = record(&[
        ("id", created.get("id").unwrap().clone()),
        ("username", Value::from("ada")),
        ("is_enabled", Value::Bool(false)),
        ("user_type_id", seeded.admin_type.get("id").unwrap().clone()),
    ]);
    users.update(&next, None).await.unwrap();
    assert!(seeded.user_roles.read(None, None).await.unwrap().is_empty());
    assert_eq!(
        ints(&seeded.user_groups.read(None, None).await.unwrap(), "group_id"),
        vec![num(&seeded.engineering, "id")]
    );
    users
        .delete(&record(&[("id", created.get("id").unwrap().clone())]), None)
        .await
        .unwrap();
    assert!(seeded.user_roles.read(None, None).await.unwrap().is_empty());
    assert!(seeded.user_groups.read(None, None).await.unwrap().is_empty());
    assert_eq!(seeded.roles.read(None, None).await.unwrap().len(), 2);
}
