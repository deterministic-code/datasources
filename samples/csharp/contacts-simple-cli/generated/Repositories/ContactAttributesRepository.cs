using Datasources;

public sealed class ContactAttributesRepository : CRUDRepository<Backend.Types.Datasource.ContactAttribute>
{
    public ContactAttributesRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.ContactAttribute> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.ContactAttribute>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.Datasource.ContactAttribute> Options() => new()
    {
        Entity = "ContactAttribute",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactAttribute"] = "contact_attributes" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Uuid"] = "uuid",
            ["Created"] = "created",
            ["Updated"] = "updated",
            ["Version"] = "version",
            ["ContactId"] = "contact_id",
            ["Key"] = "key",
            ["Value"] = "value",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "Key", "Value"],
        CreateProperties = ["Uuid", "Created", "Updated", "Version", "ContactId", "Key", "Value"],
        UpdateProperties = ["Uuid", "Created", "Updated", "Version", "ContactId", "Key", "Value"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, true),
            ["Updated"] = new("datetime", false, true),
            ["Version"] = new("binary", false, true),
            ["ContactId"] = new("integer", false, false),
            ["Key"] = new("string", false, false),
            ["Value"] = new("string", false, false),
        }),
    };
}
