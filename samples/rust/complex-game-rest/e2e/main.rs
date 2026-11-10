use complex_game_rest::{
    register, Api, Category, Contact, Game, GameState, Group, Repositories, Review, Role, User,
};
use datasources::{default, DatasourceError, Operator, Result, Where};
use serde_json::Value;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    register();
    let ds = default()?;
    let connection = ds.bind_connection(None);
    let api = Api::open(Arc::clone(&connection), complex_game_rest::clock::Clock::new())?;
    let role = ds.roles().create(Role { name: "player".into(), description: "plays".into(), ..Default::default() }).await?;
    let group = ds.groups().create(Group { name: "guild".into(), description: "guild".into(), ..Default::default() }).await?;
    let source = ds
        .contact_sources()
        .read_single_where(&Where::new(Operator::equals("name", "Manual")), None)
        .await?
        .ok_or_else(|| DatasourceError::message("source"))?;

    let user = call(&api, "POST", "/users", Some(json(&[
        ("username", Value::String("ada".into())),
        ("password", Value::String("secret".into())),
        ("settings", object(&[("theme", Value::String("dark".into()))])),
        ("roles", array(&[object(&[("name", role.name.clone().into()), ("description", role.description.clone().into())])])),
        ("groups", array(&[object(&[("name", group.name.clone().into()), ("description", group.description.clone().into())])])),
    ])), 201).await?;
    let user_id = user["id"].as_i64().ok_or_else(|| DatasourceError::message("user id"))?;
    let listed = call(&api, "GET", "/users?username=ada", None, 200).await?;
    if listed.as_array().map(Vec::len) != Some(1) {
        return Err(DatasourceError::message("get users username"));
    }
    call(&api, "PUT", &format!("/users?id={user_id}"), Some(json(&[
        ("username", Value::String("ada-byron".into())),
        ("password", Value::String("secret".into())),
        ("settings", object(&[("theme", Value::String("sepia".into()))])),
        ("roles", array(&[object(&[("name", role.name.into())])])),
        ("groups", array(&[object(&[("name", group.name.into())])])),
    ])), 200).await?;
    let loaded = call(&api, "GET", &format!("/users?id={user_id}"), None, 200).await?;
    if loaded[0]["username"] != "ada-byron" {
        return Err(DatasourceError::message("put users"));
    }

    let root = call(&api, "POST", "/categories", Some(json(&[("name", Value::String("arcade".into())), ("description", Value::String("root".into()))])), 201).await?;
    let root_id = root["id"].as_i64().ok_or_else(|| DatasourceError::message("root id"))?;
    let child = call(&api, "POST", "/categories", Some(json(&[
        ("name", Value::String("shooter".into())),
        ("description", Value::String("child".into())),
        ("category_id", root_id.into()),
    ])), 201).await?;
    let child_id = child["id"].as_i64().ok_or_else(|| DatasourceError::message("child id"))?;
    let roots = call(&api, "GET", "/categories?category_id=", None, 200).await?;
    if roots.as_array().map(Vec::len) != Some(1) || roots[0]["id"] != root_id {
        return Err(DatasourceError::message("get category roots"));
    }
    let children = call(&api, "GET", &format!("/categories?categoryId={root_id}"), None, 200).await?;
    if children.as_array().map(Vec::len) != Some(1) || children[0]["id"] != child_id {
        return Err(DatasourceError::message("get category children"));
    }

    let game = call(&api, "POST", "/games", Some(json(&[
        ("name", Value::String("star".into())),
        ("description", Value::String("space".into())),
        ("location", Value::String("orbit".into())),
        ("settings", object(&[("lives", Value::String("3".into()))])),
        ("categories", array(&[object(&[("id", child_id.into()), ("name", Value::String("shooter".into()))])])),
    ])), 201).await?;
    let game_id = game["id"].as_i64().ok_or_else(|| DatasourceError::message("game id"))?;
    call(&api, "GET", &format!("/games?id={game_id}"), None, 200).await?;
    call(&api, "PUT", &format!("/games?id={game_id}"), Some(json(&[
        ("name", Value::String("star-2".into())),
        ("description", Value::String("space".into())),
        ("location", Value::String("orbit".into())),
        ("settings", object(&[("lives", Value::String("5".into()))])),
        ("categories", array(&[object(&[("id", child_id.into()), ("name", Value::String("shooter".into()))])])),
    ])), 200).await?;

    let review = call(&api, "POST", "/reviews", Some(json(&[
        ("game_id", game_id.into()),
        ("user_id", user_id.into()),
        ("description", Value::String("fun".into())),
        ("rating", Value::String("4.50".into())),
        ("is_approved", Value::Bool(true)),
    ])), 201).await?;
    let review_id = review["id"].as_i64().ok_or_else(|| DatasourceError::message("review id"))?;
    call(&api, "GET", &format!("/reviews?gameId={game_id}&userId={user_id}"), None, 200).await?;
    call(&api, "PUT", &format!("/reviews?id={review_id}"), Some(json(&[
        ("game_id", game_id.into()),
        ("user_id", user_id.into()),
        ("description", Value::String("great".into())),
        ("rating", Value::String("5.00".into())),
        ("is_approved", Value::Bool(true)),
    ])), 200).await?;

    let state = call(&api, "POST", "/game_states", Some(json(&[
        ("game_id", game_id.into()),
        ("user_id", user_id.into()),
        ("state", Value::String("intro".into())),
    ])), 201).await?;
    let state_id = state["id"].as_i64().ok_or_else(|| DatasourceError::message("state id"))?;
    call(&api, "GET", &format!("/game_states?gameId={game_id}&userId={user_id}"), None, 200).await?;
    call(&api, "PUT", &format!("/game_states?id={state_id}"), Some(json(&[
        ("game_id", game_id.into()),
        ("user_id", user_id.into()),
        ("state", Value::String("boss".into())),
    ])), 200).await?;

    let contact = call(&api, "POST", "/contacts", Some(json(&[
        ("first_name", Value::String("Ada".into())),
        ("last_name", Value::String("Lovelace".into())),
        ("contact_source_name", Value::String(source.name.clone())),
        ("user_id", Value::String(user_id.to_string())),
        ("addresses", array(&[
            object(&[
                ("address_type_name", Value::String("Home".into())),
                ("line1", Value::String("12 Analytical Engine".into())),
                ("city", Value::String("London".into())),
                ("country", Value::String("GB".into())),
            ]),
            object(&[
                ("address_type_name", Value::String("Work".into())),
                ("line1", Value::String("1 Strand".into())),
                ("city", Value::String("London".into())),
                ("country", Value::String("GB".into())),
            ]),
        ])),
        ("phones", array(&[object(&[
            ("phone_type_name", Value::String("Mobile".into())),
            ("number", Value::String("555-0100".into())),
        ])])),
    ])), 201).await?;
    let contact_id = contact["id"].as_i64().ok_or_else(|| DatasourceError::message("contact id"))?;
    let created = call(&api, "GET", &format!("/contacts?id={contact_id}"), None, 200).await?;
    require_ada_children(&created, "Home", "London", "555-0100")?;
    if address_by_line1(contact_row(&created)?, "1 Strand")?["address_type_name"] != "Work" {
        return Err(DatasourceError::message("create work address"));
    }
    let engine_id = address_by_line1(contact_row(&created)?, "12 Analytical Engine")?["id"].clone();
    let phone_id = contact_row(&created)?["phones"][0]["id"].clone();
    call(&api, "PUT", &format!("/contacts?id={contact_id}"), Some(json(&[
        ("first_name", Value::String("Ada Byron".into())),
        ("last_name", Value::String("Lovelace".into())),
        ("contact_source_name", Value::String(source.name.clone())),
        ("user_id", Value::String(user_id.to_string())),
        ("addresses", array(&[
            object(&[
                ("id", engine_id),
                ("address_type_name", Value::String("Work".into())),
                ("line1", Value::String("12 Analytical Engine".into())),
                ("city", Value::String("Cambridge".into())),
                ("country", Value::String("FR".into())),
            ]),
            object(&[
                ("address_type_name", Value::String("Home".into())),
                ("line1", Value::String("1 College".into())),
                ("city", Value::String("Cambridge".into())),
                ("country", Value::String("GB".into())),
            ]),
        ])),
        ("phones", array(&[object(&[
            ("id", phone_id),
            ("phone_type_name", Value::String("Mobile".into())),
            ("number", Value::String("555-0111".into())),
        ])])),
    ])), 200).await?;
    let updated = call(&api, "GET", &format!("/contacts?id={contact_id}"), None, 200).await?;
    require_ada_children(&updated, "Work", "Cambridge", "555-0111")?;
    if address_by_line1(contact_row(&updated)?, "1 College")?["address_type_name"] != "Home" {
        return Err(DatasourceError::message("update home address"));
    }

    for target in [
        format!("/contacts?id={contact_id}"),
        format!("/reviews?id={review_id}"),
        format!("/game_states?id={state_id}"),
        format!("/games?id={game_id}"),
        format!("/categories?id={child_id}"),
        format!("/categories?id={root_id}"),
        format!("/users?id={user_id}"),
    ] {
        call(&api, "DELETE", &target, None, 204).await?;
        call(&api, "DELETE", &target, None, 404).await?;
        call(&api, "PUT", &target, Some("{}".into()), 404).await?;
    }
    let _ = (User::default(), Game::default(), Category::default(), Review::default(), GameState::default(), Contact::default());
    Ok(())
}

fn contact_row(listed: &Value) -> Result<&Value> {
    listed
        .as_array()
        .and_then(|rows| rows.first())
        .ok_or_else(|| DatasourceError::message("contact list"))
}

fn address_by_line1<'a>(contact: &'a Value, line1: &str) -> Result<&'a Value> {
    contact["addresses"]
        .as_array()
        .and_then(|rows| rows.iter().find(|address| address["line1"] == line1))
        .ok_or_else(|| DatasourceError::message(line1.to_string()))
}

fn require_ada_children(listed: &Value, engine_type: &str, engine_city: &str, number: &str) -> Result<()> {
    let contact = contact_row(listed)?;
    let addresses = contact["addresses"]
        .as_array()
        .ok_or_else(|| DatasourceError::message("addresses"))?;
    let phones = contact["phones"]
        .as_array()
        .ok_or_else(|| DatasourceError::message("phones"))?;
    if addresses.len() != 2 || phones.len() != 1 {
        return Err(DatasourceError::message("child counts"));
    }
    let engine = address_by_line1(contact, "12 Analytical Engine")?;
    if engine["address_type_name"] != engine_type || engine["city"] != engine_city {
        return Err(DatasourceError::message("engine address"));
    }
    if engine["contact_id"].as_i64() != contact["id"].as_i64() {
        return Err(DatasourceError::message("address contact id"));
    }
    if phones[0]["number"] != number || phones[0]["phone_type_name"] != "Mobile" {
        return Err(DatasourceError::message("phone"));
    }
    if phones[0]["contact_id"].as_i64() != contact["id"].as_i64() {
        return Err(DatasourceError::message("phone contact id"));
    }
    Ok(())
}

async fn call(api: &Api, method: &str, target: &str, body: Option<String>, status: u16) -> Result<Value> {
    let result = api.handle(method, target, body.as_deref()).await.map_err(DatasourceError::message)?;
    if result.status != status {
        return Err(DatasourceError::message(format!("{method} {target} => {} {}", result.status, result.body)));
    }
    if result.body.is_empty() {
        return Ok(Value::Null);
    }
    serde_json::from_str(&result.body).map_err(|error| DatasourceError::message(error.to_string()))
}

fn json(fields: &[(&str, Value)]) -> String {
    Value::Object(fields.iter().map(|(key, value)| ((*key).to_string(), value.clone())).collect()).to_string()
}

fn object(fields: &[(&str, Value)]) -> Value {
    Value::Object(fields.iter().map(|(key, value)| ((*key).to_string(), value.clone())).collect())
}

fn array(items: &[Value]) -> Value {
    Value::Array(items.to_vec())
}
