use datasources::{
    DefaultStatementTextRenderer, Join, OrderBy, SelectStatement, SelectStatementOptions,
};

pub fn create() -> SelectStatement {
    let fields = [
        "users.id AS id",
        "users.username AS username",
        "users.is_enabled AS is_enabled",
        "users.created AS created",
        "roles.id AS role_id",
        "roles.name AS role_name",
        "roles.is_enabled AS role_is_enabled",
        "groups.id AS group_id",
        "groups.ugly_name AS group_name",
        "groups.is_enabled AS group_is_enabled",
    ];
    let order = [
        OrderBy::asc("users.id"),
        OrderBy::asc("roles.id"),
        OrderBy::asc("groups.id"),
    ];
    SelectStatement::new(SelectStatementOptions {
        joins: &[
            Join::left("user_roles", "users.id", "user_roles.user_id"),
            Join::left("roles", "user_roles.bad_role_id", "roles.id"),
            Join::left("user_groups", "users.id", "user_groups.user_id"),
            Join::left("groups", "user_groups.group_id", "groups.id"),
        ],
        order_by: &order,
        ..SelectStatementOptions::new("users", &fields)
    })
}

pub fn text() -> String {
    DefaultStatementTextRenderer::instance().render(&create())
}
