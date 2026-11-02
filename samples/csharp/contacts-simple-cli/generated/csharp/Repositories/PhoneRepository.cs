using Datasources;

public sealed class PhoneRepository : CRUDRepository<Backend.Types.View.Phone>
{
    public PhoneAttributeRepository PhoneAttribute { get; }
    public LabelRepository Label { get; }
    public PhoneLabelRepository PhoneLabel { get; }
    public PhoneTypeRepository PhoneType { get; }

    public PhoneRepository(IConnection connection) : base(connection, Options())
    {
        PhoneAttribute = new PhoneAttributeRepository(connection);
        Label = new LabelRepository(connection);
        PhoneLabel = new PhoneLabelRepository(connection);
        PhoneType = new PhoneTypeRepository(connection);
        AddEnriched(new("Attributes", "Id", "PhoneId", "Value", Type: EnrichmentPropertyType.Dictionary), PhoneAttribute);
        AddLinked(new("Labels", "Id", "Id", Association: new("PhoneId", "LabelId")), Label, PhoneLabel);
        AddEnriched(new("PhoneTypeName", "PhoneTypeId", "Id", "Name"), PhoneType);
    }

    public static IReadonlyRepository<Backend.Types.View.Phone> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.Phone>(connection, Options());
        var PhoneAttribute = new PhoneAttributeRepository(connection);
        var Label = new LabelRepository(connection);
        var PhoneLabel = new PhoneLabelRepository(connection);
        var PhoneType = new PhoneTypeRepository(connection);
        repo.AddEnriched(new("Attributes", "Id", "PhoneId", "Value", Type: EnrichmentPropertyType.Dictionary), PhoneAttribute);
        repo.AddEnriched(new("Labels", "Id", "Id", Type: EnrichmentPropertyType.List), Label);
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
            ["PhoneTypeId"] = "phone_type_id",
            ["ContactId"] = "contact_id",
            ["Number"] = "number",
            ["DisplayName"] = "display_name",
            ["Extention"] = "extention",
            ["IsPrimary"] = "is_primary",
            ["IsVerified"] = "is_verified",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "PhoneTypeId", "ContactId", "Number", "DisplayName", "Extention", "IsPrimary", "IsVerified"],
        CreateProperties = ["Uuid", "Created", "Updated", "Version", "PhoneTypeId", "ContactId", "Number", "DisplayName", "Extention", "IsPrimary", "IsVerified"],
        UpdateProperties = ["Uuid", "Created", "Updated", "Version", "PhoneTypeId", "ContactId", "Number", "DisplayName", "Extention", "IsPrimary", "IsVerified"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, true),
            ["Updated"] = new("datetime", false, true),
            ["Version"] = new("binary", false, true),
            ["PhoneTypeId"] = new("integer", false, false),
            ["ContactId"] = new("integer", false, false),
            ["Number"] = new("string", false, false),
            ["DisplayName"] = new("string", false, false),
            ["Extention"] = new("string", false, false),
            ["IsPrimary"] = new("boolean", false, false),
            ["IsVerified"] = new("boolean", false, true),
        }),
    };
}
