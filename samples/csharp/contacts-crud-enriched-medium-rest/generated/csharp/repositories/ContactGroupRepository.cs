using Datasources;

public sealed class ContactGroupRepository : CRUDRepository<Backend.Types.View.ContactGroup>
{
    public ContactRepository Contact { get; }
    public ContactGroupMemberRepository ContactGroupMember { get; }

    public ContactGroupRepository(IConnection connection) : base(connection, Options())
    {
        Contact = new ContactRepository(connection);
        ContactGroupMember = new ContactGroupMemberRepository(connection);
        AddLinked(new("Members", "Id", "Id", Association: new("ContactGroupId", "ContactId")), Contact, ContactGroupMember);
    }

    public static IReadonlyRepository<Backend.Types.View.ContactGroup> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.ContactGroup>(connection, Options());
        var Contact = new ContactRepository(connection);
        var ContactGroupMember = new ContactGroupMemberRepository(connection);
        repo.AddEnriched(new("Members", "Id", "Id", Type: EnrichmentPropertyType.List), Contact);
        return repo;
    }

    static CRUDRepositoryOptions<Backend.Types.View.ContactGroup> Options() => new()
    {
        Entity = "ContactGroup",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactGroup"] = "contact_groups" }),
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
