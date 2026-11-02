using Datasources;

public sealed class PhoneAttributeRepository : CRUDRepository<Backend.Types.Datasource.PhoneAttribute>
{
    public PhoneAttributeRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.PhoneAttribute> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.PhoneAttribute>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.Datasource.PhoneAttribute> Options() => new()
    {
        Entity = "PhoneAttribute",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["PhoneAttribute"] = "phone_attributes" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Uuid"] = "uuid",
            ["Created"] = "created",
            ["Updated"] = "updated",
            ["Version"] = "version",
            ["PhoneId"] = "phone_id",
            ["Key"] = "key",
            ["Value"] = "value",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "PhoneId", "Key", "Value"],
        CreateProperties = ["Uuid", "Created", "Updated", "Version", "PhoneId", "Key", "Value"],
        UpdateProperties = ["Uuid", "Created", "Updated", "Version", "PhoneId", "Key", "Value"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, true),
            ["Updated"] = new("datetime", false, true),
            ["Version"] = new("binary", false, true),
            ["PhoneId"] = new("integer", false, false),
            ["Key"] = new("string", false, false),
            ["Value"] = new("string", false, false),
        }),
    };
}
