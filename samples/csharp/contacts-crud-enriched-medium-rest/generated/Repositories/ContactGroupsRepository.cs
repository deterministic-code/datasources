using Datasources;

public sealed class ContactGroupsRepository : CRUDRepository<Backend.Types.View.ContactGroup>
{
    public ContactsRepository Contact { get; }
    public ContactGroupMembersRepository ContactGroupMember { get; }

    public ContactGroupsRepository(IConnection connection) : base(connection, Options(connection))
    {
        Contact = new ContactsRepository(connection);
        ContactGroupMember = new ContactGroupMembersRepository(connection);
        AddLinked(new("Members", "Id", "Id", Association: new("ContactGroupId", "ContactId")), Contact, ContactGroupMember);
    }

    public static IReadonlyRepository<Backend.Types.View.ContactGroup> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.ContactGroup>(connection, Options(connection));
        var Contact = new ContactsRepository(connection);
        var ContactGroupMember = new ContactGroupMembersRepository(connection);
        repo.AddEnriched(new("Members", "Id", "Id", Type: EnrichmentPropertyType.List), Contact);
        return repo;
    }

    static CRUDRepositoryOptions<Backend.Types.View.ContactGroup> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.View.ContactGroup>()
            .Add(new()
            {
                Entity = "ContactGroup",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactGroup"] = "contact_groups_bases" }),
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
                CreateProperties = ["Uuid", "Created", "Updated", "Version", "Name", "Description"],
                UpdateProperties = ["Uuid", "Created", "Updated", "Version", "Name", "Description"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["Name"] = new("string", false, false),
                    ["Description"] = new("string", true, false),
                }),
            })
            .Add("sqlite", new()
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
                CreateProperties = ["Uuid", "Version", "Name", "Description"],
                UpdateProperties = ["Uuid", "Version", "Name", "Description"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["Name"] = new("string", false, false),
                    ["Description"] = new("string", true, false),
                }),
            })
            .Add("mysql", new()
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
                CreateProperties = ["Name", "Description"],
                UpdateProperties = ["Uuid", "Version", "Name", "Description"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["Name"] = new("string", false, false),
                    ["Description"] = new("string", true, false),
                }),
            })
            .Add("postgres", new()
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
                CreateProperties = ["Name", "Description"],
                UpdateProperties = ["Uuid", "Version", "Name", "Description"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["Name"] = new("string", false, false),
                    ["Description"] = new("string", true, false),
                }),
            })
            .For(connection);
}
