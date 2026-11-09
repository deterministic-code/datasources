using Datasources;

public sealed class PhonesRepository : CRUDRepository<Backend.Types.View.Phone>
{
    public PhoneAttributesRepository PhoneAttribute { get; }
    public LabelsRepository Label { get; }
    public PhoneLabelsRepository PhoneLabel { get; }
    public PhoneTypesRepository PhoneType { get; }

    public PhonesRepository(IConnection connection) : base(connection, Options())
    {
        PhoneAttribute = new PhoneAttributesRepository(connection);
        Label = new LabelsRepository(connection);
        PhoneLabel = new PhoneLabelsRepository(connection);
        PhoneType = new PhoneTypesRepository(connection);
        AddEnriched(new("Attributes", "Id", "PhoneId", "Value", Type: EnrichmentPropertyType.Dictionary), PhoneAttribute);
        AddLinked(new("Labels", "Id", "Id", "Name", Association: new("PhoneId", "LabelId")), Label, PhoneLabel);
        AddEnriched(new("PhoneTypeName", "PhoneTypeId", "Id", "Name"), PhoneType);
        AddEnriched(new("Description", "PhoneTypeId", "Id", "Description"), PhoneType);
    }

    public static IReadonlyRepository<Backend.Types.View.Phone> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.Phone>(connection, Options());
        var PhoneAttribute = new PhoneAttributesRepository(connection);
        var Label = new LabelsRepository(connection);
        var PhoneLabel = new PhoneLabelsRepository(connection);
        var PhoneType = new PhoneTypesRepository(connection);
        repo.AddEnriched(new("Attributes", "Id", "PhoneId", "Value", Type: EnrichmentPropertyType.Dictionary), PhoneAttribute);
        repo.AddEnriched(new("Labels", "Id", "Id", "Name", Type: EnrichmentPropertyType.List), Label);
        repo.AddEnriched(new("PhoneTypeName", "PhoneTypeId", "Id", "Name"), PhoneType);
        repo.AddEnriched(new("Description", "PhoneTypeId", "Id", "Description"), PhoneType);
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
            ["Number"] = "number",
            ["DisplayName"] = "display_name",
            ["Extention"] = "extention",
            ["IsPrimary"] = "is_primary",
            ["IsVerified"] = "is_verified",
            ["PhoneTypeId"] = "phone_type_id",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "Number", "DisplayName", "Extention", "IsPrimary", "IsVerified", "PhoneTypeId"],
        CreateProperties = ["Uuid", "Created", "Updated", "Version", "ContactId", "Number", "DisplayName", "Extention", "IsPrimary", "IsVerified"],
        UpdateProperties = ["Uuid", "Created", "Updated", "Version", "ContactId", "Number", "DisplayName", "Extention", "IsPrimary", "IsVerified"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, true),
            ["Updated"] = new("datetime", false, true),
            ["Version"] = new("binary", false, true),
            ["ContactId"] = new("integer", false, false),
            ["Number"] = new("string", false, false),
            ["DisplayName"] = new("string", false, false),
            ["Extention"] = new("string", false, false),
            ["IsPrimary"] = new("boolean", false, false),
            ["IsVerified"] = new("boolean", false, true),
            ["PhoneTypeName"] = new("string", false, false),
            ["Description"] = new("string", false, true),
        }),
    };
}
