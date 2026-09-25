use crate::app::App;
use crate::commands::lookup_id;
use crate::schema;
use datasources::{
    ExecuteStoredProcedureStatement, Operator, Parameter, Result, UpdateStatement, IWhere, Where,
};

pub async fn execute(app: &App<'_>, username: &str, enabled: bool) -> Result<String> {
    let updated = app.token_bytes();
    if app.use_procedures() && app.provider() == "mysql" {
        let fields = [
            Parameter::new("p_username", username),
            Parameter::new("p_is_enabled", schema::flag(app.provider(), enabled)),
            Parameter::new("p_updated", updated),
        ];
        app.command()
            .execute_statement(
                &ExecuteStoredProcedureStatement::new("set_user_enabled", &fields),
                Some(&fields),
            )
            .await?;
        return Ok(String::new());
    }
    let users = schema::ident(app.provider(), "users");
    let id = lookup_id::of(app.command(), &users, "username", username).await?;
    let fields = [
        Parameter::new("is_enabled", schema::flag(app.provider(), enabled)),
        Parameter::new("updated", updated),
    ];
    let where_clause = Where::new(Operator::equals("id", id));
    let mut parameters = fields.to_vec();
    parameters.extend(where_clause.parameters());
    app.command()
        .execute_statement(
            &UpdateStatement::new(users, &fields, Some(&where_clause)),
            Some(&parameters),
        )
        .await?;
    Ok(String::new())
}
