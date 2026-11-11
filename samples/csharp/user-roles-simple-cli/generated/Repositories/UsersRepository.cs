using Datasources;

public sealed class UsersRepository : CRUDRepository<Backend.Types.View.User>
{
    public RolesRepository Role { get; }
    public UserRolesRepository UserRole { get; }
    public GroupsRepository Group { get; }
    public UserGroupsRepository UserGroup { get; }

    public UsersRepository(IConnection connection) : base(connection, Options())
    {
        Role = new RolesRepository(connection);
        UserRole = new UserRolesRepository(connection);
        Group = new GroupsRepository(connection);
        UserGroup = new UserGroupsRepository(connection);
        AddLinked(new("Roles", "Id", "Id", "Name", Association: new("UserId", "RoleId")), Role, UserRole);
        AddLinked(new("Groups", "Id", "Id", "Name", Association: new("UserId", "GroupId")), Group, UserGroup);
    }

    public static IReadonlyRepository<Backend.Types.View.User> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.User>(connection, Options());
        var Role = new RolesRepository(connection);
        var UserRole = new UserRolesRepository(connection);
        var Group = new GroupsRepository(connection);
        var UserGroup = new UserGroupsRepository(connection);
        repo.AddEnriched(new("Roles", "Id", "Id", "Name", Type: EnrichmentPropertyType.List), Role);
        repo.AddEnriched(new("Groups", "Id", "Id", "Name", Type: EnrichmentPropertyType.List), Group);
        return repo;
    }

    static CRUDRepositoryOptions<Backend.Types.View.User> Options() => new()
    {
        Entity = "User",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["User"] = "users" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Uuid"] = "uuid",
            ["Created"] = "created",
            ["Updated"] = "updated",
            ["Username"] = "username",
            ["Password"] = "password",
            ["IsEnabled"] = "is_enabled",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Username", "Password", "IsEnabled"],
        CreateProperties = ["Username", "Password", "IsEnabled"],
        UpdateProperties = ["Uuid", "Username", "Password", "IsEnabled"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, true),
            ["Updated"] = new("binary", false, true),
            ["Username"] = new("string", false, false),
            ["Password"] = new("string", false, false),
            ["IsEnabled"] = new("boolean", false, true),
        }),
    };
}
