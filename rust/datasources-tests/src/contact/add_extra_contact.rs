use super::contact_lookups::ContactLookups;
use chrono::{DateTime, Utc};
use datasources::{
    encode_rows, CRUDRepository, DatasourceError, ICRUDRepository, IReadonlyRepository, Operator,
    Result, Row, Value, Where,
};
use uuid::Uuid;

fn record(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(k, v)| ((*k).to_string(), v.clone()))
        .collect()
}

pub async fn add_extra_contact(service: &CRUDRepository, ids: &ContactLookups) -> Result<Row> {
    let stamp = Value::DateTime(
        DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
            .expect("stamp")
            .with_timezone(&Utc),
    );
    let version = Value::Binary(vec![]);
    let uuid = Value::Uuid(Uuid::new_v4());
    let created = service
        .create(record(&[
            ("Uuid", uuid.clone()),
            ("Created", stamp.clone()),
            ("Updated", stamp.clone()),
            ("Version", version.clone()),
            ("ContactSource", Value::Int(ids.contact_source_id)),
            ("ContactType", Value::Int(ids.contact_type_id)),
            ("FirstName", Value::from("Extra")),
            ("LastName", Value::from("Contact")),
            (
                "Addresses",
                encode_rows(&[record(&[
                    ("Uuid", Value::Uuid(Uuid::new_v4())),
                    ("Created", stamp.clone()),
                    ("Updated", stamp.clone()),
                    ("Version", version.clone()),
                    ("AddressType", Value::Int(ids.address_type_id)),
                    ("Line1", Value::from("9 Extra")),
                    ("City", Value::from("Oslo")),
                ])]),
            ),
            (
                "Phones",
                encode_rows(&[record(&[
                    ("Uuid", Value::Uuid(Uuid::new_v4())),
                    ("Created", stamp.clone()),
                    ("Updated", stamp),
                    ("Version", version),
                    ("PhoneType", Value::Int(ids.phone_type_id)),
                    ("Number", Value::from("555-extra")),
                ])]),
            ),
        ]))
        .await?;
    let Value::Int(id) = created.get("Id").cloned().unwrap_or(Value::Null) else {
        return Err(DatasourceError::message("expected insert id"));
    };
    service
        .read_single(
            Some(&Where::new(Operator::equals("Id", Value::Int(id)))),
            None, None
        )
        .await?
        .ok_or_else(|| DatasourceError::message(format!("extra contact {id} missing")))
}
