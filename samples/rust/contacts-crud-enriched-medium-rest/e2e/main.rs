mod datasources_providers;
mod generated;
use generated::{Address, Contact, LocalePreferences, Phone, Repositories};
use datasources::{default, DatasourceError, Operator, Result, Where};
use std::collections::HashMap;

fn ada() -> Contact {
    Contact {
        first_name: "Ada".into(),
        last_name: "Lovelace".into(),
        contact_source_name: "Manual".into(),
        uuid: "00000000-0000-0000-0000-000000000001".into(),
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
            LocalePreferences {
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
        uuid: "00000000-0000-0000-0000-000000000002".into(),
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
            LocalePreferences {
                locale: "en-GB".into(),
                timezone: "Europe/London".into(),
            },
        )]),
        ..Default::default()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    datasources_providers::register();
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
        .read_single(&Where::new(Operator::equals("id", ada.id)))
        .await?
        .ok_or_else(|| DatasourceError::message("read ada"))?;
    if ada_read.addresses.len() != 2 || ada_read.phones.len() != 1 || ada_read.settings.len() != 1
        || ada_read.prefs.len() != 1
    {
        return Err(DatasourceError::message("read ada enrichments"));
    }
    let alan_read = contacts_repo
        .read_single(&Where::new(Operator::equals("id", alan.id)))
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
        })
        .await?;
    let ada_after = contacts_repo
        .read_single(&Where::new(Operator::equals("id", ada.id)))
        .await?
        .ok_or_else(|| DatasourceError::message("update"))?;
    if ada_after.first_name != "Ada Byron"
        || ada_after.addresses.len() != 2
        || ada_after.phones[0].number != "555-0111"
        || ada_after.settings.get("theme") != Some(&"sepia".into())
        || !ada_after.addresses.iter().any(|address| address.city == "Cambridge")
    {
        return Err(DatasourceError::message("update ada"));
    }

    let omitted = contacts_repo
        .create(Contact {
            first_name: "Outlier".into(),
            last_name: "Types".into(),
            contact_source_name: "Manual".into(),
            uuid: "00000000-0000-0000-0000-000000000003".into(),
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
                score: Some("10.5".into()),
                external_id: Some("550e8400-e29b-41d4-a716-446655440000".into()),
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
        })
        .await?;
    let valued = contacts_repo
        .read_single(&Where::new(Operator::equals("id", omitted.id)))
        .await?
        .ok_or_else(|| DatasourceError::message("outlier value"))?;
    if valued.addresses[0].latitude != Some(51)
        || valued.addresses[0].label_note.as_deref() != Some("side")
        || valued.addresses[0].floor != Some(2)
        || valued.addresses[0].score.as_deref() != Some("10.5")
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
                score: Some("0".into()),
                is_verified: Some(false),
                verified_at: None,
                stamp: None,
                ..Default::default()
            }],
            ..Default::default()
        })
        .await?;
    let zeroed = contacts_repo
        .read_single(&Where::new(Operator::equals("id", omitted.id)))
        .await?
        .ok_or_else(|| DatasourceError::message("outlier zero"))?;
    if zeroed.addresses[0].latitude != Some(0)
        || zeroed.addresses[0].label_note.as_deref() != Some("")
        || zeroed.addresses[0].floor != Some(0)
        || zeroed.addresses[0].is_verified != Some(false)
    {
        return Err(DatasourceError::message("outlier zero"));
    }

    contacts_repo.delete(ada_after).await?;
    if contacts_repo
        .read_single(&Where::new(Operator::equals("id", ada.id)))
        .await?
        .is_some()
    {
        return Err(DatasourceError::message("delete ada"));
    }
    contacts_repo.delete(alan_read).await?;
    if contacts_repo
        .read_single(&Where::new(Operator::equals("id", alan.id)))
        .await?
        .is_some()
    {
        return Err(DatasourceError::message("delete alan"));
    }
    ds.close().await?;
    Ok(())
}
