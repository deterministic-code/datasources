use crate::clock::Clock;
use crate::json_object::JsonObjectSerializer;
use crate::json_row::snake_key;
use crate::repositories::contact_group_repository::ContactGroupRepository;
use crate::repositories::contact_repository::ContactRepository;
use crate::{Address, Contact, ContactGroup, LegacyContact, Phone};
use chrono::{DateTime, Utc};
use datasources::{DatasourceError, Operator, ReadInput, Value, Where, CRUDRepository};
use futures_util::TryStreamExt;
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
        Ok(ok(
            200,
            &self
                .list(&*self.contacts, where_query(query, &["id", "first_name", "last_name", "email"]))
                .await?,
        )?)
    }

    async fn post_contact(&self, body: Option<&str>) -> Result<HttpResult, ApiError> {
        let contact = stamp(read(body)?, &self.clock);
        Ok(ok(201, &self.contacts.create(contact).await?)?)
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
        let mut contact = stamp(read(body)?, &self.clock);
        contact.id = i32::try_from(id).map_err(|_| ApiError::bad("id is required"))?;
        self.contacts.update(contact).await?;
        Ok(ok(200, &self.contacts.read_single(&by).await?.unwrap_or_default())?)
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
        Ok(ok(200, &self.list(&*self.groups_repo, where_query(query, &["id"])).await?)?)
    }

    async fn post_group(&self, body: Option<&str>) -> Result<HttpResult, ApiError> {
        let group = stamp_group(read(body)?, &self.clock);
        Ok(ok(201, &self.groups_repo.create(group).await?)?)
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
        let mut group = stamp_group(read(body)?, &self.clock);
        group.id = i32::try_from(id).map_err(|_| ApiError::bad("id is required"))?;
        self.groups_repo.update(group).await?;
        Ok(ok(200, &self.groups_repo.read_single(&by).await?.unwrap_or_default())?)
    }

    async fn get_legacy(&self, query: &HashMap<String, String>) -> Result<HttpResult, ApiError> {
        Ok(ok(
            200,
            &self
                .list(&self.legacy, where_query(query, &["key", "first_name", "last_name", "email"]))
                .await?,
        )?)
    }

    async fn post_legacy(&self, body: Option<&str>) -> Result<HttpResult, ApiError> {
        let row = text_legacy(read(body)?, &self.clock);
        if row.key.is_empty() {
            return Err(ApiError::bad("key is required"));
        }
        Ok(ok(201, &self.legacy.create(row).await?)?)
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
        let mut row = text_legacy(read(body)?, &self.clock);
        row.key = key;
        self.legacy.update(row).await?;
        Ok(ok(200, &self.legacy.read_single(&by).await?.unwrap_or_default())?)
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

    async fn list<T: datasources::Entity>(
        &self,
        repo: &CRUDRepository<T>,
        filter: Option<Where>,
    ) -> Result<Vec<T>, ApiError> {
        let none = ();
        let query: &dyn ReadInput = filter
            .as_ref()
            .map(|item| item as &dyn ReadInput)
            .unwrap_or(&none);
        Ok(repo.read(query).try_collect::<Vec<_>>().await?)
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

fn ok<T: datasources::Facet<'static>>(status: u16, value: &T) -> Result<HttpResult, ApiError> {
    Ok(HttpResult {
        status,
        body: JsonObjectSerializer::to_string(value)?,
    })
}

fn empty(status: u16) -> HttpResult {
    HttpResult {
        status,
        body: String::new(),
    }
}

fn read<T: datasources::Facet<'static> + Default>(body: Option<&str>) -> Result<T, ApiError> {
    JsonObjectSerializer::from_str(body.unwrap_or("")).map_err(ApiError::from)
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
            Some((key, value)) => (snake_key(&decode(key)), decode(value)),
            None => (snake_key(&decode(part)), String::new()),
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

fn stamp(mut contact: Contact, clock: &Clock) -> Contact {
    let now = date(clock);
    contact.email = Some(contact.email.unwrap_or_default());
    contact.notes = Some(if contact.notes.as_deref().unwrap_or("").is_empty() {
        "none".into()
    } else {
        contact.notes.unwrap_or_default()
    });
    if contact.uuid.is_empty() {
        contact.uuid = clock.uuid();
    }
    if contact.created == DateTime::<Utc>::default() {
        contact.created = now;
    }
    contact.updated = now;
    if contact.version.is_empty() {
        contact.version = vec![0];
    }
    contact.addresses = contact.addresses.into_iter().map(|address| stamp_address(address, clock)).collect();
    contact.phones = contact.phones.into_iter().map(|phone| stamp_phone(phone, clock)).collect();
    contact
}

fn stamp_group(mut group: ContactGroup, clock: &Clock) -> ContactGroup {
    let now = date(clock);
    if group.uuid.is_empty() {
        group.uuid = clock.uuid();
    }
    if group.created == DateTime::<Utc>::default() {
        group.created = now;
    }
    group.updated = now;
    if group.version.is_empty() {
        group.version = vec![0];
    }
    group
}

fn stamp_address(mut address: Address, clock: &Clock) -> Address {
    let now = date(clock);
    if address.country.as_deref().unwrap_or("").is_empty() {
        address.country = Some("US".into());
    }
    if address.uuid.is_empty() {
        address.uuid = clock.uuid();
    }
    if address.created == DateTime::<Utc>::default() {
        address.created = now;
    }
    address.updated = now;
    if address.version.is_empty() {
        address.version = vec![0];
    }
    address
}

fn stamp_phone(mut phone: Phone, clock: &Clock) -> Phone {
    let now = date(clock);
    if phone.label.as_deref().unwrap_or("").is_empty() {
        phone.label = Some("Main".into());
    }
    if phone.uuid.is_empty() {
        phone.uuid = clock.uuid();
    }
    if phone.created == DateTime::<Utc>::default() {
        phone.created = now;
    }
    phone.updated = now;
    if phone.version.is_empty() {
        phone.version = vec![0];
    }
    phone
}

fn text_legacy(mut row: LegacyContact, clock: &Clock) -> LegacyContact {
    row.email = Some(row.email.unwrap_or_default());
    if row.imported_at.is_none() {
        row.imported_at = Some(date(clock));
    }
    row
}

fn date(clock: &Clock) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(&clock.now())
        .map(|date| date.with_timezone(&Utc))
        .unwrap_or_default()
}
