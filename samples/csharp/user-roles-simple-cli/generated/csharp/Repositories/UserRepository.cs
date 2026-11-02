using Datasources;

public sealed class UserRepository : CRUDRepository<Backend.Types.Datasource.User>
{
    public UserRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.User> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.User>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.Datasource.User> Options() => new()
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
            ["Created"] = new("datetime", false, false),
            ["Updated"] = new("binary", false, true),
            ["Username"] = new("string", false, false),
            ["Password"] = new("string", false, false),
            ["IsEnabled"] = new("boolean", false, true),
        }),
    };
}
