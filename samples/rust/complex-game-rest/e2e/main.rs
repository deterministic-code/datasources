use complex_game_rest::{
    register, Api, Categories, Contact, ContactSource, GameStates, Games, Group, Repositories, Reviews, Role, Users,
};
use datasources::{default, DatasourceError, Result};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
    register();
    let ds = default()?;
    let api = Api::new(
        ds.users(),
        ds.categories(),
        ds.games(),
        ds.reviews(),
        ds.game_states(),
        ds.contacts(),
        complex_game_rest::clock::Clock::new(),
    );
    let role = ds.roles().create(Role { name: "player".into(), description: "plays".into(), ..Default::default() }).await?;
    let group = ds.groups().create(Group { name: "guild".into(), description: "guild".into(), ..Default::default() }).await?;
    ds.contact_sources().create(ContactSource { name: "Manual".into(), ..Default::default() }).await?;

    let user = call(&api, "POST", "/users", Some(json(&[
        ("username", Value::String("ada".into())),
        ("password", Value::String("secret".into())),
        ("settings", object(&[("theme", Value::String("dark".into()))])),
        ("roles", array(&[object(&[("id", role.id.into()), ("name", role.name.clone().into()), ("description", role.description.clone().into())])])),
        ("groups", array(&[object(&[("id", group.id.into()), ("name", group.name.clone().into()), ("description", group.description.clone().into())])])),
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
        ("roles", array(&[object(&[("id", role.id.into()), ("name", role.name.into())])])),
        ("groups", array(&[object(&[("id", group.id.into()), ("name", group.name.into())])])),
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
        ("contact_source_name", Value::String("Manual".into())),
        ("user_id", Value::String(user_id.to_string())),
    ])), 201).await?;
    let contact_id = contact["id"].as_i64().ok_or_else(|| DatasourceError::message("contact id"))?;
    call(&api, "GET", &format!("/contacts?userId={user_id}"), None, 200).await?;
    call(&api, "PUT", &format!("/contacts?id={contact_id}"), Some(json(&[
        ("first_name", Value::String("Ada Byron".into())),
        ("last_name", Value::String("Lovelace".into())),
        ("contact_source_name", Value::String("Manual".into())),
        ("user_id", Value::String(user_id.to_string())),
    ])), 200).await?;

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
    let _ = (Users::default(), Games::default(), Categories::default(), Reviews::default(), GameStates::default(), Contact::default());
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
