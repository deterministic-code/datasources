using Datasources;

public sealed class AddressTypeRepository : CRUDRepository<Backend.Types.Datasource.AddressType>
{
    public AddressTypeRepository(IConnection connection) : base(connection, Options()) { }

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
    };
}
