using Datasources;

public sealed class ContactSourceRepository : CRUDRepository<Backend.Types.Datasource.ContactSource>
{
    public ContactSourceRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.ContactSource> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.ContactSource>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.Datasource.ContactSource> Options() => new()
    {
        Entity = "ContactSource",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactSource"] = "contact_sources" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Uuid"] = "uuid",
            ["Created"] = "created",
            ["Updated"] = "updated",
            ["Version"] = "version",
            ["Name"] = "name",
            ["Description"] = "description",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "Description"],
    };
}
