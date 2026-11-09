using Datasources;

public sealed class ContactLabelsRepository : CRUDRepository<Backend.Types.Datasource.ContactLabel>
{
    public ContactLabelsRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.ContactLabel> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.ContactLabel>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.Datasource.ContactLabel> Options() => new()
    {
        Entity = "ContactLabel",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactLabel"] = "contact_labels" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Uuid"] = "uuid",
            ["Created"] = "created",
            ["Updated"] = "updated",
            ["Version"] = "version",
            ["ContactId"] = "contact_id",
            ["LabelId"] = "label_id",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "LabelId"],
        CreateProperties = ["Uuid", "Created", "Updated", "Version", "ContactId", "LabelId"],
        UpdateProperties = ["Uuid", "Created", "Updated", "Version", "ContactId", "LabelId"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, true),
            ["Updated"] = new("datetime", false, true),
            ["Version"] = new("binary", false, true),
            ["ContactId"] = new("integer", false, false),
            ["LabelId"] = new("integer", false, false),
        }),
    };
}
