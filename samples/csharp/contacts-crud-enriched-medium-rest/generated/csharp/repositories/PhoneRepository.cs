using Datasources;

public sealed class PhoneRepository : CRUDRepository<Backend.Types.View.Phone>
{
    public PhoneTypeRepository PhoneType { get; }

    public PhoneRepository(IConnection connection) : base(connection, Options())
    {
        PhoneType = new PhoneTypeRepository(connection);
        AddEnriched(new("PhoneTypeName", "PhoneTypeId", "Id", "Name"), PhoneType);
    }

    public static IReadonlyRepository<Backend.Types.View.Phone> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.Phone>(connection, Options());
        var PhoneType = new PhoneTypeRepository(connection);
        repo.AddEnriched(new("PhoneTypeName", "PhoneTypeId", "Id", "Name"), PhoneType);
        return repo;
    }

    static CRUDRepositoryOptions<Backend.Types.View.Phone> Options() => new()
    {
        Entity = "Phone",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Phone"] = "phones" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Uuid"] = "uuid",
            ["Created"] = "created",
            ["Updated"] = "updated",
            ["Version"] = "version",
            ["ContactId"] = "contact_id",
            ["PhoneTypeId"] = "phone_type_id",
            ["Number"] = "number",
            ["Label"] = "label",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "PhoneTypeId", "Number", "Label"],
    };
}
