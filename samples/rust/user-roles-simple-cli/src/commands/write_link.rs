use crate::app::App;
use crate::commands::{lookup_id, write_row};
use crate::schema;
use datasources::{Parameter, Result};

pub async fn execute(
    app: &App<'_>,
    username: &str,
    right_table: &str,
    right_column: &str,
    right_value: &str,
    link_table: &str,
    link_column: &str,
    procedure: &str,
) -> Result<String> {
    let users = schema::ident(app.provider(), "users");
    let user_id = lookup_id::of(app.command(), &users, "username", username).await?;
    let right = schema::ident(app.provider(), right_table);
    let right_id = lookup_id::of(app.command(), &right, right_column, right_value).await?;
    let created = app.stamp();
    let updated = app.token_bytes();
    let insert = [
        Parameter::new("user_id", user_id),
        Parameter::new(link_column, right_id),
        Parameter::new("created", created.as_str()),
        Parameter::new("updated", updated.clone()),
    ];
    let call = [
        Parameter::new("p_user_id", user_id),
        Parameter::new(
            if link_column == "bad_role_id" {
                "p_role_id"
            } else {
                "p_group_id"
            },
            right_id,
        ),
        Parameter::new("p_created", created.as_str()),
        Parameter::new("p_updated", updated),
    ];
    write_row::execute(
        app.command(),
        app.provider(),
        &schema::ident(app.provider(), link_table),
        procedure,
        &insert,
        &call,
    )
    .await
}
