using Datasources;

public sealed class AddressTypesRepository : CRUDRepository<Backend.Types.Datasource.AddressType>
{
    public AddressTypesRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.AddressType> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.AddressType>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.Datasource.AddressType> Options() => new()
    {
        Entity = "AddressType",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["AddressType"] = "address_types" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Uuid"] = "uuid",
            ["Created"] = "created",
            ["Updated"] = "updated",
            ["Version"] = "version",
            ["Name"] = "name",
            ["Description"] = "description",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "Description"],
        CreateProperties = ["Uuid", "Created", "Updated", "Version", "Name", "Description"],
        UpdateProperties = ["Uuid", "Created", "Updated", "Version", "Name", "Description"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, true),
            ["Updated"] = new("datetime", false, true),
            ["Version"] = new("binary", false, true),
            ["Name"] = new("string", false, false),
            ["Description"] = new("string", false, true),
        }),
    };
}
