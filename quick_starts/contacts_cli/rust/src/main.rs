mod datasources_providers;
mod datasources_register;
mod types;
use datasources_register::{Address, Contact, Email, Phone, Repositories};
use datasources::{default, DatasourceError, Operator, Result, Where};

#[tokio::main]
async fn main() -> Result<()> {
    datasources_providers::register();
    let ds = default()?;
    let contacts_repo = ds.contacts();
    let ada = contacts_repo
        .create(Contact {
            given_name: "Ada".into(),
            gender_type_name: "Female".into(),
            emails: vec![Email {
                email_type_name: "Personal".into(),
                address: "ada@example.com".into(),
                display_name: "Ada".into(),
                is_primary: true,
                ..Default::default()
            }],
            phones: vec![Phone {
                phone_type_name: "Mobile".into(),
                number: "555-0100".into(),
                display_name: "Ada mobile".into(),
                extention: "".into(),
                is_primary: true,
                ..Default::default()
            }],
            addresses: vec![Address {
                address_type_name: "Home".into(),
                line_1: "12 Analytical Engine".into(),
                city: "London".into(),
                country_code: "GB".into(),
                ..Default::default()
            }],
            ..Default::default()
        })
        .await?;
    println!(
        "created {} {} {} {} {}",
        ada.id, ada.given_name, ada.emails[0].address, ada.phones[0].number, ada.addresses[0].line_1
    );

    let alan = contacts_repo
        .create(Contact {
            given_name: "Alan".into(),
            gender_type_name: "Male".into(),
            emails: vec![Email {
                email_type_name: "Work".into(),
                address: "alan@work.example".into(),
                display_name: "Alan".into(),
                is_primary: true,
                ..Default::default()
            }],
            phones: vec![Phone {
                phone_type_name: "Work".into(),
                number: "555-0200".into(),
                display_name: "Alan office".into(),
                extention: "".into(),
                is_primary: true,
                ..Default::default()
            }],
            addresses: vec![Address {
                address_type_name: "Work".into(),
                line_1: "1 Bletchley Park".into(),
                city: "Bletchley".into(),
                country_code: "GB".into(),
                ..Default::default()
            }],
            ..Default::default()
        })
        .await?;
    println!(
        "created {} {} {} {} {}",
        alan.id, alan.given_name, alan.emails[0].address, alan.phones[0].number, alan.addresses[0].line_1
    );

    let ada_read = contacts_repo
        .read_single(Some(&Where::new(Operator::equals("id", ada.id))), None, None)
        .await?
        .ok_or_else(|| DatasourceError::message("read"))?;
    println!(
        "read {} {} emails={} phones={} addresses={}",
        ada_read.id,
        ada_read.given_name,
        ada_read.emails.len(),
        ada_read.phones.len(),
        ada_read.addresses.len()
    );

    let alan_read = contacts_repo
        .read_single(Some(&Where::new(Operator::equals("id", alan.id))), None, None)
        .await?
        .ok_or_else(|| DatasourceError::message("read"))?;
    println!(
        "read {} {} emails={} phones={} addresses={}",
        alan_read.id,
        alan_read.given_name,
        alan_read.emails.len(),
        alan_read.phones.len(),
        alan_read.addresses.len()
    );

    let ada_saved = contacts_repo
        .update(
            Contact {
                id: ada.id,
                given_name: "Ada Byron".into(),
                gender_type_name: "Female".into(),
                emails: vec![
                    Email {
                        id: ada.emails[0].id,
                        email_type_name: "Personal".into(),
                        address: ada.emails[0].address.clone(),
                        display_name: "Ada".into(),
                        is_primary: true,
                        ..Default::default()
                    },
                    Email {
                        email_type_name: "Work".into(),
                        address: "ada@byron.example".into(),
                        display_name: "Ada work".into(),
                        is_primary: false,
                        ..Default::default()
                    },
                ],
                phones: vec![Phone {
                    id: ada.phones[0].id,
                    phone_type_name: "Mobile".into(),
                    number: "555-0111".into(),
                    display_name: "Ada mobile".into(),
                    extention: "".into(),
                    is_primary: true,
                    ..Default::default()
                }],
                addresses: vec![Address {
                    id: ada.addresses[0].id,
                    address_type_name: "Home".into(),
                    line_1: ada.addresses[0].line_1.clone(),
                    city: "Cambridge".into(),
                    country_code: "GB".into(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            None,
        )
        .await?;
    let ada_after = contacts_repo
        .read_single(Some(&Where::new(Operator::equals("id", ada.id))), None, None)
        .await?
        .ok_or_else(|| DatasourceError::message("update"))?;
    let mut ada_emails: Vec<_> = ada_after.emails.iter().map(|email| email.address.clone()).collect();
    ada_emails.sort();
    let mut ada_phones: Vec<_> = ada_after.phones.iter().map(|phone| phone.number.clone()).collect();
    ada_phones.sort();
    println!(
        "updated {} {} rows={} emails={} phones={} {}",
        ada_after.id,
        ada_after.given_name,
        ada_saved.rows_affected,
        ada_emails.join(","),
        ada_phones.join(","),
        ada_after.addresses[0].city
    );

    let alan_saved = contacts_repo
        .update(
            Contact {
                id: alan.id,
                given_name: "Alan".into(),
                gender_type_name: "Male".into(),
                emails: vec![Email {
                    id: alan.emails[0].id,
                    email_type_name: "Work".into(),
                    address: "alan@bletchley.example".into(),
                    display_name: "Alan".into(),
                    is_primary: true,
                    ..Default::default()
                }],
                phones: vec![Phone {
                    id: alan.phones[0].id,
                    phone_type_name: "Work".into(),
                    number: "555-0201".into(),
                    display_name: "Alan office".into(),
                    extention: "".into(),
                    is_primary: true,
                    ..Default::default()
                }],
                addresses: vec![Address {
                    id: alan.addresses[0].id,
                    address_type_name: "Work".into(),
                    line_1: "2 Hut 8".into(),
                    city: alan.addresses[0].city.clone(),
                    country_code: "GB".into(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            None,
        )
        .await?;
    let alan_after = contacts_repo
        .read_single(Some(&Where::new(Operator::equals("id", alan.id))), None, None)
        .await?
        .ok_or_else(|| DatasourceError::message("update"))?;
    println!(
        "updated {} {} rows={} emails={} phones={} {}",
        alan_after.id,
        alan_after.given_name,
        alan_saved.rows_affected,
        alan_after.emails.iter().map(|email| email.address.clone()).collect::<Vec<_>>().join(","),
        alan_after.phones.iter().map(|phone| phone.number.clone()).collect::<Vec<_>>().join(","),
        alan_after.addresses[0].line_1
    );

    let ada_removed = contacts_repo.delete(ada_after, None).await?;
    if contacts_repo
        .read_single(Some(&Where::new(Operator::equals("id", ada.id))), None, None)
        .await?
        .is_some()
    {
        return Err(DatasourceError::message("delete"));
    }
    println!("deleted {} rows={}", ada.id, ada_removed.rows_affected);

    let alan_removed = contacts_repo.delete(alan_after, None).await?;
    if contacts_repo
        .read_single(Some(&Where::new(Operator::equals("id", alan.id))), None, None)
        .await?
        .is_some()
    {
        return Err(DatasourceError::message("delete"));
    }
    println!("deleted {} rows={}", alan.id, alan_removed.rows_affected);
    Ok(())
}
