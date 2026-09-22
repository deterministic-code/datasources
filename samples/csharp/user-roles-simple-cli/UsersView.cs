using Datasources;

namespace UserRolesSimpleCli;

public static class UsersView
{
    public static TypeMap TypeMap { get; } = new()
    {
        Key = "id",
        Fields = new Dictionary<string, TypeMapField>
        {
            ["id"] = "id",
            ["username"] = "username",
            ["is_enabled"] = "is_enabled",
            ["created"] = "created",
            ["roles"] = new TypeMapGroup
            {
                Key = "role_id",
                Fields = new Dictionary<string, TypeMapField>
                {
                    ["id"] = "role_id",
                    ["name"] = "role_name",
                    ["is_enabled"] = "role_is_enabled"
                }
            },
            ["groups"] = new TypeMapGroup
            {
                Key = "group_id",
                Fields = new Dictionary<string, TypeMapField>
                {
                    ["id"] = "group_id",
                    ["name"] = "group_name",
                    ["is_enabled"] = "group_is_enabled"
                }
            }
        }
    };
}
