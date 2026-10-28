mod datasources_providers;
mod datasources_register;
use datasources_register::Repositories;
use datasources::{
    default, encode_rows, DatasourceError, ICRUDRepository, IReadonlyRepository, Operator, Result,
    Row, Value, Where,
};

#[tokio::main]
async fn main() -> Result<()> {
    let ds = default()?;
    let contacts_repo = ds.contacts();
    let mut ada_row = contact(
        "Ada",
        "Female",
        &[email(None, "Personal", "ada@example.com", "Ada", true)],
        &[phone(None, "Mobile", "555-0100", "Ada mobile", true)],
        &[address(None, "Home", "12 Analytical Engine", "London")],
    );
    let ada = contacts_repo.create(&mut ada_row).await?;
    let ada_id = id_of(&ada)?;
    println!(
        "created {ada_id} Ada {} {} {}",
        cells(&ada, "emails", "address"),
        cells(&ada, "phones", "number"),
        cells(&ada, "addresses", "line_1")
    );

    let mut alan_row = contact(
        "Alan",
        "Male",
        &[email(None, "Work", "alan@work.example", "Alan", true)],
        &[phone(None, "Work", "555-0200", "Alan office", true)],
        &[address(None, "Work", "1 Bletchley Park", "Bletchley")],
    );
    let alan = contacts_repo.create(&mut alan_row).await?;
    let alan_id = id_of(&alan)?;
    println!(
        "created {alan_id} Alan {} {} {}",
        cells(&alan, "emails", "address"),
        cells(&alan, "phones", "number"),
        cells(&alan, "addresses", "line_1")
    );

    let ada_read = contacts_repo
        .read_single(Some(&Where::new(Operator::equals("id", ada_id))), None, None)
        .await?
        .ok_or_else(|| DatasourceError::message("read"))?;
    println!(
        "read {ada_id} Ada emails={} phones={} addresses={}",
        rows(ada_read.get("emails")).len(),
        rows(ada_read.get("phones")).len(),
        rows(ada_read.get("addresses")).len()
    );

    let alan_read = contacts_repo
        .read_single(Some(&Where::new(Operator::equals("id", alan_id))), None, None)
        .await?
        .ok_or_else(|| DatasourceError::message("read"))?;
    println!(
        "read {alan_id} Alan emails={} phones={} addresses={}",
        rows(alan_read.get("emails")).len(),
        rows(alan_read.get("phones")).len(),
        rows(alan_read.get("addresses")).len()
    );

    let ada_email = first(&ada, "emails")?;
    let ada_phone = first(&ada, "phones")?;
    let ada_address = first(&ada, "addresses")?;
    let mut ada_update = contact(
        "Ada Byron",
        "Female",
        &[
            email(
                ada_email.get("id").cloned(),
                "Personal",
                "ada@example.com",
                "Ada",
                true,
            ),
            email(None, "Work", "ada@byron.example", "Ada work", false),
        ],
        &[phone(
            ada_phone.get("id").cloned(),
            "Mobile",
            "555-0111",
            "Ada mobile",
            true,
        )],
        &[address(
            ada_address.get("id").cloned(),
            "Home",
            ada_address
                .get("line_1")
                .map(Value::wire_text)
                .unwrap_or_default(),
            "Cambridge",
        )],
    );
    ada_update.insert("id".into(), Value::from(ada_id));
    let ada_saved = contacts_repo.update(&mut ada_update, None).await?;
    let ada_after = contacts_repo
        .read_single(Some(&Where::new(Operator::equals("id", ada_id))), None, None)
        .await?
        .ok_or_else(|| DatasourceError::message("update"))?;
    println!(
        "updated {ada_id} Ada Byron rows={} emails={} phones={} {}",
        ada_saved.rows_affected,
        cells(&ada_after, "emails", "address"),
        cells(&ada_after, "phones", "number"),
        cells(&ada_after, "addresses", "city")
    );

    let alan_email = first(&alan, "emails")?;
    let alan_phone = first(&alan, "phones")?;
    let alan_address = first(&alan, "addresses")?;
    let mut alan_update = contact(
        "Alan",
        "Male",
        &[email(
            alan_email.get("id").cloned(),
            "Work",
            "alan@bletchley.example",
            "Alan",
            true,
        )],
        &[phone(
            alan_phone.get("id").cloned(),
            "Work",
            "555-0201",
            "Alan office",
            true,
        )],
        &[address(
            alan_address.get("id").cloned(),
            "Work",
            "2 Hut 8",
            alan_address
                .get("city")
                .map(Value::wire_text)
                .unwrap_or_default(),
        )],
    );
    alan_update.insert("id".into(), Value::from(alan_id));
    let alan_saved = contacts_repo.update(&mut alan_update, None).await?;
    let alan_after = contacts_repo
        .read_single(Some(&Where::new(Operator::equals("id", alan_id))), None, None)
        .await?
        .ok_or_else(|| DatasourceError::message("update"))?;
    println!(
        "updated {alan_id} Alan rows={} emails={} phones={} {}",
        alan_saved.rows_affected,
        cells(&alan_after, "emails", "address"),
        cells(&alan_after, "phones", "number"),
        cells(&alan_after, "addresses", "line_1")
    );

    let ada_removed = contacts_repo.delete(&ada_after, None).await?;
    if contacts_repo
        .read_single(Some(&Where::new(Operator::equals("id", ada_id))), None, None)
        .await?
        .is_some()
    {
        return Err(DatasourceError::message("delete"));
    }
    println!("deleted {ada_id} rows={}", ada_removed.rows_affected);

    let alan_removed = contacts_repo.delete(&alan_after, None).await?;
    if contacts_repo
        .read_single(Some(&Where::new(Operator::equals("id", alan_id))), None, None)
        .await?
        .is_some()
    {
        return Err(DatasourceError::message("delete"));
    }
    println!("deleted {alan_id} rows={}", alan_removed.rows_affected);
    Ok(())
}

fn contact(given_name: &str, gender: &str, emails: &[Row], phones: &[Row], addresses: &[Row]) -> Row {
    [
        ("name", Value::from(given_name)),
        ("given_name", Value::from(given_name)),
        ("gender_type_name", Value::from(gender)),
        ("is_starred", Value::from(false)),
        ("is_favorite", Value::from(false)),
        ("is_deleted", Value::from(false)),
        ("emails", encode_rows(emails)),
        ("phones", encode_rows(phones)),
        ("addresses", encode_rows(addresses)),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value))
    .collect()
}

fn email(id: Option<Value>, type_name: &str, address: &str, display_name: &str, primary: bool) -> Row {
    let mut row = [
        ("email_type_name", Value::from(type_name)),
        ("address", Value::from(address)),
        ("display_name", Value::from(display_name)),
        ("is_primary", Value::from(primary)),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value))
    .collect::<Row>();
    if let Some(id) = id {
        row.insert("id".into(), id);
    }
    row
}

fn phone(id: Option<Value>, type_name: &str, number: &str, display_name: &str, primary: bool) -> Row {
    let mut row = [
        ("phone_type_name", Value::from(type_name)),
        ("number", Value::from(number)),
        ("display_name", Value::from(display_name)),
        ("extention", Value::from("")),
        ("is_primary", Value::from(primary)),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value))
    .collect::<Row>();
    if let Some(id) = id {
        row.insert("id".into(), id);
    }
    row
}

fn address(id: Option<Value>, type_name: &str, line1: impl Into<String>, city: impl Into<String>) -> Row {
    let mut row = [
        ("address_type_name", Value::from(type_name)),
        ("line_1", Value::from(line1.into())),
        ("city", Value::from(city.into())),
        ("country_code", Value::from("GB")),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value))
    .collect::<Row>();
    if let Some(id) = id {
        row.insert("id".into(), id);
    }
    row
}

fn id_of(row: &Row) -> Result<i64> {
    match row.get("id") {
        Some(Value::Int(id)) => Ok(*id),
        _ => Err(DatasourceError::message("id")),
    }
}

fn first(row: &Row, list: &str) -> Result<Row> {
    rows(row.get(list))
        .into_iter()
        .next()
        .ok_or_else(|| DatasourceError::message(list))
}

fn cells(row: &Row, list: &str, field: &str) -> String {
    let mut values: Vec<_> = rows(row.get(list))
        .iter()
        .filter_map(|item| item.get(field).map(Value::wire_text))
        .collect();
    values.sort();
    values.join(",")
}

fn rows(value: Option<&Value>) -> Vec<Row> {
    match value {
        Some(Value::Text(text)) if !text.is_empty() => text.split('|').map(row_of).collect(),
        _ => Vec::new(),
    }
}

fn row_of(text: &str) -> Row {
    text.split(',')
        .filter_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            Some((
                key.to_string(),
                match value {
                    "true" => Value::Bool(true),
                    "false" => Value::Bool(false),
                    _ => value.parse::<i64>().map(Value::Int).unwrap_or_else(|_| Value::from(value)),
                },
            ))
        })
        .collect()
}
