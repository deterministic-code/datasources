using Datasources;

public sealed class AddressAttributeRepository : CRUDRepository<Backend.Types.Datasource.AddressAttribute>
{
    public AddressAttributeRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.AddressAttribute> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.AddressAttribute>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.Datasource.AddressAttribute> Options() => new()
    {
        Entity = "AddressAttribute",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["AddressAttribute"] = "address_attributes" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Uuid"] = "uuid",
            ["Created"] = "created",
            ["Updated"] = "updated",
            ["Version"] = "version",
            ["AddressId"] = "address_id",
            ["Key"] = "key",
            ["Value"] = "value",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "AddressId", "Key", "Value"],
        CreateProperties = ["Uuid", "Created", "Updated", "Version", "AddressId", "Key", "Value"],
        UpdateProperties = ["Uuid", "Created", "Updated", "Version", "AddressId", "Key", "Value"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, true),
            ["Updated"] = new("datetime", false, true),
            ["Version"] = new("binary", false, true),
            ["AddressId"] = new("integer", false, false),
            ["Key"] = new("string", false, false),
            ["Value"] = new("string", false, false),
        }),
    };
}
