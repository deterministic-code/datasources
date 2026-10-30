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
        addresses: vec![Address {
            address_type_name: "Home".into(),
            line1: "12 Analytical Engine".into(),
            city: "London".into(),
            country: Some("GB".into()),
            ..Default::default()
        }],
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
    if ada.first_name != "Ada" || ada.addresses.len() != 1 || ada.phones.len() != 1
        || ada.settings.get("theme") != Some(&"dark".into())
        || ada.prefs.get("home").map(|pref| pref.locale.as_str()) != Some("en-GB")
        || ada.addresses[0].line1 != "12 Analytical Engine"
        || ada.phones[0].number != "555-0100"
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
    if ada_read.addresses.len() != 1 || ada_read.phones.len() != 1 || ada_read.settings.len() != 1
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
                    address_type_name: "Home".into(),
                    line1: ada.addresses[0].line1.clone(),
                    city: "Cambridge".into(),
                    country: Some("GB".into()),
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
