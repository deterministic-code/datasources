using Datasources;

public sealed class PhonesRepository : CRUDRepository<Backend.Types.View.Phone>
{
    public PhoneTypesRepository PhoneType { get; }

    public PhonesRepository(IConnection connection) : base(connection, Options(connection))
    {
        PhoneType = new PhoneTypesRepository(connection);
        AddEnriched(new("PhoneTypeName", "PhoneTypeId", "Id", "Name"), PhoneType);
        AddEnriched(new("Description", "PhoneTypeId", "Id", "Description"), PhoneType);
    }

    public static IReadonlyRepository<Backend.Types.View.Phone> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.Phone>(connection, Options(connection));
        var PhoneType = new PhoneTypesRepository(connection);
        repo.AddEnriched(new("PhoneTypeName", "PhoneTypeId", "Id", "Name"), PhoneType);
        repo.AddEnriched(new("Description", "PhoneTypeId", "Id", "Description"), PhoneType);
        return repo;
    }

    static CRUDRepositoryOptions<Backend.Types.View.Phone> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.View.Phone>()
            .Add(new()
            {
                Entity = "Phone",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Phone"] = "phone_bases" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["ContactId"] = "contact_id",
                    ["Number"] = "number",
                    ["Label"] = "label",
                    ["PhoneTypeId"] = "phone_type_id",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "Number", "Label", "PhoneTypeId"],
                CreateProperties = ["ContactId", "Number", "Label"],
                UpdateProperties = ["Uuid", "ContactId", "Number", "Label"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["ContactId"] = new("biginteger", false, false),
                    ["Number"] = new("string", false, false),
                    ["Label"] = new("string", true, false),
                    ["PhoneTypeName"] = new("string", false, false),
                    ["Description"] = new("string", true, false),
                }),
            })
            .Add("sqlite", new()
            {
                Entity = "Phone",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Phone"] = "phone_bases" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["ContactId"] = "contact_id",
                    ["Number"] = "number",
                    ["Label"] = "label",
                    ["PhoneTypeId"] = "phone_type_id",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "Number", "Label", "PhoneTypeId"],
                CreateProperties = ["Uuid", "Created", "Version", "ContactId", "Number", "Label"],
                UpdateProperties = ["Uuid", "Created", "Version", "ContactId", "Number", "Label"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["ContactId"] = new("biginteger", false, false),
                    ["Number"] = new("string", false, false),
                    ["Label"] = new("string", true, false),
                    ["PhoneTypeName"] = new("string", false, false),
                    ["Description"] = new("string", true, false),
                }),
            })
            .For(connection);
}
