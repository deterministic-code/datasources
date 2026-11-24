use contacts_crud_enriched_medium_rest::{
    register, repositories, Address, Contact, LocalePreference, Phone, Repositories,
};
use datasources::{
    default, BatchCommandIBulkWriteStrategy, BulkCRUDRepository, DatasourceError,
    Enrichment, EnrichmentPropertyType, Entity, LoadLevelOrderReadonlyEnrichmentStrategy,
    Operator, ReadonlyRepository, Result, Row, Where,
};
use datasources::chrono::{DateTime, Datelike, Utc};
use futures_util::TryStreamExt;
use std::collections::HashMap;
use std::sync::Arc;

fn uuid(text: &str) -> datasources::uuid::Uuid {
    text.parse().expect("uuid")
}

fn decimal(text: &str) -> datasources::rust_decimal::Decimal {
    text.parse().expect("decimal")
}

fn ada() -> Contact {
    Contact {
        first_name: "Ada".into(),
        last_name: "Lovelace".into(),
        contact_source_name: "Manual".into(),
        uuid: uuid("00000000-0000-0000-0000-000000000001"),
        version: vec![0],
        notes: Some("poet".into()),
        email: Some("ada@example.com".into()),
        addresses: vec![
            Address {
                address_type_name: "Home".into(),
                line1: "12 Analytical Engine".into(),
                line2: Some("Room 12".into()),
                city: "London".into(),
                region: Some("England".into()),
                postal_code: Some("SW1A 1AA".into()),
                country: Some("GB".into()),
                ..Default::default()
            },
            Address {
                address_type_name: "Work".into(),
                line1: "1 Strand".into(),
                city: "London".into(),
                ..Default::default()
            },
        ],
        phones: vec![Phone {
            phone_type_name: "Mobile".into(),
            number: "555-0100".into(),
            ..Default::default()
        }],
        settings: HashMap::from([("theme".into(), "dark".into())]),
        prefs: HashMap::from([(
            "home".into(),
            LocalePreference {
                locale: "en-GB".into(),
                timezone: "Europe/London".into(),
            },
        )]),
        ..Default::default()
    }
}

fn alan() -> Contact {
    Contact {
        first_name: "Alan".into(),
        last_name: "Turing".into(),
        contact_source_name: "Manual".into(),
        uuid: uuid("00000000-0000-0000-0000-000000000002"),
        version: vec![0],
        addresses: vec![Address {
            address_type_name: "Home".into(),
            line1: "1 Bletchley Park".into(),
            city: "Bletchley".into(),
            country: Some("GB".into()),
            ..Default::default()
        }],
        phones: vec![Phone {
            phone_type_name: "Mobile".into(),
            number: "555-0200".into(),
            ..Default::default()
        }],
        settings: HashMap::from([("theme".into(), "light".into())]),
        prefs: HashMap::from([(
            "lab".into(),
            LocalePreference {
                locale: "en-GB".into(),
                timezone: "Europe/London".into(),
            },
        )]),
        ..Default::default()
    }
}

fn bulk_contact_row(index: usize) -> Result<Row> {
    let children = 5 + index % 6;
    let contact = Contact {
        first_name: format!("Bulk{index}"),
        last_name: "Bulk".into(),
        contact_source_name: "Manual".into(),
        addresses: (0..children - 1)
            .map(|n| Address {
                address_type_name: if n % 2 == 0 { "Home" } else { "Work" }.into(),
                line1: format!("{index}-{n} Bulk Lane"),
                city: "London".into(),
                external_id: Some(bulk_external_id(index, n)),
                verified_at: Some(bulk_verified_at()),
                ..Default::default()
            })
            .collect(),
        phones: vec![Phone {
            phone_type_name: "Mobile".into(),
            number: format!("555-1{index:03}"),
            ..Default::default()
        }],
        ..Default::default()
    };
    let mut row = Contact::object_converter().reverse_convert(&contact)?;
    row.set_payload("addresses", contact.payload("addresses"))?;
    row.set_payload("phones", contact.payload("phones"))?;
    Ok(row)
}

fn bulk_external_id(index: usize, n: usize) -> datasources::uuid::Uuid {
    uuid(&format!("00000000-0000-0000-0001-{index:04}{n:08}"))
}

fn bulk_verified_at() -> DateTime<Utc> {
    "2020-01-02T03:04:05.000Z".parse().expect("datetime")
}

fn assigned_date(value: DateTime<Utc>) -> bool {
    value.year() >= 2020
}

fn contact_from_row(row: &Row) -> Result<Contact> {
    let mut contact = Contact::object_converter().convert(row)?;
    contact.hydrate_from_row(row)?;
    Ok(contact)
}

fn require_bulk_contact(row: &Contact, index: usize) -> Result<()> {
    if row.first_name != format!("Bulk{index}") || row.last_name != "Bulk" || row.contact_source_name != "Manual" {
        return Err(DatasourceError::message("bulk contact fields"));
    }
    if row.id <= 0 || row.uuid.is_nil() {
        return Err(DatasourceError::message("bulk contact uuid"));
    }
    if !assigned_date(row.created) || !assigned_date(row.updated) {
        return Err(DatasourceError::message("bulk contact datetime"));
    }
    let children = 5 + index % 6;
    if row.addresses.len() + row.phones.len() != children {
        return Err(DatasourceError::message("bulk child counts"));
    }
    for n in 0..children - 1 {
        let address = row
            .addresses
            .iter()
            .find(|item| item.line1 == format!("{index}-{n} Bulk Lane"))
            .ok_or_else(|| DatasourceError::message("bulk address fields"))?;
        if address.address_type_name != if n % 2 == 0 { "Home" } else { "Work" } || address.city != "London" {
            return Err(DatasourceError::message("bulk address fields"));
        }
        if address.contact_id != row.id {
            return Err(DatasourceError::message("bulk address contact id"));
        }
        if address.id <= 0 || address.uuid.is_nil() || address.external_id != Some(bulk_external_id(index, n)) {
            return Err(DatasourceError::message("bulk address uuid"));
        }
        if !assigned_date(address.created)
            || !assigned_date(address.updated)
            || address.verified_at != Some(bulk_verified_at())
        {
            return Err(DatasourceError::message("bulk address datetime"));
        }
    }
    let phone = row.phones.first().ok_or_else(|| DatasourceError::message("bulk phone fields"))?;
    if phone.number != format!("555-1{index:03}") || phone.phone_type_name != "Mobile" {
        return Err(DatasourceError::message("bulk phone fields"));
    }
    if phone.contact_id != row.id {
        return Err(DatasourceError::message("bulk phone contact id"));
    }
    if phone.id <= 0 || phone.uuid.is_nil() {
        return Err(DatasourceError::message("bulk phone uuid"));
    }
    if !assigned_date(phone.created) || !assigned_date(phone.updated) {
        return Err(DatasourceError::message("bulk phone datetime"));
    }
    Ok(())
}

fn require_ada_children(row: &Contact, engine_type: &str, engine_city: &str, number: &str) -> Result<()> {
    if row.addresses.len() != 2 || row.phones.len() != 1 {
        return Err(DatasourceError::message("child counts"));
    }
    let engine = row
        .addresses
        .iter()
        .find(|address| address.line1 == "12 Analytical Engine")
        .ok_or_else(|| DatasourceError::message("engine address"))?;
    if engine.address_type_name != engine_type || engine.city != engine_city {
        return Err(DatasourceError::message("engine address"));
    }
    if engine.contact_id != row.id {
        return Err(DatasourceError::message("address contact id"));
    }
    if row.phones[0].number != number || row.phones[0].phone_type_name != "Mobile" {
        return Err(DatasourceError::message("phone"));
    }
    if row.phones[0].contact_id != row.id {
        return Err(DatasourceError::message("phone contact id"));
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    register();
    let ds = default()?;
    let contacts_repo = ds.contacts();
    let ada = contacts_repo.create(ada()).await?;
    if ada.first_name != "Ada" || ada.addresses.len() != 2 || ada.phones.len() != 1
        || ada.settings.get("theme") != Some(&"dark".into())
        || ada.prefs.get("home").map(|pref| pref.locale.as_str()) != Some("en-GB")
        || ada.addresses[0].line1 != "12 Analytical Engine"
        || ada.addresses[0].line2.as_deref() != Some("Room 12")
        || ada.addresses[1].address_type_name != "Work"
        || ada.phones[0].number != "555-0100"
        || ada.addresses[0].latitude.is_some()
    {
        return Err(DatasourceError::message("create ada"));
    }
    let alan = contacts_repo.create(alan()).await?;
    if alan.first_name != "Alan" || alan.addresses.len() != 1 || alan.phones.len() != 1 {
        return Err(DatasourceError::message("create alan"));
    }

    let ada_read = contacts_repo
        .read_single_where(&Where::new(Operator::equals("id", ada.id)), None)
        .await?
        .ok_or_else(|| DatasourceError::message("read ada"))?;
    if ada_read.settings.len() != 1 || ada_read.prefs.len() != 1 {
        return Err(DatasourceError::message("read ada enrichments"));
    }
    require_ada_children(&ada_read, "Home", "London", "555-0100")?;
    if ada_read
        .addresses
        .iter()
        .find(|address| address.line1 == "1 Strand")
        .map(|address| address.address_type_name.as_str())
        != Some("Work")
    {
        return Err(DatasourceError::message("read work address"));
    }
    let alan_read = contacts_repo
        .read_single_where(&Where::new(Operator::equals("id", alan.id)), None)
        .await?
        .ok_or_else(|| DatasourceError::message("read alan"))?;
    if alan_read.addresses[0].city != "Bletchley" {
        return Err(DatasourceError::message("read alan address"));
    }

    contacts_repo
        .update(Contact {
            id: ada.id,
            first_name: "Ada Byron".into(),
            last_name: ada.last_name.clone(),
            contact_source_name: "Manual".into(),
            uuid: ada.uuid.clone(),
            version: ada.version.clone(),
            addresses: vec![
                Address {
                    id: ada.addresses[0].id,
                    address_type_name: "Work".into(),
                    line1: ada.addresses[0].line1.clone(),
                    line2: Some(String::new()),
                    city: "Cambridge".into(),
                    region: Some(String::new()),
                    country: Some("FR".into()),
                    ..Default::default()
                },
                Address {
                    address_type_name: "Home".into(),
                    line1: "1 College".into(),
                    city: "Cambridge".into(),
                    country: Some("GB".into()),
                    ..Default::default()
                },
            ],
            phones: vec![Phone {
                id: ada.phones[0].id,
                phone_type_name: "Mobile".into(),
                number: "555-0111".into(),
                ..Default::default()
            }],
            settings: HashMap::from([("theme".into(), "sepia".into())]),
            prefs: ada.prefs.clone(),
            ..Default::default()
        }, None)
        .await?;
    let ada_after = contacts_repo
        .read_single_where(&Where::new(Operator::equals("id", ada.id)), None)
        .await?
        .ok_or_else(|| DatasourceError::message("update"))?;
    if ada_after.first_name != "Ada Byron"
        || ada_after.settings.get("theme") != Some(&"sepia".into())
        || ada_after
            .addresses
            .iter()
            .find(|address| address.line1 == "12 Analytical Engine")
            .and_then(|address| address.country.as_deref())
            != Some("FR")
    {
        return Err(DatasourceError::message("update ada"));
    }
    require_ada_children(&ada_after, "Work", "Cambridge", "555-0111")?;
    if ada_after
        .addresses
        .iter()
        .find(|address| address.line1 == "1 College")
        .map(|address| address.address_type_name.as_str())
        != Some("Home")
    {
        return Err(DatasourceError::message("update home address"));
    }

    let omitted = contacts_repo
        .create(Contact {
            first_name: "Outlier".into(),
            last_name: "Types".into(),
            contact_source_name: "Manual".into(),
            uuid: uuid("00000000-0000-0000-0000-000000000003"),
            version: vec![0],
            addresses: vec![Address {
                address_type_name: "Home".into(),
                line1: "1 Null Lane".into(),
                city: "London".into(),
                ..Default::default()
            }],
            ..Default::default()
        })
        .await?;
    if omitted.addresses[0].latitude.is_some()
        || omitted.addresses[0].is_verified.is_some()
        || omitted.addresses[0].score.is_some()
    {
        return Err(DatasourceError::message("outlier omit"));
    }
    contacts_repo
        .update(Contact {
            id: omitted.id,
            first_name: omitted.first_name.clone(),
            last_name: omitted.last_name.clone(),
            contact_source_name: "Manual".into(),
            uuid: omitted.uuid.clone(),
            version: omitted.version.clone(),
            addresses: vec![Address {
                id: omitted.addresses[0].id,
                address_type_name: "Home".into(),
                line1: "1 Null Lane".into(),
                city: "London".into(),
                latitude: Some(51),
                label_note: Some("side".into()),
                floor: Some(2),
                score: Some(decimal("10.5")),
                external_id: Some(uuid("550e8400-e29b-41d4-a716-446655440000")),
                is_verified: Some(true),
                verified_at: Some(
                    datasources::chrono::DateTime::parse_from_rfc3339("2020-01-02T03:04:05.000Z")
                        .map_err(|_| DatasourceError::message("date"))?
                        .with_timezone(&datasources::chrono::Utc),
                ),
                stamp: Some(vec![1]),
                ..Default::default()
            }],
            ..Default::default()
        }, None)
        .await?;
    let valued = contacts_repo
        .read_single_where(&Where::new(Operator::equals("id", omitted.id)), None)
        .await?
        .ok_or_else(|| DatasourceError::message("outlier value"))?;
    if valued.addresses[0].latitude != Some(51)
        || valued.addresses[0].label_note.as_deref() != Some("side")
        || valued.addresses[0].floor != Some(2)
        || valued.addresses[0].score != Some(decimal("10.5"))
        || valued.addresses[0].is_verified != Some(true)
    {
        return Err(DatasourceError::message("outlier value"));
    }
    contacts_repo
        .update(Contact {
            id: omitted.id,
            first_name: omitted.first_name.clone(),
            last_name: omitted.last_name.clone(),
            contact_source_name: "Manual".into(),
            uuid: omitted.uuid.clone(),
            version: valued.version.clone(),
            addresses: vec![Address {
                id: omitted.addresses[0].id,
                address_type_name: "Home".into(),
                line1: "1 Null Lane".into(),
                city: "London".into(),
                latitude: Some(0),
                label_note: Some(String::new()),
                floor: Some(0),
                score: Some(decimal("0")),
                is_verified: Some(false),
                verified_at: None,
                stamp: None,
                ..Default::default()
            }],
            ..Default::default()
        }, None)
        .await?;
    let zeroed = contacts_repo
        .read_single_where(&Where::new(Operator::equals("id", omitted.id)), None)
        .await?
        .ok_or_else(|| DatasourceError::message("outlier zero"))?;
    if zeroed.addresses[0].latitude != Some(0)
        || zeroed.addresses[0].label_note.as_deref() != Some("")
        || zeroed.addresses[0].floor != Some(0)
        || zeroed.addresses[0].is_verified != Some(false)
    {
        return Err(DatasourceError::message("outlier zero"));
    }

    let connection = ds.create_connection().await?;
    if !connection.is_open() {
        connection.open().await?;
    }
    let address_types = repositories::addressTypesRepository::AddressTypesRepository::new(Arc::clone(&connection))?;
    let phone_types = repositories::phoneTypesRepository::PhoneTypesRepository::new(Arc::clone(&connection))?;
    let contact_sources = repositories::contactSourcesRepository::ContactSourcesRepository::new(Arc::clone(&connection))?;
    let mut addresses_bulk = BulkCRUDRepository::with_strategy(
        Arc::clone(&connection),
        repositories::addressesRepository::AddressesRepository::options(Arc::clone(&connection))?,
        Arc::new(BatchCommandIBulkWriteStrategy::new()),
    )?;
    addresses_bulk.add_enriched(
        Enrichment::new("address_type_name", "address_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object),
        &address_types,
        None,
    );
    addresses_bulk.add_enriched(
        Enrichment::new("description", "address_type_id", "id", Some("description".into()), EnrichmentPropertyType::Object),
        &address_types,
        None,
    );
    let mut phones_bulk = BulkCRUDRepository::with_strategy(
        Arc::clone(&connection),
        repositories::phonesRepository::PhonesRepository::options(Arc::clone(&connection))?,
        Arc::new(BatchCommandIBulkWriteStrategy::new()),
    )?;
    phones_bulk.add_enriched(
        Enrichment::new("phone_type_name", "phone_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object),
        &phone_types,
        None,
    );
    phones_bulk.add_enriched(
        Enrichment::new("description", "phone_type_id", "id", Some("description".into()), EnrichmentPropertyType::Object),
        &phone_types,
        None,
    );
    let settings_bulk = BulkCRUDRepository::with_strategy(
        Arc::clone(&connection),
        repositories::contactSettingsRepository::ContactSettingsRepository::options(Arc::clone(&connection))?,
        Arc::new(BatchCommandIBulkWriteStrategy::new()),
    )?;
    let prefs_bulk = BulkCRUDRepository::with_strategy(
        Arc::clone(&connection),
        repositories::contactPreferencesRepository::ContactPreferencesRepository::options(Arc::clone(&connection))?,
        Arc::new(BatchCommandIBulkWriteStrategy::new()),
    )?;
    let mut contacts_bulk = BulkCRUDRepository::with_strategy(
        Arc::clone(&connection),
        repositories::contactsRepository::ContactsRepository::options(Arc::clone(&connection))?,
        Arc::new(BatchCommandIBulkWriteStrategy::new()),
    )?;
    contacts_bulk.add_enriched(
        Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List),
        Arc::new(addresses_bulk),
        None,
    );
    contacts_bulk.add_enriched(
        Enrichment::new("phones", "id", "contact_id", None, EnrichmentPropertyType::List),
        Arc::new(phones_bulk),
        None,
    );
    contacts_bulk.add_enriched(
        Enrichment::new("settings", "id", "contact_id", Some("value".into()), EnrichmentPropertyType::Dictionary)
            .with_dictionary_key("key"),
        Arc::new(settings_bulk),
        None,
    );
    contacts_bulk.add_enriched(
        Enrichment::new("prefs", "id", "contact_id", None, EnrichmentPropertyType::Dictionary).with_dictionary_key("key"),
        Arc::new(prefs_bulk),
        None,
    );
    contacts_bulk.add_enriched(
        Enrichment::new("contact_source_name", "contact_source_id", "id", Some("name".into()), EnrichmentPropertyType::Object),
        &contact_sources,
        None,
    );
    contacts_bulk.add_enriched(
        Enrichment::new("description", "contact_source_id", "id", Some("description".into()), EnrichmentPropertyType::Object),
        &contact_sources,
        None,
    );
    let mut bulk_items = (0..10).map(bulk_contact_row).collect::<Result<Vec<_>>>()?;
    let created_bulk = contacts_bulk.create_bulk(&mut bulk_items).await?;
    if created_bulk.len() != 10 {
        return Err(DatasourceError::message("bulk create ids"));
    }
    let mut ids = Vec::new();
    for (index, row) in created_bulk.iter().enumerate() {
        let created = contact_from_row(row)?;
        require_bulk_contact(&created, index)?;
        let id = i64::from(created.id);
        if ids.contains(&id) {
            return Err(DatasourceError::message("bulk create ids"));
        }
        ids.push(id);
    }
    let loaded_all: Vec<Contact> = contacts_repo.read(None, None).try_collect().await?;
    for (index, id) in ids.iter().enumerate() {
        let loaded = loaded_all
            .iter()
            .find(|row| i64::from(row.id) == *id)
            .ok_or_else(|| DatasourceError::message("bulk read child counts"))?;
        require_bulk_contact(loaded, index)?;
    }

    let strategy = Arc::new(LoadLevelOrderReadonlyEnrichmentStrategy::new().with_batch_size(3));
    let options = repositories::contactsRepository::ContactsRepository::options(Arc::clone(&connection))?;
    let mut load_level = ReadonlyRepository::<Contact>::new(Arc::clone(&connection), options.read)?;
    let load_level_addresses = repositories::addressesRepository::AddressesRepository::new(Arc::clone(&connection))?;
    let load_level_phones = repositories::phonesRepository::PhonesRepository::new(Arc::clone(&connection))?;
    let load_level_settings =
        repositories::contactSettingsRepository::ContactSettingsRepository::new(Arc::clone(&connection))?;
    let load_level_prefs =
        repositories::contactPreferencesRepository::ContactPreferencesRepository::new(Arc::clone(&connection))?;
    let load_level_sources =
        repositories::contactSourcesRepository::ContactSourcesRepository::new(Arc::clone(&connection))?;
    load_level.add_enriched(
        Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List),
        &load_level_addresses,
        Some(Arc::clone(&strategy) as _),
    );
    load_level.add_enriched(
        Enrichment::new("phones", "id", "contact_id", None, EnrichmentPropertyType::List),
        &load_level_phones,
        Some(Arc::clone(&strategy) as _),
    );
    load_level.add_enriched(
        Enrichment::new(
            "settings",
            "id",
            "contact_id",
            Some("value".into()),
            EnrichmentPropertyType::Dictionary,
        )
        .with_dictionary_key("key"),
        &load_level_settings,
        Some(Arc::clone(&strategy) as _),
    );
    load_level.add_enriched(
        Enrichment::new("prefs", "id", "contact_id", None, EnrichmentPropertyType::Dictionary)
            .with_dictionary_key("key"),
        &load_level_prefs,
        Some(Arc::clone(&strategy) as _),
    );
    load_level.add_enriched(
        Enrichment::new(
            "contact_source_name",
            "contact_source_id",
            "id",
            Some("name".into()),
            EnrichmentPropertyType::Object,
        ),
        &load_level_sources,
        Some(Arc::clone(&strategy) as _),
    );
    load_level.add_enriched(
        Enrichment::new(
            "description",
            "contact_source_id",
            "id",
            Some("description".into()),
            EnrichmentPropertyType::Object,
        ),
        &load_level_sources,
        Some(Arc::clone(&strategy) as _),
    );
    let load_level_ada = load_level
        .read_single_where(&Where::new(Operator::equals("id", ada.id)), None)
        .await?
        .ok_or_else(|| DatasourceError::message("load-level-order ada"))?;
    if load_level_ada.first_name != "Ada Byron"
        || load_level_ada.settings.get("theme") != Some(&"sepia".into())
        || load_level_ada
            .prefs
            .get("home")
            .map(|pref| pref.locale.as_str())
            != Some("en-GB")
    {
        return Err(DatasourceError::message("load-level-order ada"));
    }
    require_ada_children(&load_level_ada, "Work", "Cambridge", "555-0111")
        .map_err(|_| DatasourceError::message("load-level-order ada"))?;
    let load_level_all: Vec<Contact> = load_level.read(None, None).try_collect().await?;
    for (index, id) in ids.iter().enumerate() {
        let loaded = load_level_all
            .iter()
            .find(|row| i64::from(row.id) == *id)
            .ok_or_else(|| DatasourceError::message("load-level-order bulk"))?;
        require_bulk_contact(loaded, index).map_err(|_| DatasourceError::message("load-level-order bulk"))?;
    }

    contacts_repo.delete(ada_after, None).await?;
    if contacts_repo
        .read_single_where(&Where::new(Operator::equals("id", ada.id)), None)
        .await?
        .is_some()
    {
        return Err(DatasourceError::message("delete ada"));
    }
    contacts_repo.delete(alan_read, None).await?;
    if contacts_repo
        .read_single_where(&Where::new(Operator::equals("id", alan.id)), None)
        .await?
        .is_some()
    {
        return Err(DatasourceError::message("delete alan"));
    }
    ds.close().await?;
    Ok(())
}
