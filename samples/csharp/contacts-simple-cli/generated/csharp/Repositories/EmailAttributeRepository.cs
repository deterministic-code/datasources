using Datasources;

public sealed class EmailAttributeRepository : CRUDRepository<Backend.Types.Datasource.EmailAttribute>
{
    public EmailAttributeRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.EmailAttribute> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.EmailAttribute>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.Datasource.EmailAttribute> Options() => new()
    {
        Entity = "EmailAttribute",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["EmailAttribute"] = "email_attributes" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Uuid"] = "uuid",
            ["Created"] = "created",
            ["Updated"] = "updated",
            ["Version"] = "version",
            ["EmailId"] = "email_id",
            ["Key"] = "key",
            ["Value"] = "value",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "EmailId", "Key", "Value"],
        CreateProperties = ["Uuid", "Created", "Updated", "Version", "EmailId", "Key", "Value"],
        UpdateProperties = ["Uuid", "Created", "Updated", "Version", "EmailId", "Key", "Value"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, true),
            ["Updated"] = new("datetime", false, true),
            ["Version"] = new("binary", false, true),
            ["EmailId"] = new("integer", false, false),
            ["Key"] = new("string", false, false),
            ["Value"] = new("string", false, false),
        }),
    };
}
