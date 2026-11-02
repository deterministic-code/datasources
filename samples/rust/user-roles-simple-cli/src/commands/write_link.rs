use crate::app::App;
use crate::commands::{lookup_id, write_row};
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
    let user_id = lookup_id::of(app.command(), "users", "username", username).await?;
    let right_id = lookup_id::of(app.command(), right_table, right_column, right_value).await?;
    let insert = [
        Parameter::new("user_id", user_id),
        Parameter::new(link_column, right_id),
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
    ];
    write_row::execute(
        app.command(),
        app.provider(),
        link_table,
        procedure,
        &insert,
        &call,
        app.use_procedures(),
    )
    .await
}
