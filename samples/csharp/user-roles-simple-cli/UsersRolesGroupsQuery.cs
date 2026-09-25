using Datasources;

namespace UserRolesSimpleCli;

public static class UsersRolesGroupsQuery
{
    public static ISelectStatement Create(string provider = "sqlite")
    {
        var users = Schema.Ident(provider, "users");
        var roles = Schema.Ident(provider, "roles");
        var groups = Schema.Ident(provider, "groups");
        var userRoles = Schema.Ident(provider, "user_roles");
        var userGroups = Schema.Ident(provider, "user_groups");
        return new SelectStatement(new()
        {
            Table = users,
            Fields =
            [
                $"{users}.id AS id",
                $"{users}.username AS username",
                $"{users}.is_enabled AS is_enabled",
                $"{users}.created AS created",
                $"{roles}.id AS role_id",
                $"{roles}.name AS role_name",
                $"{roles}.is_enabled AS role_is_enabled",
                $"{groups}.id AS group_id",
                $"{groups}.ugly_name AS group_name",
                $"{groups}.is_enabled AS group_is_enabled"
            ],
            Joins =
            [
                Join.Left(userRoles, $"{users}.id", $"{userRoles}.user_id"),
                Join.Left(roles, $"{userRoles}.bad_role_id", $"{roles}.id"),
                Join.Left(userGroups, $"{users}.id", $"{userGroups}.user_id"),
                Join.Left(groups, $"{userGroups}.group_id", $"{groups}.id")
            ],
            OrderBy = [OrderBy.Asc($"{users}.id"), OrderBy.Asc($"{roles}.id"), OrderBy.Asc($"{groups}.id")],
        });
    }
}
