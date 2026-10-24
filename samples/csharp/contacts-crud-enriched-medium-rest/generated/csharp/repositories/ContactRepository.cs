using Datasources;

public sealed class ContactRepository : CRUDRepository<Backend.Types.View.Contact>
{
    public AddressRepository Address { get; }
    public PhoneRepository Phone { get; }
    public ContactSettingsRepository ContactSettings { get; }
    public ContactPreferencesRepository ContactPreferences { get; }
    public ContactSourceRepository ContactSource { get; }

    public ContactRepository(IConnection connection) : base(connection, Options(connection))
    {
        Address = new AddressRepository(connection);
        Phone = new PhoneRepository(connection);
        ContactSettings = new ContactSettingsRepository(connection);
        ContactPreferences = new ContactPreferencesRepository(connection);
        ContactSource = new ContactSourceRepository(connection);
        AddEnriched(new("Addresses", "Id", "ContactId", Type: EnrichmentPropertyType.List), Address);
        AddEnriched(new("Phones", "Id", "ContactId", Type: EnrichmentPropertyType.List), Phone);
        AddEnriched(new("Settings", "Id", "ContactId", "Value", Type: EnrichmentPropertyType.Dictionary), ContactSettings);
        AddEnriched(new("Prefs", "Id", "ContactId", Type: EnrichmentPropertyType.Dictionary), ContactPreferences);
        AddEnriched(new("ContactSourceName", "ContactSourceId", "Id", "Name"), ContactSource);
    }

    public static IReadonlyRepository<Backend.Types.View.Contact> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.Contact>(connection, Options(connection));
        var Address = new AddressRepository(connection);
        var Phone = new PhoneRepository(connection);
        var ContactSettings = new ContactSettingsRepository(connection);
        var ContactPreferences = new ContactPreferencesRepository(connection);
        var ContactSource = new ContactSourceRepository(connection);
        repo.AddEnriched(new("Addresses", "Id", "ContactId", Type: EnrichmentPropertyType.List), Address);
        repo.AddEnriched(new("Phones", "Id", "ContactId", Type: EnrichmentPropertyType.List), Phone);
        repo.AddEnriched(new("Settings", "Id", "ContactId", "Value", Type: EnrichmentPropertyType.Dictionary), ContactSettings);
        repo.AddEnriched(new("Prefs", "Id", "ContactId", Type: EnrichmentPropertyType.Dictionary), ContactPreferences);
        repo.AddEnriched(new("ContactSourceName", "ContactSourceId", "Id", "Name"), ContactSource);
        return repo;
    }

    static CRUDRepositoryOptions<Backend.Types.View.Contact> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.View.Contact>()
            .Add(new()
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
                    ["ContactSourceId"] = "contact_source_id",
                    ["FirstName"] = "first_name",
                    ["LastName"] = "last_name",
                    ["Email"] = "email",
                    ["Notes"] = "notes",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactSourceId", "FirstName", "LastName", "Email", "Notes"],
                CreateProperties = ["Uuid", "Version", "ContactSourceId", "FirstName", "LastName", "Email", "Notes"],
                UpdateProperties = ["Uuid", "Version", "ContactSourceId", "FirstName", "LastName", "Email", "Notes"],
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
                    ["ContactSourceId"] = "contact_source_id",
                    ["FirstName"] = "first_name",
                    ["LastName"] = "last_name",
                    ["Email"] = "email",
                    ["Notes"] = "notes",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactSourceId", "FirstName", "LastName", "Email", "Notes"],
                CreateProperties = ["ContactSourceId", "FirstName", "LastName", "Email", "Notes"],
                UpdateProperties = ["Uuid", "Version", "ContactSourceId", "FirstName", "LastName", "Email", "Notes"],
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
                    ["ContactSourceId"] = "contact_source_id",
                    ["FirstName"] = "first_name",
                    ["LastName"] = "last_name",
                    ["Email"] = "email",
                    ["Notes"] = "notes",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactSourceId", "FirstName", "LastName", "Email", "Notes"],
                CreateProperties = ["ContactSourceId", "FirstName", "LastName", "Email", "Notes"],
                UpdateProperties = ["Uuid", "Version", "ContactSourceId", "FirstName", "LastName", "Email", "Notes"],
            })
            .For(connection);
}
