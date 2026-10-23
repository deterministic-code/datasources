pub const SEED_CONTACT_COUNT: usize = 250;

pub fn seed_contacts_sql(count: usize, version: &str) -> Vec<String> {
    let stamp = "2024-01-01T00:00:00Z";
    let uuid = |n: usize| format!("00000000-0000-4000-8000-{:012}", n);
    let mut statements = Vec::new();
    let mut address_id = 1usize;
    let mut phone_id = 1usize;
    for id in 1..=count {
        let source = (id % 3) + 1;
        let type_id = (id % 2) + 1;
        statements.push(format!(
            r#"INSERT INTO "contacts_base" ("id", "uuid", "created", "updated", "version", "contact_source_id", "contact_type_id", "first_name", "last_name", "email", "notes") VALUES ({id}, '{}', '{stamp}', '{stamp}', {version}, {source}, {type_id}, 'First{id}', 'Last{id}', 'c{id}@ex.com', NULL)"#,
            uuid(id)
        ));
        for a in 0..(id % 4) {
            let address_type = if a == 0 { 1 } else { 2 };
            statements.push(format!(
                r#"INSERT INTO "address" ("id", "uuid", "created", "updated", "version", "contact_id", "address_type_id", "line1", "line2", "city", "region", "postal_code", "country") VALUES ({address_id}, '{}', '{stamp}', '{stamp}', {version}, {id}, {address_type}, '{id} Line {}', NULL, 'City{id}', NULL, NULL, NULL)"#,
                uuid(1000 + address_id),
                a + 1
            ));
            address_id += 1;
        }
        for p in 0..(id % 3) {
            let phone_type = if p == 0 { 1 } else { 2 };
            statements.push(format!(
                r#"INSERT INTO "phone" ("id", "uuid", "created", "updated", "version", "contact_id", "phone_type_id", "number") VALUES ({phone_id}, '{}', '{stamp}', '{stamp}', {version}, {id}, {phone_type}, '555-000-{:04}-{p}')"#,
                uuid(2000 + phone_id),
                id
            ));
            phone_id += 1;
        }
    }
    statements
}

pub fn seed_identity_sql(engine: &str) -> Vec<String> {
    let tables = ["contacts_base", "address", "phone"];
    match engine {
        "postgres" => tables
            .iter()
            .map(|table| {
                format!(
                    "SELECT setval(pg_get_serial_sequence('{table}', 'id'), COALESCE((SELECT MAX(id) FROM \"{table}\"), 1))"
                )
            })
            .collect(),
        "mysql" => tables
            .iter()
            .map(|table| format!("ALTER TABLE {table} AUTO_INCREMENT = {}", SEED_CONTACT_COUNT + 1))
            .collect(),
        _ => vec![],
    }
}
