using Datasources;

public sealed class ContactsRepository : CRUDRepository<Backend.Types.View.Contact>
{
    public AddressesRepository Address { get; }
    public PhonesRepository Phone { get; }
    public ContactSettingsRepository ContactSettings { get; }
    public ContactPreferencesRepository ContactPreferences { get; }
    public ContactSourcesRepository ContactSource { get; }

    public ContactsRepository(IConnection connection) : base(connection, Options(connection))
    {
        Address = new AddressesRepository(connection);
        Phone = new PhonesRepository(connection);
        ContactSettings = new ContactSettingsRepository(connection);
        ContactPreferences = new ContactPreferencesRepository(connection);
        ContactSource = new ContactSourcesRepository(connection);
        AddEnriched(new("Addresses", "Id", "ContactId", Type: EnrichmentPropertyType.List), Address);
        AddEnriched(new("Phones", "Id", "ContactId", Type: EnrichmentPropertyType.List), Phone);
        AddEnriched(new("Settings", "Id", "ContactId", "Value", Type: EnrichmentPropertyType.Dictionary), ContactSettings);
        AddEnriched(new("Prefs", "Id", "ContactId", Type: EnrichmentPropertyType.Dictionary), ContactPreferences);
        AddEnriched(new("ContactSourceName", "ContactSourceId", "Id", "Name"), ContactSource);
        AddEnriched(new("Description", "ContactSourceId", "Id", "Description"), ContactSource);
    }

    public static IReadonlyRepository<Backend.Types.View.Contact> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.Contact>(connection, Options(connection));
        var Address = new AddressesRepository(connection);
        var Phone = new PhonesRepository(connection);
        var ContactSettings = new ContactSettingsRepository(connection);
        var ContactPreferences = new ContactPreferencesRepository(connection);
        var ContactSource = new ContactSourcesRepository(connection);
        repo.AddEnriched(new("Addresses", "Id", "ContactId", Type: EnrichmentPropertyType.List), Address);
        repo.AddEnriched(new("Phones", "Id", "ContactId", Type: EnrichmentPropertyType.List), Phone);
        repo.AddEnriched(new("Settings", "Id", "ContactId", "Value", Type: EnrichmentPropertyType.Dictionary), ContactSettings);
        repo.AddEnriched(new("Prefs", "Id", "ContactId", Type: EnrichmentPropertyType.Dictionary), ContactPreferences);
        repo.AddEnriched(new("ContactSourceName", "ContactSourceId", "Id", "Name"), ContactSource);
        repo.AddEnriched(new("Description", "ContactSourceId", "Id", "Description"), ContactSource);
        return repo;
    }

    static CRUDRepositoryOptions<Backend.Types.View.Contact> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.View.Contact>()
            .Add(new()
            {
                Entity = "Contact",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Contact"] = "contacts_bases" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["FirstName"] = "first_name",
                    ["LastName"] = "last_name",
                    ["Email"] = "email",
                    ["Notes"] = "notes",
                    ["ContactSourceId"] = "contact_source_id",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "FirstName", "LastName", "Email", "Notes", "ContactSourceId"],
                CreateProperties = ["Uuid", "Created", "Updated", "Version", "FirstName", "LastName", "Email", "Notes"],
                UpdateProperties = ["Uuid", "Created", "Updated", "Version", "FirstName", "LastName", "Email", "Notes"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["FirstName"] = new("string", false, false),
                    ["LastName"] = new("string", false, false),
                    ["Email"] = new("string", true, false),
                    ["Notes"] = new("string", true, false),
                    ["ContactSourceName"] = new("string", false, false),
                    ["Description"] = new("string", true, false),
                }),
            })
            .Add("sqlite", new()
            {
                Entity = "Contact",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Contact"] = "contacts" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["FirstName"] = "first_name",
                    ["LastName"] = "last_name",
                    ["Email"] = "email",
                    ["Notes"] = "notes",
                    ["ContactSourceId"] = "contact_source_id",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "FirstName", "LastName", "Email", "Notes", "ContactSourceId"],
                CreateProperties = ["Uuid", "Version", "FirstName", "LastName", "Email", "Notes"],
                UpdateProperties = ["Uuid", "Version", "FirstName", "LastName", "Email", "Notes"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["FirstName"] = new("string", false, false),
                    ["LastName"] = new("string", false, false),
                    ["Email"] = new("string", true, false),
                    ["Notes"] = new("string", true, false),
                    ["ContactSourceName"] = new("string", false, false),
                    ["Description"] = new("string", true, false),
                }),
            })
            .Add("mysql", new()
            {
                Entity = "Contact",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Contact"] = "contacts" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["FirstName"] = "first_name",
                    ["LastName"] = "last_name",
                    ["Email"] = "email",
                    ["Notes"] = "notes",
                    ["ContactSourceId"] = "contact_source_id",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "FirstName", "LastName", "Email", "Notes", "ContactSourceId"],
                CreateProperties = ["FirstName", "LastName", "Email", "Notes"],
                UpdateProperties = ["Uuid", "Version", "FirstName", "LastName", "Email", "Notes"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["FirstName"] = new("string", false, false),
                    ["LastName"] = new("string", false, false),
                    ["Email"] = new("string", true, false),
                    ["Notes"] = new("string", true, false),
                    ["ContactSourceName"] = new("string", false, false),
                    ["Description"] = new("string", true, false),
                }),
            })
            .Add("postgres", new()
            {
                Entity = "Contact",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Contact"] = "contacts" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["FirstName"] = "first_name",
                    ["LastName"] = "last_name",
                    ["Email"] = "email",
                    ["Notes"] = "notes",
                    ["ContactSourceId"] = "contact_source_id",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "FirstName", "LastName", "Email", "Notes", "ContactSourceId"],
                CreateProperties = ["FirstName", "LastName", "Email", "Notes"],
                UpdateProperties = ["Uuid", "Version", "FirstName", "LastName", "Email", "Notes"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["FirstName"] = new("string", false, false),
                    ["LastName"] = new("string", false, false),
                    ["Email"] = new("string", true, false),
                    ["Notes"] = new("string", true, false),
                    ["ContactSourceName"] = new("string", false, false),
                    ["Description"] = new("string", true, false),
                }),
            })
            .For(connection);
}
