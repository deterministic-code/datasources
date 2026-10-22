using Datasources;

public sealed class ContactChangeLogRepository : CRUDRepository<Backend.Types.View.ContactChangeLog>
{
    public ContactChangeLogRepository(IConnection connection) : base(connection, Options()) { }

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
    };
}
