use crate::clock::Clock;
use crate::json_object::JsonObjectSerializer;
use crate::{Categories, Contact, GameStates, Games, Reviews, Users};
use chrono::{DateTime, Utc};
use datasources::{
    DatasourceError, Expression, Identifier, Operator, ReadInput, Value, Where, CRUDRepository,
};
use futures_util::TryStreamExt;
use std::collections::HashMap;
use std::sync::Arc;

pub struct HttpResult {
    pub status: u16,
    pub body: String,
}

pub struct Api {
    users: Arc<CRUDRepository<Users>>,
    categories: Arc<CRUDRepository<Categories>>,
    games: Arc<CRUDRepository<Games>>,
    reviews: Arc<CRUDRepository<Reviews>>,
    states: Arc<CRUDRepository<GameStates>>,
    contacts: Arc<CRUDRepository<Contact>>,
    clock: Clock,
}

impl Api {
    pub fn new(
        users: Arc<CRUDRepository<Users>>,
        categories: Arc<CRUDRepository<Categories>>,
        games: Arc<CRUDRepository<Games>>,
        reviews: Arc<CRUDRepository<Reviews>>,
        states: Arc<CRUDRepository<GameStates>>,
        contacts: Arc<CRUDRepository<Contact>>,
        clock: Clock,
    ) -> Self {
        Self { users, categories, games, reviews, states, contacts, clock }
    }

    pub async fn handle(&self, method: &str, target: &str, body: Option<&str>) -> Result<HttpResult, String> {
        self.dispatch(method, target, body).await.or_else(map_error)
    }

    async fn dispatch(&self, method: &str, target: &str, body: Option<&str>) -> Result<HttpResult, ApiError> {
        let split = target.find('?');
        let raw = split.map(|i| &target[..i]).unwrap_or(target);
        let path = raw.trim_end_matches('/');
        let path = if path.is_empty() { "/" } else { path };
        let query = parse(split.map(|i| &target[i + 1..]).unwrap_or(""));
        match (method.to_ascii_uppercase().as_str(), path) {
            ("GET", "/users") => get(&self.users, &query, &["id", "username"]).await,
            ("POST", "/users") => post(&self.users, stamp_user(read(body)?, &self.clock)).await,
            ("PUT", "/users") => {
                put(&self.users, &query, stamp_user(read(body)?, &self.clock), |row, id| row.id = id).await
            }
            ("DELETE", "/users") => delete(&self.users, &query).await,
            ("GET", "/categories") => self.get_categories(&query).await,
            ("POST", "/categories") => post(&self.categories, stamp_category(read(body)?, &self.clock)).await,
            ("PUT", "/categories") => {
                put(&self.categories, &query, stamp_category(read(body)?, &self.clock), |row, id| row.id = id).await
            }
            ("DELETE", "/categories") => delete(&self.categories, &query).await,
            ("GET", "/games") => get(&self.games, &query, &["id", "name"]).await,
            ("POST", "/games") => post(&self.games, stamp_game(read(body)?, &self.clock)).await,
            ("PUT", "/games") => {
                put(&self.games, &query, stamp_game(read(body)?, &self.clock), |row, id| row.id = id).await
            }
            ("DELETE", "/games") => delete(&self.games, &query).await,
            ("GET", "/reviews") => get(&self.reviews, &query, &["id", "game_id", "user_id"]).await,
            ("POST", "/reviews") => post(&self.reviews, stamp_review(read(body)?, &self.clock)).await,
            ("PUT", "/reviews") => {
                put(&self.reviews, &query, stamp_review(read(body)?, &self.clock), |row, id| row.id = id).await
            }
            ("DELETE", "/reviews") => delete(&self.reviews, &query).await,
            ("GET", "/game_states") => get(&self.states, &query, &["id", "game_id", "user_id"]).await,
            ("POST", "/game_states") => post(&self.states, stamp_state(read(body)?, &self.clock)).await,
            ("PUT", "/game_states") => {
                put(&self.states, &query, stamp_state(read(body)?, &self.clock), |row, id| row.id = id).await
            }
            ("DELETE", "/game_states") => delete(&self.states, &query).await,
            ("GET", "/contacts") => get(&self.contacts, &query, &["id", "user_id"]).await,
            ("POST", "/contacts") => post(&self.contacts, stamp_contact(read(body)?, &self.clock)).await,
            ("PUT", "/contacts") => {
                put(&self.contacts, &query, stamp_contact(read(body)?, &self.clock), |row, id| row.id = id).await
            }
            ("DELETE", "/contacts") => delete(&self.contacts, &query).await,
            _ => Ok(HttpResult { status: 404, body: String::new() }),
        }
    }

    async fn get_categories(&self, query: &HashMap<String, String>) -> Result<HttpResult, ApiError> {
        let raw = query.get("category_id").cloned().unwrap_or_default();
        let filter = if raw.is_empty() {
            Where::new(Operator::is_null(Expression::Identifier(Identifier::new("category_id"))))
        } else {
            let id: i64 = raw.parse().map_err(|_| ApiError::bad("categoryId is required"))?;
            Where::new(Operator::equals("category_id", id))
        };
        Ok(ok(200, &list(&self.categories, Some(filter)).await?)?)
    }
}

enum ApiError {
    Bad { status: u16, message: String },
    Data(DatasourceError),
}

impl ApiError {
    fn bad(message: impl Into<String>) -> Self {
        Self::Bad { status: 400, message: message.into() }
    }
}

impl From<DatasourceError> for ApiError {
    fn from(error: DatasourceError) -> Self {
        Self::Data(error)
    }
}

fn map_error(error: ApiError) -> Result<HttpResult, String> {
    Ok(match error {
        ApiError::Bad { status, message } => HttpResult { status, body: message },
        ApiError::Data(error) => HttpResult { status: 400, body: error.to_string() },
    })
}

fn ok<T: datasources::Facet<'static>>(status: u16, value: &T) -> Result<HttpResult, ApiError> {
    Ok(HttpResult { status, body: JsonObjectSerializer::to_string(value)? })
}

fn empty(status: u16) -> HttpResult {
    HttpResult { status, body: String::new() }
}

fn read<T: datasources::Facet<'static> + Default>(body: Option<&str>) -> Result<T, ApiError> {
    JsonObjectSerializer::from_str(body.unwrap_or("")).map_err(ApiError::from)
}

async fn get<T: datasources::Entity + datasources::Facet<'static>>(
    repo: &CRUDRepository<T>,
    query: &HashMap<String, String>,
    keys: &[&str],
) -> Result<HttpResult, ApiError> {
    Ok(ok(200, &list(repo, where_query(query, keys)).await?)?)
}

async fn post<T: datasources::Entity + datasources::Facet<'static>>(repo: &CRUDRepository<T>, row: T) -> Result<HttpResult, ApiError> {
    Ok(ok(201, &repo.create(row).await?)?)
}

async fn put<T: datasources::Entity + datasources::Facet<'static>>(
    repo: &CRUDRepository<T>,
    query: &HashMap<String, String>,
    mut row: T,
    set_id: impl FnOnce(&mut T, i64),
) -> Result<HttpResult, ApiError> {
    let id = required_number(query, "id")?;
    let by = Where::new(Operator::equals("id", id));
    if repo.read_single(&by).await?.is_none() {
        return Ok(empty(404));
    }
    set_id(&mut row, id);
    repo.update(row).await?;
    Ok(ok(200, &repo.read_single(&by).await?.unwrap_or_default())?)
}

async fn delete<T: datasources::Entity>(repo: &CRUDRepository<T>, query: &HashMap<String, String>) -> Result<HttpResult, ApiError> {
    let id = required_number(query, "id")?;
    let by = Where::new(Operator::equals("id", id));
    let Some(row) = repo.read_single(&by).await? else {
        return Ok(empty(404));
    };
    repo.delete(row).await?;
    Ok(empty(204))
}

async fn list<T: datasources::Entity>(repo: &CRUDRepository<T>, filter: Option<Where>) -> Result<Vec<T>, ApiError> {
    let none = ();
    let query: &dyn ReadInput = filter.as_ref().map(|item| item as &dyn ReadInput).unwrap_or(&none);
    Ok(repo.read(query).try_collect::<Vec<_>>().await?)
}

fn where_query(query: &HashMap<String, String>, keys: &[&str]) -> Option<Where> {
    let parts: Vec<_> = keys
        .iter()
        .filter_map(|key| {
            let value = query.get(*key).filter(|value| !value.is_empty())?;
            Some(Operator::equals(
                *key,
                if number_key(key) {
                    Value::Int(value.parse().ok()?)
                } else {
                    Value::Text(value.clone())
                },
            ))
        })
        .collect();
    parts.into_iter().reduce(Operator::and).map(Where::new)
}

fn number_key(key: &str) -> bool {
    matches!(key, "id" | "user_id" | "game_id" | "category_id")
}

fn required(query: &HashMap<String, String>, key: &str) -> Result<String, ApiError> {
    query
        .get(key)
        .filter(|value| !value.is_empty())
        .cloned()
        .ok_or_else(|| ApiError::bad(format!("{key} is required")))
}

fn required_number(query: &HashMap<String, String>, key: &str) -> Result<i64, ApiError> {
    required(query, key)?.parse().map_err(|_| ApiError::bad(format!("{key} is required")))
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

fn snake_key(key: &str) -> String {
    let mut out = String::new();
    for (index, ch) in key.chars().enumerate() {
        if ch.is_uppercase() {
            if index > 0 {
                out.push('_');
            }
            out.extend(ch.to_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
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

fn stamp_user(mut user: Users, clock: &Clock) -> Users {
    stamp(&mut user.uuid, &mut user.created, &mut user.updated, &mut user.version, clock);
    for role in &mut user.roles {
        stamp(&mut role.uuid, &mut role.created, &mut role.updated, &mut role.version, clock);
    }
    for group in &mut user.groups {
        stamp(&mut group.uuid, &mut group.created, &mut group.updated, &mut group.version, clock);
    }
    user
}

fn stamp_category(mut category: Categories, clock: &Clock) -> Categories {
    stamp(&mut category.uuid, &mut category.created, &mut category.updated, &mut category.version, clock);
    category
}

fn stamp_game(mut game: Games, clock: &Clock) -> Games {
    stamp(&mut game.uuid, &mut game.created, &mut game.updated, &mut game.version, clock);
    for category in &mut game.categories {
        stamp(&mut category.uuid, &mut category.created, &mut category.updated, &mut category.version, clock);
    }
    game
}

fn stamp_review(mut review: Reviews, clock: &Clock) -> Reviews {
    stamp(&mut review.uuid, &mut review.created, &mut review.updated, &mut review.version, clock);
    review
}

fn stamp_state(mut state: GameStates, clock: &Clock) -> GameStates {
    stamp(&mut state.uuid, &mut state.created, &mut state.updated, &mut state.version, clock);
    state
}

fn stamp_contact(mut contact: Contact, clock: &Clock) -> Contact {
    if contact.email.as_deref().unwrap_or("").is_empty() {
        contact.email = Some(String::new());
    }
    if contact.notes.as_deref().unwrap_or("").is_empty() {
        contact.notes = Some("none".into());
    }
    stamp(&mut contact.uuid, &mut contact.created, &mut contact.updated, &mut contact.version, clock);
    contact
}

fn stamp(
    uuid: &mut String,
    created: &mut DateTime<Utc>,
    updated: &mut DateTime<Utc>,
    version: &mut Vec<u8>,
    clock: &Clock,
) {
    if uuid.is_empty() {
        *uuid = clock.uuid();
    }
    if *created == DateTime::<Utc>::default() {
        *created = date(clock);
    }
    *updated = date(clock);
    if version.is_empty() {
        *version = vec![0];
    }
}

fn date(clock: &Clock) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(&clock.now())
        .map(|date| date.with_timezone(&Utc))
        .unwrap_or_default()
}
