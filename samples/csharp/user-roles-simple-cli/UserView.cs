namespace UserRolesSimpleCli;

public sealed record UserView(
    int id,
    string username,
    bool is_enabled,
    string created,
    RoleView[] roles,
    GroupView[] groups);
