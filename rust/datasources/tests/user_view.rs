use futures_util::TryStreamExt;
use datasources::*;
use datasources_memory::MemoryDriver;
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

fn names(rows: &[Row]) -> Vec<String> {
    let mut values: Vec<String> = rows
        .iter()
        .map(|row| match row.get("name") {
            Some(Value::Text(text)) => text.clone(),
            _ => panic!("name"),
        })
        .collect();
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
    let connection = MemoryDriver.connection(None);
    connection.open().await.unwrap();
    let mut users_options = CRUDRepositoryOptions::default();
    users_options.entity = "users".into();
    users_options.write_enrichment = write;
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
        .create(&mut record(&[("name", Value::from("admin"))]), None)
        .await
        .unwrap();
    let admin_role = roles
        .create(&mut record(&[("name", Value::from("admin"))]), None)
        .await
        .unwrap();
    let editor_role = roles
        .create(&mut record(&[("name", Value::from("editor"))]), None)
        .await
        .unwrap();
    let engineering = groups
        .create(&mut record(&[("name", Value::from("engineering"))]), None)
        .await
        .unwrap();
    let ops = groups
        .create(&mut record(&[("name", Value::from("ops"))]), None)
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
            Enrichment::new("user_type", "user_type_id", "id", None, datasources::EnrichmentPropertyType::Object),
            seeded.user_types.clone(),
            None,
        )
        .add_linked(
            Enrichment::new("roles", "id", "id", None, datasources::EnrichmentPropertyType::Object)
                .linked(Association::new("user_id", "role_id")),
            seeded.roles.clone(),
            seeded.user_roles.clone(),
            None,
        )
        .add_linked(
            Enrichment::new("groups", "id", "id", None, datasources::EnrichmentPropertyType::Object)
                .linked(Association::new("user_id", "group_id")),
            seeded.groups.clone(),
            seeded.user_groups.clone(),
            None,
        );
}

async fn create_ada(users: &mut CRUDRepository, seeded: &Seed, roles: Value, groups: Value) -> Row {
    link(users, seeded);
    users.create(&mut ada(seeded, roles, groups), None).await.unwrap()
}

#[tokio::test]
async fn create_update_and_delete_syncs_link_rows() {
    let (mut users, seeded) = open(None).await;
    users
        .add_enriched(
            Enrichment::new("user_type", "user_type_id", "id", None, datasources::EnrichmentPropertyType::Object),
            seeded.user_types.clone(),
            None,
        )
        .add_linked(
            Enrichment::new("roles", "id", "id", None, datasources::EnrichmentPropertyType::Object)
                .linked(Association::new("user_id", "role_id")),
            seeded.roles.clone(),
            seeded.user_roles.clone(),
            None,
        )
        .add_linked(
            Enrichment::new("groups", "id", "id", None, datasources::EnrichmentPropertyType::Object)
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
        ), None)
        .await
        .unwrap();
    let created_role_links = seeded.user_roles.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
    let created_group_links = seeded.user_groups.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
    let mut next = ada(
        &seeded,
        encode_rows(&[seeded.editor_role.clone()]),
        encode_rows(&[seeded.engineering.clone(), seeded.ops.clone()]),
    );
    next.insert("id".into(), created.get("id").unwrap().clone());
    let saved = users.update(&mut next, None, None).await.unwrap();
    let updated = users
        .read_single_by_id(created.get("id").unwrap())
        .await
        .unwrap()
        .unwrap();
    let updated_role_links = seeded.user_roles.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
    let updated_group_links = seeded.user_groups.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
    let removed = users
        .delete(&record(&[("id", created.get("id").unwrap().clone())]), None, None)
        .await
        .unwrap();

    assert_eq!(num(&created, "id") > 0, true);
    assert_eq!(created.get("username"), Some(&Value::from("ada")));
    assert_eq!(created.get("is_enabled"), Some(&Value::Bool(true)));
    assert_eq!(num(&created, "user_type_id"), num(&seeded.admin_type, "id"));
    assert_eq!(
        contains_id(&created, "user_type", num(&seeded.admin_type, "id")),
        true
    );
    assert_eq!(
        contains_id(&created, "roles", num(&seeded.admin_role, "id")),
        true
    );
    assert_eq!(
        contains_id(&created, "roles", num(&seeded.editor_role, "id")),
        true
    );
    assert_eq!(
        contains_id(&created, "groups", num(&seeded.engineering, "id")),
        true
    );
    assert_eq!(
        ints(&created_role_links, "user_id"),
        vec![num(&created, "id"), num(&created, "id")]
    );
    assert_eq!(
        ints(&created_role_links, "role_id"),
        ints(
            &[seeded.admin_role.clone(), seeded.editor_role.clone()],
            "id"
        )
    );
    assert_eq!(
        ints(&created_group_links, "user_id"),
        vec![num(&created, "id")]
    );
    assert_eq!(
        ints(&created_group_links, "group_id"),
        vec![num(&seeded.engineering, "id")]
    );
    assert_eq!(saved.rows_affected, 1);
    assert_eq!(num(&updated, "id"), num(&created, "id"));
    assert_eq!(updated.get("username"), Some(&Value::from("ada")));
    assert_eq!(updated.get("is_enabled"), Some(&Value::Bool(true)));
    assert_eq!(num(&updated, "user_type_id"), num(&seeded.admin_type, "id"));
    assert_eq!(
        contains_id(&updated, "roles", num(&seeded.editor_role, "id")),
        true
    );
    assert_eq!(
        contains_id(&updated, "roles", num(&seeded.admin_role, "id")),
        false
    );
    assert_eq!(
        contains_id(&updated, "groups", num(&seeded.engineering, "id")),
        true
    );
    assert_eq!(
        contains_id(&updated, "groups", num(&seeded.ops, "id")),
        true
    );
    assert_eq!(
        ints(&updated_role_links, "user_id"),
        vec![num(&created, "id")]
    );
    assert_eq!(
        ints(&updated_role_links, "role_id"),
        vec![num(&seeded.editor_role, "id")]
    );
    assert_eq!(
        ints(&updated_group_links, "user_id"),
        vec![num(&created, "id"), num(&created, "id")]
    );
    assert_eq!(
        ints(&updated_group_links, "group_id"),
        ints(&[seeded.engineering.clone(), seeded.ops.clone()], "id")
    );
    assert_eq!(removed.rows_affected, 1);
    assert_eq!(users.read(None, None, None).try_collect::<Vec<_>>().await.unwrap().is_empty(), true);
    assert_eq!(
        seeded.user_roles.read(None, None, None).try_collect::<Vec<_>>().await.unwrap().is_empty(),
        true
    );
    assert_eq!(
        seeded
            .user_groups
            .read(None, None, None)
            .try_collect::<Vec<_>>().await
            .unwrap()
            .is_empty(),
        true
    );
    assert_eq!(
        names(&seeded.roles.read(None, None, None).try_collect::<Vec<_>>().await.unwrap()),
        vec!["admin".to_string(), "editor".to_string()]
    );
    assert_eq!(
        names(&seeded.groups.read(None, None, None).try_collect::<Vec<_>>().await.unwrap()),
        vec!["engineering".to_string(), "ops".to_string()]
    );
    assert_eq!(
        seeded.user_types.read(None, None, None).try_collect::<Vec<_>>().await.unwrap()[0]["name"],
        Value::from("admin")
    );
}

#[tokio::test]
async fn null_list_clears_that_association() {
    let (mut users, seeded) = open(None).await;
    let roles = encode_rows(&[seeded.admin_role.clone(), seeded.editor_role.clone()]);
    let groups = encode_rows(&[seeded.engineering.clone()]);
    let created = create_ada(&mut users, &seeded, roles, groups).await;
    let mut next = record(&[
        ("id", created.get("id").unwrap().clone()),
        ("username", Value::from("ada")),
        ("is_enabled", Value::Bool(true)),
        ("user_type_id", seeded.admin_type.get("id").unwrap().clone()),
        ("groups", encode_rows(&[seeded.engineering.clone()])),
    ]);
    let saved = users.update(&mut next, None, None).await.unwrap();
    let loaded = users
        .read_single_by_id(created.get("id").unwrap())
        .await
        .unwrap()
        .unwrap();
    let user_roles = seeded.user_roles.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
    let user_groups = seeded.user_groups.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
    assert_eq!(num(&created, "id") > 0, true);
    assert_eq!(saved.rows_affected, 1);
    assert_eq!(num(&loaded, "id"), num(&created, "id"));
    assert_eq!(loaded.get("username"), Some(&Value::from("ada")));
    assert_eq!(num(&loaded, "user_type_id"), num(&seeded.admin_type, "id"));
    assert_eq!(
        contains_id(&loaded, "user_type", num(&seeded.admin_type, "id")),
        true
    );
    assert_eq!(
        contains_id(&loaded, "groups", num(&seeded.engineering, "id")),
        true
    );
    assert_eq!(loaded.get("roles"), Some(&Value::Text(String::new())));
    assert!(user_roles.is_empty());
    assert_eq!(ints(&user_groups, "user_id"), vec![num(&created, "id")]);
    assert_eq!(
        ints(&user_groups, "group_id"),
        vec![num(&seeded.engineering, "id")]
    );
    assert_eq!(
        names(&seeded.roles.read(None, None, None).try_collect::<Vec<_>>().await.unwrap()),
        vec!["admin".to_string(), "editor".to_string()]
    );
    assert_eq!(
        names(&seeded.groups.read(None, None, None).try_collect::<Vec<_>>().await.unwrap()),
        vec!["engineering".to_string(), "ops".to_string()]
    );
}

#[tokio::test]
async fn empty_list_clears_that_association() {
    let (mut users, seeded) = open(None).await;
    let roles = encode_rows(&[seeded.admin_role.clone(), seeded.editor_role.clone()]);
    let groups = encode_rows(&[seeded.engineering.clone()]);
    let created = create_ada(&mut users, &seeded, roles, groups).await;
    let mut next = record(&[
        ("id", created.get("id").unwrap().clone()),
        ("username", Value::from("ada")),
        ("is_enabled", Value::Bool(true)),
        ("user_type_id", seeded.admin_type.get("id").unwrap().clone()),
        ("roles", encode_rows(&[])),
        ("groups", encode_rows(&[seeded.engineering.clone()])),
    ]);
    let saved = users.update(&mut next, None, None).await.unwrap();
    let loaded = users
        .read_single_by_id(created.get("id").unwrap())
        .await
        .unwrap()
        .unwrap();
    let user_roles = seeded.user_roles.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
    let user_groups = seeded.user_groups.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
    assert_eq!(num(&created, "id") > 0, true);
    assert_eq!(saved.rows_affected, 1);
    assert_eq!(num(&loaded, "id"), num(&created, "id"));
    assert_eq!(loaded.get("username"), Some(&Value::from("ada")));
    assert_eq!(num(&loaded, "user_type_id"), num(&seeded.admin_type, "id"));
    assert_eq!(
        contains_id(&loaded, "user_type", num(&seeded.admin_type, "id")),
        true
    );
    assert_eq!(loaded.get("roles"), Some(&Value::Text(String::new())));
    assert_eq!(
        contains_id(&loaded, "groups", num(&seeded.engineering, "id")),
        true
    );
    assert!(user_roles.is_empty());
    assert_eq!(ints(&user_groups, "user_id"), vec![num(&created, "id")]);
    assert_eq!(
        ints(&user_groups, "group_id"),
        vec![num(&seeded.engineering, "id")]
    );
    assert_eq!(
        names(&seeded.roles.read(None, None, None).try_collect::<Vec<_>>().await.unwrap()),
        vec!["admin".to_string(), "editor".to_string()]
    );
    assert_eq!(
        names(&seeded.groups.read(None, None, None).try_collect::<Vec<_>>().await.unwrap()),
        vec!["engineering".to_string(), "ops".to_string()]
    );
}

#[tokio::test]
async fn duplicate_related_ids_insert_one_row() {
    let (mut users, seeded) = open(None).await;
    let roles = encode_rows(&[seeded.admin_role.clone(), seeded.admin_role.clone()]);
    let groups = encode_rows(&[seeded.engineering.clone()]);
    let created = create_ada(&mut users, &seeded, roles, groups).await;
    let role_links = seeded.user_roles.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
    let group_links = seeded.user_groups.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
    assert_eq!(num(&created, "id") > 0, true);
    assert_eq!(ints(&role_links, "user_id"), vec![num(&created, "id")]);
    assert_eq!(
        ints(&role_links, "role_id"),
        vec![num(&seeded.admin_role, "id")]
    );
    assert_eq!(ints(&group_links, "user_id"), vec![num(&created, "id")]);
    assert_eq!(
        ints(&group_links, "group_id"),
        vec![num(&seeded.engineering, "id")]
    );
    assert_eq!(
        names(&seeded.roles.read(None, None, None).try_collect::<Vec<_>>().await.unwrap()),
        vec!["admin".to_string(), "editor".to_string()]
    );
}

#[tokio::test]
async fn missing_related_id_throws() {
    let (mut users, seeded) = open(None).await;
    link(&mut users, &seeded);
    let error = users
        .create(&mut ada(
            &seeded,
            encode_rows(&[record(&[
                ("id", Value::Int(0)),
                ("name", Value::from("ghost")),
            ])]),
            encode_rows(&[]),
        ), None)
        .await
        .unwrap_err();
    let stored = users.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
    assert!(error.to_string().contains("requires \"id\""));
    assert_eq!(stored.len(), 1);
    assert_eq!(num(&stored[0], "id") > 0, true);
    assert_eq!(stored[0].get("username"), Some(&Value::from("ada")));
    assert!(seeded.user_roles.read(None, None, None).try_collect::<Vec<_>>().await.unwrap().is_empty());
}

#[tokio::test]
async fn lazy_write_skips_unloaded_links() {
    let (mut users, seeded) = open(Some(Arc::new(LazyWriteEnrichmentStrategy::new(
        None, None, None, false,
    ))))
    .await;
    link(&mut users, &seeded);
    let mut creating = record(&[
        ("username", Value::from("ada")),
        ("is_enabled", Value::Bool(true)),
        ("user_type_id", seeded.admin_type.get("id").unwrap().clone()),
        ("groups", encode_rows(&[seeded.engineering.clone()])),
    ]);
    let created = users.create(&mut creating, None).await.unwrap();
    let created_role_links = seeded.user_roles.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
    let created_group_links = seeded.user_groups.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
    let mut next = record(&[
        ("id", created.get("id").unwrap().clone()),
        ("username", Value::from("ada")),
        ("is_enabled", Value::Bool(false)),
        ("user_type_id", seeded.admin_type.get("id").unwrap().clone()),
    ]);
    let skipped = users.update(&mut next, None, None).await.unwrap();
    let after_skip = users
        .read_single_by_id(created.get("id").unwrap())
        .await
        .unwrap()
        .unwrap();
    let skipped_role_links = seeded.user_roles.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
    let skipped_group_links = seeded.user_groups.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
    let mut linked_row = record(&[
        ("id", created.get("id").unwrap().clone()),
        ("username", Value::from("ada")),
        ("is_enabled", Value::Bool(false)),
        ("user_type_id", seeded.admin_type.get("id").unwrap().clone()),
        ("roles", encode_rows(&[seeded.editor_role.clone()])),
    ]);
    let linked = users.update(&mut linked_row, None, None).await.unwrap();
    let linked_role_links = seeded.user_roles.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
    let linked_group_links = seeded.user_groups.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
    let removed = users
        .delete(&record(&[("id", created.get("id").unwrap().clone())]), None, None)
        .await
        .unwrap();
    assert_eq!(num(&created, "id") > 0, true);
    assert!(created_role_links.is_empty());
    assert_eq!(
        ints(&created_group_links, "user_id"),
        vec![num(&created, "id")]
    );
    assert_eq!(
        ints(&created_group_links, "group_id"),
        vec![num(&seeded.engineering, "id")]
    );
    assert_eq!(skipped.rows_affected, 1);
    assert_eq!(num(&after_skip, "id"), num(&created, "id"));
    assert_eq!(after_skip.get("username"), Some(&Value::from("ada")));
    assert_eq!(after_skip.get("is_enabled"), Some(&Value::Bool(false)));
    assert!(skipped_role_links.is_empty());
    assert_eq!(
        ints(&skipped_group_links, "user_id"),
        vec![num(&created, "id")]
    );
    assert_eq!(
        ints(&skipped_group_links, "group_id"),
        vec![num(&seeded.engineering, "id")]
    );
    assert_eq!(linked.rows_affected, 1);
    assert_eq!(
        ints(&linked_role_links, "user_id"),
        vec![num(&created, "id")]
    );
    assert_eq!(
        ints(&linked_role_links, "role_id"),
        vec![num(&seeded.editor_role, "id")]
    );
    assert_eq!(
        ints(&linked_group_links, "user_id"),
        vec![num(&created, "id")]
    );
    assert_eq!(
        ints(&linked_group_links, "group_id"),
        vec![num(&seeded.engineering, "id")]
    );
    assert_eq!(
        names(&seeded.roles.read(None, None, None).try_collect::<Vec<_>>().await.unwrap()),
        vec!["admin".to_string(), "editor".to_string()]
    );
    assert_eq!(
        names(&seeded.groups.read(None, None, None).try_collect::<Vec<_>>().await.unwrap()),
        vec!["engineering".to_string(), "ops".to_string()]
    );
    assert_eq!(removed.rows_affected, 1);
    assert!(users.read(None, None, None).try_collect::<Vec<_>>().await.unwrap().is_empty());
    assert!(seeded.user_roles.read(None, None, None).try_collect::<Vec<_>>().await.unwrap().is_empty());
    assert!(seeded
        .user_groups
        .read(None, None, None)
        .try_collect::<Vec<_>>().await
        .unwrap()
        .is_empty());
}
