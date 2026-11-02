use crate::clock::Clock;
use crate::generated::repositories::contact_group_repository::ContactGroupRepository;
use crate::generated::repositories::contact_repository::ContactRepository;
use crate::generated::{Address, Contact, ContactGroup, LegacyContact, Phone};
use crate::json_row::{json_to_row, object_from, row_to_json, rows_to_json};
use datasources::{
    chrono::{DateTime, Utc}, DatasourceError, FromSchema, Operator, ReadInput, Schema, Value, Where,
    CRUDRepository, IReadonlyRepository,
};
use futures_util::TryStreamExt;
use serde_json::{Map, Value as Json};
use std::collections::HashMap;

pub struct HttpResult {
    pub status: u16,
    pub body: String,
}

pub struct Api {
    contacts: ContactRepository,
    legacy: CRUDRepository<LegacyContact>,
    groups_repo: ContactGroupRepository,
    clock: Clock,
}

impl Api {
    pub fn new(
        contacts: ContactRepository,
        legacy: CRUDRepository<LegacyContact>,
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
        Ok(ok(200, self.list_json(&*self.contacts, self.contacts.schema(), query, &["id", "first_name", "last_name", "email"]).await?))
    }

    async fn post_contact(&self, body: Option<&str>) -> Result<HttpResult, ApiError> {
        let contact = self.read_contact(body)?;
        Ok(ok(201, self.entity_json(&self.contacts.create(contact).await?, self.contacts.schema())?))
    }

    async fn put_contact(
        &self,
        query: &HashMap<String, String>,
        body: Option<&str>,
    ) -> Result<HttpResult, ApiError> {
        let id = required_number(query, "id")?;
        let by = Where::new(Operator::equals("id", id));
        if self.contacts.read_single(&by).await?.is_none() {
            return Ok(empty(404));
        }
        let mut contact = self.read_contact(body)?;
        contact.id = id as i32;
        self.contacts.update(contact).await?;
        Ok(ok(200, self.entity_json(&self.contacts.read_single(&by).await?.unwrap_or_default(), self.contacts.schema())?))
    }

    async fn delete_contact(&self, query: &HashMap<String, String>) -> Result<HttpResult, ApiError> {
        let id = required_number(query, "id")?;
        let by = Where::new(Operator::equals("id", id));
        let Some(contact) = self.contacts.read_single(&by).await? else {
            return Ok(empty(404));
        };
        self.contacts.delete(contact).await?;
        Ok(empty(204))
    }

    async fn get_groups(&self, query: &HashMap<String, String>) -> Result<HttpResult, ApiError> {
        Ok(ok(200, self.list_json(&*self.groups_repo, self.groups_repo.schema(), query, &["id"]).await?))
    }

    async fn post_group(&self, body: Option<&str>) -> Result<HttpResult, ApiError> {
        let group = self.read_group(body)?;
        Ok(ok(201, self.entity_json(&self.groups_repo.create(group).await?, self.groups_repo.schema())?))
    }

    async fn put_group(
        &self,
        query: &HashMap<String, String>,
        body: Option<&str>,
    ) -> Result<HttpResult, ApiError> {
        let id = required_number(query, "id")?;
        let by = Where::new(Operator::equals("id", id));
        if self.groups_repo.read_single(&by).await?.is_none() {
            return Ok(empty(404));
        }
        let mut group = self.read_group(body)?;
        group.id = id as i32;
        self.groups_repo.update(group).await?;
        Ok(ok(200, self.entity_json(&self.groups_repo.read_single(&by).await?.unwrap_or_default(), self.groups_repo.schema())?))
    }

    async fn get_legacy(&self, query: &HashMap<String, String>) -> Result<HttpResult, ApiError> {
        Ok(ok(200, self.list_json(&self.legacy, self.legacy.schema(), query, &["key", "first_name", "last_name", "email"]).await?))
    }

    async fn post_legacy(&self, body: Option<&str>) -> Result<HttpResult, ApiError> {
        let row = text_legacy(self.read_entity::<LegacyContact>(body, self.legacy.schema())?, &self.clock);
        if row.key.is_empty() {
            return Err(ApiError::bad("key is required"));
        }
        Ok(ok(201, self.entity_json(&self.legacy.create(row).await?, self.legacy.schema())?))
    }

    async fn put_legacy(
        &self,
        query: &HashMap<String, String>,
        body: Option<&str>,
    ) -> Result<HttpResult, ApiError> {
        let key = required(query, "key")?;
        let by = Where::new(Operator::equals("key", key.clone()));
        if self.legacy.read_single(&by).await?.is_none() {
            return Ok(empty(404));
        }
        let mut row = text_legacy(self.read_entity::<LegacyContact>(body, self.legacy.schema())?, &self.clock);
        row.key = key;
        self.legacy.update(row).await?;
        Ok(ok(200, self.entity_json(&self.legacy.read_single(&by).await?.unwrap_or_default(), self.legacy.schema())?))
    }

    async fn delete_legacy(&self, query: &HashMap<String, String>) -> Result<HttpResult, ApiError> {
        let key = required(query, "key")?;
        let by = Where::new(Operator::equals("key", key));
        let Some(row) = self.legacy.read_single(&by).await? else {
            return Ok(empty(404));
        };
        self.legacy.delete(row).await?;
        Ok(empty(204))
    }

    async fn list_json<T: FromSchema>(
        &self,
        repo: &impl IReadonlyRepository<T>,
        schema: &Schema,
        query: &HashMap<String, String>,
        keys: &[&str],
    ) -> Result<Json, ApiError> {
        let items = self.read_items(repo, query, keys).await?;
        let mut rows = Vec::new();
        for item in items {
            rows.push(item.to_schema(schema)?);
        }
        Ok(rows_to_json(&rows))
    }

    async fn read_items<T: FromSchema>(
        &self,
        repo: &impl IReadonlyRepository<T>,
        query: &HashMap<String, String>,
        keys: &[&str],
    ) -> Result<Vec<T>, ApiError> {
        let filter = where_query(query, keys);
        let none = ();
        let query: &dyn ReadInput = filter
            .as_ref()
            .map(|item| item as &dyn ReadInput)
            .unwrap_or(&none);
        Ok(repo.read(query).try_collect::<Vec<_>>().await?)
    }

    fn read_contact(&self, body: Option<&str>) -> Result<Contact, ApiError> {
        let mut json = object_from(body).map_err(ApiError::bad)?;
        let object = json.as_object_mut().ok_or_else(|| ApiError::bad("invalid json"))?;
        stamp_contact_json(object, &self.clock);
        let mut contact = Contact::from_schema(json_to_row(&json).map_err(ApiError::bad)?, self.contacts.schema())?;
        stamp_contact(&mut contact, &self.clock);
        Ok(contact)
    }

    fn read_group(&self, body: Option<&str>) -> Result<ContactGroup, ApiError> {
        let mut json = object_from(body).map_err(ApiError::bad)?;
        let object = json.as_object_mut().ok_or_else(|| ApiError::bad("invalid json"))?;
        stamp_group_json(object, &self.clock);
        let mut group = ContactGroup::from_schema(json_to_row(&json).map_err(ApiError::bad)?, self.groups_repo.schema())?;
        stamp_group(&mut group, &self.clock);
        Ok(group)
    }

    fn read_entity<T: FromSchema>(&self, body: Option<&str>, schema: &Schema) -> Result<T, ApiError> {
        let json = object_from(body).map_err(ApiError::bad)?;
        T::from_schema(json_to_row(&json).map_err(ApiError::bad)?, schema).map_err(ApiError::from)
    }

    fn entity_json<T: FromSchema>(&self, item: &T, schema: &Schema) -> Result<Json, ApiError> {
        Ok(row_to_json(&item.to_schema(schema)?))
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

fn stamp_contact(contact: &mut Contact, clock: &Clock) {
    if contact.email.as_deref().unwrap_or("").is_empty() {
        contact.email = Some(String::new());
    }
    if contact.notes.as_deref().unwrap_or("").is_empty() {
        contact.notes = Some("none".into());
    }
    stamp_base(&mut contact.uuid, &mut contact.created, &mut contact.updated, &mut contact.version, clock);
    for address in &mut contact.addresses {
        stamp_address(address, clock);
    }
    for phone in &mut contact.phones {
        stamp_phone(phone, clock);
    }
}

fn stamp_group(group: &mut ContactGroup, clock: &Clock) {
    stamp_base(&mut group.uuid, &mut group.created, &mut group.updated, &mut group.version, clock);
}

fn stamp_address(address: &mut Address, clock: &Clock) {
    if address.line2.as_deref().unwrap_or("").is_empty() {
        address.line2 = Some(String::new());
    }
    if address.region.as_deref().unwrap_or("").is_empty() {
        address.region = Some(String::new());
    }
    if address.postal_code.as_deref().unwrap_or("").is_empty() {
        address.postal_code = Some(String::new());
    }
    if address.country.as_deref().unwrap_or("").is_empty() {
        address.country = Some("US".into());
    }
    stamp_base(&mut address.uuid, &mut address.created, &mut address.updated, &mut address.version, clock);
}

fn stamp_phone(phone: &mut Phone, clock: &Clock) {
    if phone.label.as_deref().unwrap_or("").is_empty() {
        phone.label = Some("Main".into());
    }
    stamp_base(&mut phone.uuid, &mut phone.created, &mut phone.updated, &mut phone.version, clock);
}

fn stamp_base(
    uuid: &mut String,
    created: &mut DateTime<Utc>,
    updated: &mut DateTime<Utc>,
    version: &mut Vec<u8>,
    clock: &Clock,
) {
    if uuid.is_empty() {
        *uuid = clock.uuid();
    }
    let now = parse_clock(clock);
    if *created == DateTime::<Utc>::UNIX_EPOCH {
        *created = now;
    }
    *updated = now;
    if version.is_empty() {
        *version = vec![0];
    }
}

fn stamp_contact_json(object: &mut Map<String, Json>, clock: &Clock) {
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
    stamp_list(object, "addresses", clock, stamp_address_json);
    stamp_list(object, "phones", clock, stamp_phone_json);
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

fn stamp_group_json(object: &mut Map<String, Json>, clock: &Clock) {
    set_json_text(object, "name", "");
    stamp_base_json(object, clock);
    stamp_list(object, "members", clock, |_, _| {});
}

fn stamp_address_json(object: &mut Map<String, Json>, clock: &Clock) {
    for key in ["line1", "line2", "city", "region", "postal_code"] {
        set_json_text(object, key, "");
    }
    set_json_text(object, "country", "US");
    stamp_base_json(object, clock);
}

fn stamp_phone_json(object: &mut Map<String, Json>, clock: &Clock) {
    set_json_text(object, "number", "");
    set_json_text(object, "label", "Main");
    stamp_base_json(object, clock);
}

fn stamp_base_json(object: &mut Map<String, Json>, clock: &Clock) {
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

fn parse_clock(clock: &Clock) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(&clock.now())
        .map(|value| value.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

fn text_legacy(mut row: LegacyContact, clock: &Clock) -> LegacyContact {
    if row.imported_at.is_none() {
        row.imported_at = Some(parse_clock(clock));
    }
    row
}
