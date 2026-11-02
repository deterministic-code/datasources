using Datasources;

namespace UserRolesSimpleCli;

public static class UsersRolesGroupsQuery
{
    public static ISelectStatement Create() =>
        new SelectStatement(new()
        {
            Table = "users",
            Fields =
            [
                "users.id AS id",
                "users.username AS username",
                "users.is_enabled AS is_enabled",
                "users.created AS created",
                "roles.id AS role_id",
                "roles.name AS role_name",
                "roles.is_enabled AS role_is_enabled",
                "groups.id AS group_id",
                "groups.ugly_name AS group_name",
                "groups.is_enabled AS group_is_enabled"
            ],
            Joins =
            [
                Join.Left("user_roles", "users.id", "user_roles.user_id"),
                Join.Left("roles", "user_roles.bad_role_id", "roles.id"),
                Join.Left("user_groups", "users.id", "user_groups.user_id"),
                Join.Left("groups", "user_groups.group_id", "groups.id")
            ],
            OrderBy = [OrderBy.Asc("users.id"), OrderBy.Asc("roles.id"), OrderBy.Asc("groups.id")],
        });
}
