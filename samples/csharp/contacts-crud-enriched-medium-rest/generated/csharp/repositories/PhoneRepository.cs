using Datasources;

public sealed class PhoneRepository : CRUDRepository<Backend.Types.View.Phone>
{
    public PhoneTypeRepository PhoneType { get; }

    public PhoneRepository(IConnection connection) : base(connection, Options(connection))
    {
        PhoneType = new PhoneTypeRepository(connection);
        AddEnriched(new("PhoneTypeName", "PhoneTypeId", "Id", "Name"), PhoneType);
    }

    public static IReadonlyRepository<Backend.Types.View.Phone> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.Phone>(connection, Options(connection));
        var PhoneType = new PhoneTypeRepository(connection);
        repo.AddEnriched(new("PhoneTypeName", "PhoneTypeId", "Id", "Name"), PhoneType);
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
                    ["PhoneTypeId"] = "phone_type_id",
                    ["Number"] = "number",
                    ["Label"] = "label",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "PhoneTypeId", "Number", "Label"],
                CreateProperties = ["Uuid", "Created", "Updated", "Version", "ContactId", "PhoneTypeId", "Number", "Label"],
                UpdateProperties = ["Uuid", "Created", "Updated", "Version", "ContactId", "PhoneTypeId", "Number", "Label"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["ContactId"] = new("integer", false, false),
                    ["PhoneTypeId"] = new("integer", false, false),
                    ["Number"] = new("string", false, false),
                    ["Label"] = new("string", true, false),
                }),
            })
            .Add("sqlite", new()
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
                CreateProperties = ["Uuid", "Version", "ContactId", "PhoneTypeId", "Number", "Label"],
                UpdateProperties = ["Uuid", "Version", "ContactId", "PhoneTypeId", "Number", "Label"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["ContactId"] = new("integer", false, false),
                    ["PhoneTypeId"] = new("integer", false, false),
                    ["Number"] = new("string", false, false),
                    ["Label"] = new("string", true, false),
                }),
            })
            .Add("mysql", new()
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
                CreateProperties = ["ContactId", "PhoneTypeId", "Number", "Label"],
                UpdateProperties = ["Uuid", "Version", "ContactId", "PhoneTypeId", "Number", "Label"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["ContactId"] = new("integer", false, false),
                    ["PhoneTypeId"] = new("integer", false, false),
                    ["Number"] = new("string", false, false),
                    ["Label"] = new("string", true, false),
                }),
            })
            .Add("postgres", new()
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
                CreateProperties = ["ContactId", "PhoneTypeId", "Number", "Label"],
                UpdateProperties = ["Uuid", "Version", "ContactId", "PhoneTypeId", "Number", "Label"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["ContactId"] = new("integer", false, false),
                    ["PhoneTypeId"] = new("integer", false, false),
                    ["Number"] = new("string", false, false),
                    ["Label"] = new("string", true, false),
                }),
            })
            .For(connection);
}
