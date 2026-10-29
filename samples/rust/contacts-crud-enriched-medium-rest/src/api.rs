use futures_util::TryStreamExt;
use crate::clock::Clock;
use crate::json_row::{
    ensure_text, json_to_row, object_from, row_to_json, rows_to_json, set_text, text,
};
use crate::repositories::contact_group_repository::ContactGroupRepository;
use crate::repositories::contact_repository::ContactRepository;
use datasources::{
    DatasourceError, Operator, ReadInput, Row, Value, Where, CRUDRepository, ICRUDRepository,
    IReadonlyRepository,
};
use serde_json::{Map, Value as Json};
use std::collections::HashMap;

pub struct HttpResult {
    pub status: u16,
    pub body: String,
}

pub struct Api {
    contacts: ContactRepository,
    legacy: CRUDRepository,
    groups_repo: ContactGroupRepository,
    clock: Clock,
}

impl Api {
    pub fn new(
        contacts: ContactRepository,
        legacy: CRUDRepository,
        groups_repo: ContactGroupRepository,
        clock: Clock,
    ) -> Self {
        Self { contacts, legacy, groups_repo, clock }
    }

    pub async fn handle(
        &self,
        method: &str,
        target: &str,
        body: Option<&str>,
    ) -> Result<HttpResult, String> {
        let split = target.find('?');
        let raw = split.map(|i| &target[..i]).unwrap_or(target);
        let path = raw.trim_end_matches('/');
        let path = if path.is_empty() { "/" } else { path };
        let query = parse(split.map(|i| &target[i + 1..]).unwrap_or(""));
        match (method.to_ascii_uppercase().as_str(), path) {
            ("GET", "/contacts") => self.get_contacts(&query).await,
            ("POST", "/contacts") => self.post_contact(body).await,
            ("PUT", "/contacts") => self.put_contact(&query, body).await,
            ("DELETE", "/contacts") => self.delete_contact(&query).await,
            ("GET", "/legacy_contacts") => self.get_legacy(&query).await,
            ("POST", "/legacy_contacts") => self.post_legacy(body).await,
            ("PUT", "/legacy_contacts") => self.put_legacy(&query, body).await,
            ("DELETE", "/legacy_contacts") => self.delete_legacy(&query).await,
            ("GET", "/contact_groups") => self.get_groups(&query).await,
            ("POST", "/contact_groups") => self.post_group(body).await,
            ("PUT", "/contact_groups") => self.put_group(&query, body).await,
            _ => Ok(HttpResult {
                status: 404,
                body: String::new(),
            }),
        }
        .or_else(map_error)
    }

    async fn get_contacts(&self, query: &HashMap<String, String>) -> Result<HttpResult, ApiError> {
        Ok(ok(200, rows_to_json(&self.read_rows(&*self.contacts, query, &["id", "first_name", "last_name", "email"]).await?)))
    }

    async fn post_contact(&self, body: Option<&str>) -> Result<HttpResult, ApiError> {
        let row = self.stamp(object_from(body).map_err(ApiError::bad)?)?;
        Ok(ok(201, row_to_json(&self.contacts.create(row).await?)))
    }

    async fn put_contact(
        &self,
        query: &HashMap<String, String>,
        body: Option<&str>,
    ) -> Result<HttpResult, ApiError> {
        let id = required_number(query, "id")?;
        let by = Where::new(Operator::equals("id", id));
        if self.contacts.read_single(Some(&by), None, None).await?.is_none() {
            return Ok(empty(404));
        }
        let mut row = self.stamp(object_from(body).map_err(ApiError::bad)?)?;
        row.insert("id".into(), Value::Int(id));
        self.contacts.update(row, Some(&by)).await?;
        Ok(ok(200, row_to_json(&self.contacts.read_single(Some(&by), None, None).await?.unwrap_or_default())))
    }

    async fn delete_contact(&self, query: &HashMap<String, String>) -> Result<HttpResult, ApiError> {
        let id = required_number(query, "id")?;
        let by = Where::new(Operator::equals("id", id));
        let Some(row) = self.contacts.read_single(Some(&by), None, None).await? else {
            return Ok(empty(404));
        };
        self.contacts.delete(row, Some(&by)).await?;
        Ok(empty(204))
    }

    async fn get_groups(&self, query: &HashMap<String, String>) -> Result<HttpResult, ApiError> {
        Ok(ok(200, rows_to_json(&self.read_rows(&*self.groups_repo, query, &["id"]).await?)))
    }

    async fn post_group(&self, body: Option<&str>) -> Result<HttpResult, ApiError> {
        let row = self.stamp_group(object_from(body).map_err(ApiError::bad)?)?;
        Ok(ok(201, row_to_json(&self.groups_repo.create(row).await?)))
    }

    async fn put_group(
        &self,
        query: &HashMap<String, String>,
        body: Option<&str>,
    ) -> Result<HttpResult, ApiError> {
        let id = required_number(query, "id")?;
        let by = Where::new(Operator::equals("id", id));
        if self.groups_repo.read_single(Some(&by), None, None).await?.is_none() {
            return Ok(empty(404));
        }
        let mut row = self.stamp_group(object_from(body).map_err(ApiError::bad)?)?;
        row.insert("id".into(), Value::Int(id));
        self.groups_repo.update(row, Some(&by)).await?;
        Ok(ok(200, row_to_json(&self.groups_repo.read_single(Some(&by), None, None).await?.unwrap_or_default())))
    }

    async fn get_legacy(&self, query: &HashMap<String, String>) -> Result<HttpResult, ApiError> {
        Ok(ok(200, rows_to_json(&self.read_rows(&self.legacy, query, &["key", "first_name", "last_name", "email"]).await?)))
    }

    async fn post_legacy(&self, body: Option<&str>) -> Result<HttpResult, ApiError> {
        let row = text_legacy(json_to_row(&object_from(body).map_err(ApiError::bad)?)?, &self.clock);
        if text(row.get("key")).is_empty() {
            return Err(ApiError::bad("key is required"));
        }
        Ok(ok(201, row_to_json(&self.legacy.create(row).await?)))
    }

    async fn put_legacy(
        &self,
        query: &HashMap<String, String>,
        body: Option<&str>,
    ) -> Result<HttpResult, ApiError> {
        let key = required(query, "key")?;
        let by = Where::new(Operator::equals("key", key.clone()));
        if self.legacy.read_single(Some(&by), None, None).await?.is_none() {
            return Ok(empty(404));
        }
        let mut row = text_legacy(json_to_row(&object_from(body).map_err(ApiError::bad)?)?, &self.clock);
        row.insert("key".into(), Value::Text(key));
        self.legacy.update(row, Some(&by)).await?;
        Ok(ok(200, row_to_json(&self.legacy.read_single(Some(&by), None, None).await?.unwrap_or_default())))
    }

    async fn delete_legacy(&self, query: &HashMap<String, String>) -> Result<HttpResult, ApiError> {
        let key = required(query, "key")?;
        let by = Where::new(Operator::equals("key", key));
        let Some(row) = self.legacy.read_single(Some(&by), None, None).await? else {
            return Ok(empty(404));
        };
        self.legacy.delete(row, Some(&by)).await?;
        Ok(empty(204))
    }

    async fn read_rows(
        &self,
        repo: &datasources::CRUDRepository,
        query: &HashMap<String, String>,
        keys: &[&str],
    ) -> Result<Vec<Row>, ApiError> {
        let filter = where_query(query, keys);
        Ok(repo.read(filter.as_ref().map(|item| item as &dyn ReadInput), None, None).try_collect::<Vec<_>>().await?)
    }

    fn stamp(&self, mut json: Json) -> Result<Row, ApiError> {
        let object = json.as_object_mut().ok_or_else(|| ApiError::bad("invalid json"))?;
        stamp_contact(object, &self.clock);
        json_to_row(&json).map_err(ApiError::from)
    }

    fn stamp_group(&self, mut json: Json) -> Result<Row, ApiError> {
        let object = json.as_object_mut().ok_or_else(|| ApiError::bad("invalid json"))?;
        stamp_group(object, &self.clock);
        json_to_row(&json).map_err(ApiError::from)
    }
}

enum ApiError {
    Bad { status: u16, message: String },
    Data(DatasourceError),
}

impl ApiError {
    fn bad(message: impl Into<String>) -> Self {
        Self::Bad {
            status: 400,
            message: message.into(),
        }
    }
}

impl From<DatasourceError> for ApiError {
    fn from(error: DatasourceError) -> Self {
        Self::Data(error)
    }
}

impl From<String> for ApiError {
    fn from(message: String) -> Self {
        Self::bad(message)
    }
}

fn map_error(error: ApiError) -> Result<HttpResult, String> {
    Ok(match error {
        ApiError::Bad { status, message } => HttpResult {
            status,
            body: message,
        },
        ApiError::Data(error) => HttpResult {
            status: 400,
            body: error.to_string(),
        },
    })
}

fn ok(status: u16, body: Json) -> HttpResult {
    HttpResult {
        status,
        body: body.to_string(),
    }
}

fn empty(status: u16) -> HttpResult {
    HttpResult {
        status,
        body: String::new(),
    }
}

fn where_query(query: &HashMap<String, String>, keys: &[&str]) -> Option<Where> {
    let parts: Vec<_> = keys
        .iter()
        .filter_map(|key| {
            let value = query.get(*key).filter(|value| !value.is_empty())?;
            Some(Operator::equals(
                *key,
                if *key == "id" {
                    Value::Int(value.parse().ok()?)
                } else {
                    Value::Text(value.clone())
                },
            ))
        })
        .collect();
    parts.into_iter().reduce(|left, right| Operator::and(left, right)).map(Where::new)
}

fn required(query: &HashMap<String, String>, key: &str) -> Result<String, ApiError> {
    query
        .get(key)
        .filter(|value| !value.is_empty())
        .cloned()
        .ok_or_else(|| ApiError::bad(format!("{key} is required")))
}

fn required_number(query: &HashMap<String, String>, key: &str) -> Result<i64, ApiError> {
    required(query, key)?
        .parse()
        .map_err(|_| ApiError::bad(format!("{key} is required")))
}

fn parse(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter(|part| !part.is_empty())
        .map(|part| match part.split_once('=') {
            Some((key, value)) => (crate::json_row::snake_key(&decode(key)), decode(value)),
            None => (crate::json_row::snake_key(&decode(part)), String::new()),
        })
        .collect()
}

fn decode(text: &str) -> String {
    let text = text.replace('+', " ");
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(value) = u8::from_str_radix(&text[i + 1..i + 3], 16) {
                out.push(value);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn stamp_contact(object: &mut Map<String, Json>, clock: &Clock) {
    set_json_text(object, "email", "");
    set_json_text(object, "notes", "none");
    if object.get("uuid").and_then(Json::as_str).unwrap_or("").is_empty() {
        object.insert("uuid".into(), Json::String(clock.uuid()));
    }
    if object.get("created").and_then(Json::as_str).unwrap_or("").is_empty() {
        object.insert("created".into(), Json::String(clock.now()));
    }
    object.insert("updated".into(), Json::String(clock.now()));
    if object.get("version").and_then(Json::as_str).unwrap_or("").is_empty() {
        object.insert("version".into(), Json::String("AA==".into()));
    }
    stamp_list(object, "addresses", clock, stamp_address);
    stamp_list(object, "phones", clock, stamp_phone);
}

fn stamp_list(
    object: &mut Map<String, Json>,
    key: &str,
    clock: &Clock,
    stamp: fn(&mut Map<String, Json>, &Clock),
) {
    let items = match object.remove(key) {
        Some(Json::Array(items)) => items,
        _ => Vec::new(),
    };
    object.insert(
        key.into(),
        Json::Array(
            items
                .into_iter()
                .map(|item| {
                    let mut map = item.as_object().cloned().unwrap_or_default();
                    stamp(&mut map, clock);
                    Json::Object(map)
                })
                .collect(),
        ),
    );
}

fn stamp_group(object: &mut Map<String, Json>, clock: &Clock) {
    set_json_text(object, "name", "");
    stamp_base(object, clock);
    stamp_list(object, "members", clock, |_, _| {});
}

fn stamp_address(object: &mut Map<String, Json>, clock: &Clock) {
    for key in ["line1", "line2", "city", "region", "postal_code"] {
        set_json_text(object, key, "");
    }
    set_json_text(object, "country", "US");
    stamp_base(object, clock);
}

fn stamp_phone(object: &mut Map<String, Json>, clock: &Clock) {
    set_json_text(object, "number", "");
    set_json_text(object, "label", "Main");
    stamp_base(object, clock);
}

fn stamp_base(object: &mut Map<String, Json>, clock: &Clock) {
    if object.get("uuid").and_then(Json::as_str).unwrap_or("").is_empty() {
        object.insert("uuid".into(), Json::String(clock.uuid()));
    }
    if object.get("created").and_then(Json::as_str).unwrap_or("").is_empty() {
        object.insert("created".into(), Json::String(clock.now()));
    }
    object.insert("updated".into(), Json::String(clock.now()));
    if object.get("version").and_then(Json::as_str).unwrap_or("").is_empty() {
        object.insert("version".into(), Json::String("AA==".into()));
    }
}

fn set_json_text(object: &mut Map<String, Json>, key: &str, fallback: &str) {
    let value = match object.get(key) {
        None | Some(Json::Null) => fallback.to_string(),
        Some(Json::String(text)) => text.clone(),
        Some(other) => other.to_string(),
    };
    object.insert(key.into(), Json::String(value));
}

fn text_legacy(mut row: Row, clock: &Clock) -> Row {
    ensure_text(&mut row, "key");
    ensure_text(&mut row, "first_name");
    ensure_text(&mut row, "last_name");
    ensure_text(&mut row, "email");
    if text(row.get("imported_at")).is_empty() {
        set_text(&mut row, "imported_at", clock.now());
    }
    row
}
