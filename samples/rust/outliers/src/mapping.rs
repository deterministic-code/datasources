use crate::format::{map_error, map_lines};
use crate::mapping_cell::MappingCell;
use datasources::chrono::{TimeZone, Utc};
use datasources::rust_decimal::Decimal;
use datasources::uuid::Uuid;
use datasources::{
    eq, Casing, ChangeCasingMapper, Column, Connection, CreateTableStatement, CRUDRepository,
    CRUDRepositoryOptions, IdentityMapper, Mapper, PrimaryKey, Schema, SchemaColumn, Value, Where,
};
use futures_util::TryStreamExt;
use std::collections::HashMap;
use std::sync::Arc;

const TOKEN: &str = "550e8400-e29b-41d4-a716-446655440000";

pub async fn run(connection: Arc<dyn Connection>) -> Vec<String> {
    let mut lines = Vec::new();
    let command = connection.create_command();
    if let Err(error) = command
        .execute_statement(&create_table(connection.provider()), None)
        .await
    {
        return map_error("create_table", &error.to_string());
    }
    let Ok(cells) = CRUDRepository::<MappingCell>::new(Arc::clone(&connection), options()) else {
        return map_error("create", "repository");
    };
    let mut created = match cells.create(seed(1)).await {
        Ok(row) => {
            lines.extend(map_lines("create", &pairs(&row)));
            row
        }
        Err(error) => {
            lines.extend(map_error("create", &error.to_string()));
            seed(1)
        }
    };
    match cells.read(None, None).try_collect::<Vec<_>>().await {
        Ok(rows) => {
            for row in rows {
                lines.extend(map_lines("read", &pairs(&row)));
            }
        }
        Err(error) => lines.extend(map_error("read", &error.to_string())),
    }
    let clause = Where::binary("id", eq, Value::from(created.id));
    match cells.read_where(&clause, None).try_collect::<Vec<_>>().await {
        Ok(rows) => {
            for row in rows {
                lines.extend(map_lines("read_where", &pairs(&row)));
            }
        }
        Err(error) => lines.extend(map_error("read_where", &error.to_string())),
    }
    push_opt(&mut lines, "read_single", cells.read_single(None, None).await);
    push_opt(&mut lines, "read_single_where", cells.read_single_where(&clause, None).await);
    push_opt(
        &mut lines,
        "read_single_by_id",
        cells.read_single_by_id(&Value::from(created.id)).await,
    );
    let mut key = datasources::Row::new();
    key.insert("id".into(), Value::from(created.id));
    push_opt(&mut lines, "read_single_by_id_row", cells.read_single_by_id(&key).await);
    created.text_to_string = "grace".into();
    push_write(
        &mut lines,
        "update",
        cells.update(created.clone(), None).await,
        cells.read_single_by_id(&Value::from(created.id)).await,
    );
    created.text_to_string = "lin".into();
    push_write(
        &mut lines,
        "update_where",
        cells.update(created.clone(), Some(&clause)).await,
        cells.read_single_by_id(&Value::from(created.id)).await,
    );
    let mut extra = seed(2);
    extra.text_to_string = "second".into();
    let extra = cells.create(extra.clone()).await.unwrap_or(extra);
    push_write(&mut lines, "delete", cells.delete(extra.clone(), None).await, Ok(Some(extra)));
    push_write(
        &mut lines,
        "delete_where",
        cells.delete(created.clone(), Some(&clause)).await,
        Ok(Some(created)),
    );
    match cells.create(seed(3)).await {
        Ok(row) => lines.extend(map_lines("create_again", &pairs(&row))),
        Err(error) => lines.extend(map_error("create_again", &error.to_string())),
    }
    lines
}

fn push_opt(
    lines: &mut Vec<String>,
    method: &str,
    result: datasources::Result<Option<MappingCell>>,
) {
    match result {
        Ok(Some(row)) => lines.extend(map_lines(method, &pairs(&row))),
        Ok(None) => lines.push(format!("map {method}")),
        Err(error) => lines.extend(map_error(method, &error.to_string())),
    }
}

fn push_write(
    lines: &mut Vec<String>,
    method: &str,
    written: datasources::Result<datasources::Response>,
    read: datasources::Result<Option<MappingCell>>,
) {
    match written {
        Ok(_) => match read {
            Ok(Some(row)) => lines.extend(map_lines(method, &pairs(&row))),
            Ok(None) => lines.push(format!("map {method}")),
            Err(error) => lines.extend(map_error(method, &error.to_string())),
        },
        Err(error) => lines.extend(map_error(method, &error.to_string())),
    }
}

fn options() -> CRUDRepositoryOptions {
    let mappings = Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
    let names = [
        "id",
        "text_to_string",
        "text_to_integer",
        "text_to_decimal",
        "text_to_datetime",
        "text_to_binary",
        "text_to_guid",
        "int_to_integer",
        "int_to_float",
        "int_to_decimal",
        "int_to_boolean",
        "float_to_integer",
        "float_to_float",
        "float_to_decimal",
        "decimal_to_integer",
        "decimal_to_float",
        "decimal_to_decimal",
        "bool_to_boolean",
        "datetime_to_datetime",
        "binary_to_binary",
        "binary_to_guid",
        "guid_to_string",
        "guid_to_guid",
    ];
    let mut columns = HashMap::new();
    columns.insert(
        "id".into(),
        SchemaColumn { type_name: "integer".into(), is_id: true, ..SchemaColumn::new("integer") },
    );
    columns.insert("text_to_string".into(), SchemaColumn::new("string"));
    columns.insert("text_to_integer".into(), SchemaColumn::new("integer"));
    columns.insert("text_to_decimal".into(), SchemaColumn::new("decimal"));
    columns.insert("text_to_datetime".into(), SchemaColumn::new("datetime"));
    columns.insert("text_to_binary".into(), SchemaColumn::new("binary"));
    columns.insert("text_to_guid".into(), SchemaColumn::new("uuid"));
    columns.insert("int_to_integer".into(), SchemaColumn::new("integer"));
    columns.insert("int_to_float".into(), SchemaColumn::new("float"));
    columns.insert("int_to_decimal".into(), SchemaColumn::new("decimal"));
    columns.insert("int_to_boolean".into(), SchemaColumn::new("boolean"));
    columns.insert("float_to_integer".into(), SchemaColumn::new("integer"));
    columns.insert("float_to_float".into(), SchemaColumn::new("float"));
    columns.insert("float_to_decimal".into(), SchemaColumn::new("decimal"));
    columns.insert("decimal_to_integer".into(), SchemaColumn::new("integer"));
    columns.insert("decimal_to_float".into(), SchemaColumn::new("float"));
    columns.insert("decimal_to_decimal".into(), SchemaColumn::new("decimal"));
    columns.insert("bool_to_boolean".into(), SchemaColumn::new("boolean"));
    columns.insert("datetime_to_datetime".into(), SchemaColumn::new("datetime"));
    columns.insert("binary_to_binary".into(), SchemaColumn::new("binary"));
    columns.insert("binary_to_guid".into(), SchemaColumn::new("uuid"));
    columns.insert("guid_to_string".into(), SchemaColumn::new("string"));
    columns.insert("guid_to_guid".into(), SchemaColumn::new("uuid"));
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "MappingCell".into();
    options.entity_mapper = Some(mappings as Arc<dyn Mapper<String, String> + Send + Sync>);
    options.property_mappings =
        Some(Arc::new(IdentityMapper::<String>::new()) as Arc<dyn Mapper<String, String> + Send + Sync>);
    options.ids = vec!["id".into()];
    options.read_properties = names.iter().map(|name| (*name).into()).collect();
    options.create_properties = Some(names.iter().map(|name| (*name).into()).collect());
    options.update_properties =
        Some(names.iter().filter(|name| **name != "id").map(|name| (*name).into()).collect());
    options.schema = Schema::new(columns);
    options
}

fn create_table(provider: &str) -> CreateTableStatement {
    let text = native(provider, "text");
    let integer = native(provider, "integer");
    let floating = native(provider, "float");
    let decimal = native(provider, "decimal");
    let boolean = native(provider, "boolean");
    let datetime = native(provider, "datetime");
    let binary = native(provider, "binary");
    let guid = native(provider, "guid");
    let columns = [
        Column::new("id", integer).identity(),
        Column::new("text_to_string", text),
        Column::new("text_to_integer", text),
        Column::new("text_to_decimal", text),
        Column::new("text_to_datetime", text),
        Column::new("text_to_binary", text),
        Column::new("text_to_guid", text),
        Column::new("int_to_integer", integer),
        Column::new("int_to_float", integer),
        Column::new("int_to_decimal", integer),
        Column::new("int_to_boolean", integer),
        Column::new("float_to_integer", floating),
        Column::new("float_to_float", floating),
        Column::new("float_to_decimal", floating),
        Column::new("decimal_to_integer", decimal),
        Column::new("decimal_to_float", decimal),
        Column::new("decimal_to_decimal", decimal),
        Column::new("bool_to_boolean", boolean),
        Column::new("datetime_to_datetime", datetime),
        Column::new("binary_to_binary", binary),
        Column::new("binary_to_guid", binary),
        Column::new("guid_to_string", guid),
        Column::new("guid_to_guid", guid),
    ];
    let constraints = if provider == "sqlite" {
        vec![]
    } else {
        vec![PrimaryKey::new("pk_mapping_cell", &["id"])]
    };
    CreateTableStatement::new("mapping_cell", &columns, &constraints)
}

fn native<'a>(provider: &str, source: &'a str) -> &'a str {
    match (provider, source) {
        ("mysql", "text") => "TEXT",
        ("mysql", "integer") => "INT",
        ("mysql", "float") => "DOUBLE",
        ("mysql", "decimal") => "DECIMAL(28, 14)",
        ("mysql", "boolean") => "TINYINT(1)",
        ("mysql", "datetime") => "DATETIME(3)",
        ("mysql", "binary") => "BLOB",
        ("mysql", "guid") => "CHAR(36)",
        ("postgres", "text") => "TEXT",
        ("postgres", "integer") => "INTEGER",
        ("postgres", "float") => "DOUBLE PRECISION",
        ("postgres", "decimal") => "NUMERIC",
        ("postgres", "boolean") => "BOOLEAN",
        ("postgres", "datetime") => "TIMESTAMPTZ",
        ("postgres", "binary") => "BYTEA",
        ("postgres", "guid") => "UUID",
        (_, "text") => "TEXT",
        (_, "integer") => "INTEGER",
        (_, "float") => "REAL",
        (_, "decimal") => "NUMERIC",
        (_, "boolean") => "BOOLEAN",
        (_, "datetime") => "TEXT",
        (_, "binary") => "BLOB",
        _ => "TEXT",
    }
}

fn seed(id: i64) -> MappingCell {
    let token = Uuid::parse_str(TOKEN).unwrap();
    let opened = Utc.with_ymd_and_hms(2020, 1, 2, 3, 4, 5).unwrap();
    MappingCell {
        id,
        text_to_string: "ada".into(),
        text_to_integer: 10,
        text_to_decimal: Decimal::new(1050, 2),
        text_to_datetime: opened,
        text_to_binary: vec![1, 2, 3, 4],
        text_to_guid: token,
        int_to_integer: 10,
        int_to_float: 10.0,
        int_to_decimal: Decimal::from(7),
        int_to_boolean: true,
        float_to_integer: 10,
        float_to_float: 10.9,
        float_to_decimal: Decimal::new(105, 1),
        decimal_to_integer: 10,
        decimal_to_float: 1.2345678901234567,
        decimal_to_decimal: Decimal::new(1050, 2),
        bool_to_boolean: true,
        datetime_to_datetime: opened,
        binary_to_binary: vec![1, 2, 3, 4],
        binary_to_guid: token,
        guid_to_string: token.to_string(),
        guid_to_guid: token,
    }
}

fn pairs(row: &MappingCell) -> Vec<(&'static str, String)> {
    let empty_date = Utc.timestamp_opt(0, 0).single().unwrap();
    let date = |value: datasources::chrono::DateTime<Utc>| {
        if value == empty_date {
            String::new()
        } else {
            value.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
        }
    };
    let guid = |value: Uuid| if value.is_nil() { String::new() } else { value.to_string() };
    let bytes = |value: &[u8]| value.iter().map(|byte| format!("{byte:02x}")).collect();
    vec![
        ("id", row.id.to_string()),
        ("text_to_string", row.text_to_string.clone()),
        ("text_to_integer", row.text_to_integer.to_string()),
        ("text_to_decimal", row.text_to_decimal.to_string()),
        ("text_to_datetime", date(row.text_to_datetime)),
        ("text_to_binary", bytes(&row.text_to_binary)),
        ("text_to_guid", guid(row.text_to_guid)),
        ("int_to_integer", row.int_to_integer.to_string()),
        ("int_to_float", row.int_to_float.to_string()),
        ("int_to_decimal", row.int_to_decimal.to_string()),
        ("int_to_boolean", if row.int_to_boolean { "true".into() } else { "false".into() }),
        ("float_to_integer", row.float_to_integer.to_string()),
        ("float_to_float", row.float_to_float.to_string()),
        ("float_to_decimal", row.float_to_decimal.to_string()),
        ("decimal_to_integer", row.decimal_to_integer.to_string()),
        ("decimal_to_float", row.decimal_to_float.to_string()),
        ("decimal_to_decimal", row.decimal_to_decimal.to_string()),
        ("bool_to_boolean", if row.bool_to_boolean { "true".into() } else { "false".into() }),
        ("datetime_to_datetime", date(row.datetime_to_datetime)),
        ("binary_to_binary", bytes(&row.binary_to_binary)),
        ("binary_to_guid", guid(row.binary_to_guid)),
        ("guid_to_string", row.guid_to_string.clone()),
        ("guid_to_guid", guid(row.guid_to_guid)),
    ]
}
