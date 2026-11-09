using Datasources;

public sealed class ContactsRepository : CRUDRepository<Backend.Types.View.Contact>
{
    public AddressesRepository Address { get; }
    public EmailsRepository Email { get; }
    public PhonesRepository Phone { get; }
    public ContactAttributesRepository ContactAttribute { get; }
    public LabelsRepository Label { get; }
    public ContactLabelsRepository ContactLabel { get; }
    public GendersRepository Gender { get; }

    public ContactsRepository(IConnection connection) : base(connection, Options())
    {
        Address = new AddressesRepository(connection);
        Email = new EmailsRepository(connection);
        Phone = new PhonesRepository(connection);
        ContactAttribute = new ContactAttributesRepository(connection);
        Label = new LabelsRepository(connection);
        ContactLabel = new ContactLabelsRepository(connection);
        Gender = new GendersRepository(connection);
        AddEnriched(new("Addresses", "Id", "ContactId", Type: EnrichmentPropertyType.List), Address);
        AddEnriched(new("Emails", "Id", "ContactId", Type: EnrichmentPropertyType.List), Email);
        AddEnriched(new("Phones", "Id", "ContactId", Type: EnrichmentPropertyType.List), Phone);
        AddEnriched(new("Attributes", "Id", "ContactId", "Value", Type: EnrichmentPropertyType.Dictionary), ContactAttribute);
        AddLinked(new("Labels", "Id", "Id", "Name", Association: new("ContactId", "LabelId")), Label, ContactLabel);
        AddEnriched(new("GenderTypeName", "GenderTypeId", "Id", "Name"), Gender);
    }

    public static IReadonlyRepository<Backend.Types.View.Contact> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.Contact>(connection, Options());
        var Address = new AddressesRepository(connection);
        var Email = new EmailsRepository(connection);
        var Phone = new PhonesRepository(connection);
        var ContactAttribute = new ContactAttributesRepository(connection);
        var Label = new LabelsRepository(connection);
        var ContactLabel = new ContactLabelsRepository(connection);
        var Gender = new GendersRepository(connection);
        repo.AddEnriched(new("Addresses", "Id", "ContactId", Type: EnrichmentPropertyType.List), Address);
        repo.AddEnriched(new("Emails", "Id", "ContactId", Type: EnrichmentPropertyType.List), Email);
        repo.AddEnriched(new("Phones", "Id", "ContactId", Type: EnrichmentPropertyType.List), Phone);
        repo.AddEnriched(new("Attributes", "Id", "ContactId", "Value", Type: EnrichmentPropertyType.Dictionary), ContactAttribute);
        repo.AddEnriched(new("Labels", "Id", "Id", "Name", Type: EnrichmentPropertyType.List), Label);
        repo.AddEnriched(new("GenderTypeName", "GenderTypeId", "Id", "Name"), Gender);
        return repo;
    }

    static CRUDRepositoryOptions<Backend.Types.View.Contact> Options() => new()
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
            ["Name"] = "name",
            ["GivenName"] = "given_name",
            ["AdditionalName"] = "additional_name",
            ["Pronunciation"] = "pronunciation",
            ["FamilyName"] = "family_name",
            ["NickName"] = "nick_name",
            ["JobTitle"] = "job_title",
            ["Organization"] = "organization",
            ["IsStarred"] = "is_starred",
            ["IsFavorite"] = "is_favorite",
            ["IsDeleted"] = "is_deleted",
            ["GenderTypeId"] = "gender_type_id",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "GivenName", "AdditionalName", "Pronunciation", "FamilyName", "NickName", "JobTitle", "Organization", "IsStarred", "IsFavorite", "IsDeleted", "GenderTypeId"],
        CreateProperties = ["Uuid", "Created", "Updated", "Version", "Name", "GivenName", "AdditionalName", "Pronunciation", "FamilyName", "NickName", "JobTitle", "Organization", "IsStarred", "IsFavorite", "IsDeleted"],
        UpdateProperties = ["Uuid", "Created", "Updated", "Version", "Name", "GivenName", "AdditionalName", "Pronunciation", "FamilyName", "NickName", "JobTitle", "Organization", "IsStarred", "IsFavorite", "IsDeleted"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, true),
            ["Updated"] = new("datetime", false, true),
            ["Version"] = new("binary", false, true),
            ["Name"] = new("string", false, false),
            ["GivenName"] = new("string", false, false),
            ["AdditionalName"] = new("string", false, true),
            ["Pronunciation"] = new("string", false, true),
            ["FamilyName"] = new("string", false, true),
            ["NickName"] = new("string", false, true),
            ["JobTitle"] = new("string", false, true),
            ["Organization"] = new("string", false, true),
            ["IsStarred"] = new("boolean", false, false),
            ["IsFavorite"] = new("boolean", false, false),
            ["IsDeleted"] = new("boolean", false, false),
            ["GenderTypeName"] = new("string", false, false),
        }),
    };
}
