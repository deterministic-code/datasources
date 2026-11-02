using Datasources;

public sealed class EmailLabelRepository : CRUDRepository<Backend.Types.Datasource.EmailLabel>
{
    public EmailLabelRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.EmailLabel> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.EmailLabel>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.Datasource.EmailLabel> Options() => new()
    {
        Entity = "EmailLabel",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["EmailLabel"] = "email_labels" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Uuid"] = "uuid",
            ["Created"] = "created",
            ["Updated"] = "updated",
            ["Version"] = "version",
            ["EmailId"] = "email_id",
            ["LabelId"] = "label_id",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "EmailId", "LabelId"],
        CreateProperties = ["Uuid", "Created", "Updated", "Version", "EmailId", "LabelId"],
        UpdateProperties = ["Uuid", "Created", "Updated", "Version", "EmailId", "LabelId"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, true),
            ["Updated"] = new("datetime", false, true),
            ["Version"] = new("binary", false, true),
            ["EmailId"] = new("integer", false, false),
            ["LabelId"] = new("integer", false, false),
        }),
    };
}
