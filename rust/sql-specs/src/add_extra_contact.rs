use crate::contact_lookups::ContactLookups;
use chrono::{DateTime, Utc};
use datasources::{
    encode_rows, DatasourceError, DeepCRUDRepository, ReadonlyRepository, Repository, Result, Row,
    Value, WhereBuilder,
};
use uuid::Uuid;

fn record(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(k, v)| ((*k).to_string(), v.clone()))
        .collect()
}

pub async fn add_extra_contact(service: &DeepCRUDRepository, ids: &ContactLookups) -> Result<Row> {
    let stamp = Value::DateTime(
        DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
            .expect("stamp")
            .with_timezone(&Utc),
    );
    let version = Value::Binary(vec![]);
    let uuid = Value::Uuid(Uuid::new_v4());
    let response = service
        .add(&record(&[
            ("uuid", uuid.clone()),
            ("created", stamp.clone()),
            ("updated", stamp.clone()),
            ("version", version.clone()),
            ("contact_source_id", Value::Int(ids.contact_source_id)),
            ("contact_type_id", Value::Int(ids.contact_type_id)),
            ("first_name", Value::from("Extra")),
            ("last_name", Value::from("Contact")),
            (
                "addresses",
                encode_rows(&[record(&[
                    ("uuid", Value::Uuid(Uuid::new_v4())),
                    ("created", stamp.clone()),
                    ("updated", stamp.clone()),
                    ("version", version.clone()),
                    ("address_type_id", Value::Int(ids.address_type_id)),
                    ("line1", Value::from("9 Extra")),
                    ("city", Value::from("Oslo")),
                ])]),
            ),
            (
                "phones",
                encode_rows(&[record(&[
                    ("uuid", Value::Uuid(Uuid::new_v4())),
                    ("created", stamp.clone()),
                    ("updated", stamp),
                    ("version", version),
                    ("phone_type_id", Value::Int(ids.phone_type_id)),
                    ("number", Value::from("555-extra")),
                ])]),
            ),
        ]))
        .await?;
    let Value::Int(id) = response.result else {
        return Err(DatasourceError::message("expected insert id"));
    };
    service
        .get_single_where(&WhereBuilder::equals("id", Value::Int(id)))
        .await?
        .ok_or_else(|| DatasourceError::message(format!("extra contact {id} missing")))
}
