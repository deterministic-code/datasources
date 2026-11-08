using Datasources;

public sealed class ContactChangeLogsRepository : CRUDRepository<Backend.Types.View.ContactChangeLog>
{
    public ContactChangeLogsRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.View.ContactChangeLog> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.View.ContactChangeLog>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.View.ContactChangeLog> Options() => new()
    {
        Entity = "ContactChangeLog",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactChangeLog"] = "contact_change_logs" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["OccurredAt"] = "occurred_at",
            ["ContactId"] = "contact_id",
            ["ChangedBy"] = "changed_by",
            ["ChangeSummary"] = "change_summary",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "OccurredAt", "ContactId", "ChangedBy", "ChangeSummary"],
        CreateProperties = ["OccurredAt", "ContactId", "ChangedBy", "ChangeSummary"],
        UpdateProperties = ["OccurredAt", "ContactId", "ChangedBy", "ChangeSummary"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false),
            ["OccurredAt"] = new("datetime", false, false),
            ["ContactId"] = new("biginteger", false, false),
            ["ChangedBy"] = new("string", false, false),
            ["ChangeSummary"] = new("string", false, false),
        }),
    };
}
