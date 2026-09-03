use datasources::{
    encode_rows, DeepCRUDRepository, DatasourceError, Repository, Result, Row, Value, WhereBuilder,
};

fn record(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(k, v)| ((*k).to_string(), v.clone()))
        .collect()
}

pub async fn add_extra_contact(service: &DeepCRUDRepository) -> Result<Row> {
    let stamp = Value::from("2024-01-01T00:00:00Z");
    let version = Value::Binary(vec![]);
    let uuid = Value::from("00000000-0000-4000-8000-000000000251");
    let response = service
        .add(&record(&[
            ("uuid", uuid.clone()),
            ("created", stamp.clone()),
            ("updated", stamp.clone()),
            ("version", version.clone()),
            ("contact_source_id", Value::Int(1)),
            ("contact_type_id", Value::Int(1)),
            ("first_name", Value::from("Extra")),
            ("last_name", Value::from("Contact")),
            (
                "addresses",
                encode_rows(&[record(&[
                    ("uuid", uuid.clone()),
                    ("created", stamp.clone()),
                    ("updated", stamp.clone()),
                    ("version", version.clone()),
                    ("line1", Value::from("9 Extra")),
                    ("city", Value::from("Oslo")),
                ])]),
            ),
            (
                "phones",
                encode_rows(&[record(&[
                    ("uuid", uuid),
                    ("created", stamp.clone()),
                    ("updated", stamp),
                    ("version", version),
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
