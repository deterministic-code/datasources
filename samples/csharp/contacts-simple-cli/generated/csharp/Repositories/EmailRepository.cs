using Datasources;

public sealed class EmailRepository : CRUDRepository<Backend.Types.View.Email>
{
    public EmailAttributeRepository EmailAttribute { get; }
    public LabelRepository Label { get; }
    public EmailLabelRepository EmailLabel { get; }
    public EmailTypeRepository EmailType { get; }

    public EmailRepository(IConnection connection) : base(connection, Options())
    {
        EmailAttribute = new EmailAttributeRepository(connection);
        Label = new LabelRepository(connection);
        EmailLabel = new EmailLabelRepository(connection);
        EmailType = new EmailTypeRepository(connection);
        AddEnriched(new("Attributes", "Id", "EmailId", "Value", Type: EnrichmentPropertyType.Dictionary), EmailAttribute);
        AddLinked(new("Labels", "Id", "Id", Association: new("EmailId", "LabelId")), Label, EmailLabel);
        AddEnriched(new("EmailTypeName", "EmailTypeId", "Id", "Name"), EmailType);
    }

    public static IReadonlyRepository<Backend.Types.View.Email> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.Email>(connection, Options());
        var EmailAttribute = new EmailAttributeRepository(connection);
        var Label = new LabelRepository(connection);
        var EmailLabel = new EmailLabelRepository(connection);
        var EmailType = new EmailTypeRepository(connection);
        repo.AddEnriched(new("Attributes", "Id", "EmailId", "Value", Type: EnrichmentPropertyType.Dictionary), EmailAttribute);
        repo.AddEnriched(new("Labels", "Id", "Id", Type: EnrichmentPropertyType.List), Label);
        repo.AddEnriched(new("EmailTypeName", "EmailTypeId", "Id", "Name"), EmailType);
        return repo;
    }

    static CRUDRepositoryOptions<Backend.Types.View.Email> Options() => new()
    {
        Entity = "Email",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Email"] = "emails" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Uuid"] = "uuid",
            ["Created"] = "created",
            ["Updated"] = "updated",
            ["Version"] = "version",
            ["EmailTypeId"] = "email_type_id",
            ["ContactId"] = "contact_id",
            ["Address"] = "address",
            ["DisplayName"] = "display_name",
            ["IsPrimary"] = "is_primary",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "EmailTypeId", "ContactId", "Address", "DisplayName", "IsPrimary"],
        CreateProperties = ["Uuid", "Created", "Updated", "Version", "EmailTypeId", "ContactId", "Address", "DisplayName", "IsPrimary"],
        UpdateProperties = ["Uuid", "Created", "Updated", "Version", "EmailTypeId", "ContactId", "Address", "DisplayName", "IsPrimary"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, true),
            ["Updated"] = new("datetime", false, true),
            ["Version"] = new("binary", false, true),
            ["EmailTypeId"] = new("integer", false, false),
            ["ContactId"] = new("integer", false, false),
            ["Address"] = new("string", false, true),
            ["DisplayName"] = new("string", false, true),
            ["IsPrimary"] = new("boolean", false, false),
        }),
    };
}
