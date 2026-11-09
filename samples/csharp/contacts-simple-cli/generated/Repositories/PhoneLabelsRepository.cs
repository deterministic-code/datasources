using Datasources;

public sealed class PhoneLabelsRepository : CRUDRepository<Backend.Types.Datasource.PhoneLabel>
{
    public PhoneLabelsRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.PhoneLabel> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.PhoneLabel>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.Datasource.PhoneLabel> Options() => new()
    {
        Entity = "PhoneLabel",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["PhoneLabel"] = "phone_labels" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Uuid"] = "uuid",
            ["Created"] = "created",
            ["Updated"] = "updated",
            ["Version"] = "version",
            ["PhoneId"] = "phone_id",
            ["LabelId"] = "label_id",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "PhoneId", "LabelId"],
        CreateProperties = ["Uuid", "Created", "Updated", "Version", "PhoneId", "LabelId"],
        UpdateProperties = ["Uuid", "Created", "Updated", "Version", "PhoneId", "LabelId"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, true),
            ["Updated"] = new("datetime", false, true),
            ["Version"] = new("binary", false, true),
            ["PhoneId"] = new("integer", false, false),
            ["LabelId"] = new("integer", false, false),
        }),
    };
}
