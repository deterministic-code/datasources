use crate::schema;
use datasources::{Join, QueryStatement, Text};

pub fn create(provider: &str) -> QueryStatement {
    let users = schema::ident(provider, "users");
    let roles = schema::ident(provider, "roles");
    let groups = schema::ident(provider, "groups");
    let user_roles = schema::ident(provider, "user_roles");
    let user_groups = schema::ident(provider, "user_groups");
    let fields = [
        format!("{users}.id AS id"),
        format!("{users}.username AS username"),
        format!("{users}.is_enabled AS is_enabled"),
        format!("{users}.created AS created"),
        format!("{roles}.id AS role_id"),
        format!("{roles}.name AS role_name"),
        format!("{roles}.is_enabled AS role_is_enabled"),
        format!("{groups}.id AS group_id"),
        format!("{groups}.ugly_name AS group_name"),
        format!("{groups}.is_enabled AS group_is_enabled"),
    ];
    let field_refs: Vec<&str> = fields.iter().map(String::as_str).collect();
    let left_ur = format!("{users}.id");
    let right_ur = format!("{user_roles}.user_id");
    let left_role = format!("{user_roles}.bad_role_id");
    let right_role = format!("{roles}.id");
    let left_ug = format!("{users}.id");
    let right_ug = format!("{user_groups}.user_id");
    let left_group = format!("{user_groups}.group_id");
    let right_group = format!("{groups}.id");
    QueryStatement::new(
        &users,
        &field_refs,
        None,
        &[
            Join::left(&user_roles, &left_ur, &right_ur),
            Join::left(&roles, &left_role, &right_role),
            Join::left(&user_groups, &left_ug, &right_ug),
            Join::left(&groups, &left_group, &right_group),
        ],
    )
}

pub fn text(provider: &str) -> String {
    create(provider).get_text()
}
