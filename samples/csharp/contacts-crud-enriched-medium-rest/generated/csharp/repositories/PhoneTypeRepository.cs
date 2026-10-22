using Datasources;

public sealed class PhoneTypeRepository : CRUDRepository<Backend.Types.Datasource.PhoneType>
{
    public PhoneTypeRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.PhoneType> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.PhoneType>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.Datasource.PhoneType> Options() => new()
    {
        Entity = "PhoneType",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["PhoneType"] = "phone_types" }),
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
