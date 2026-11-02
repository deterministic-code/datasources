using Datasources;

public sealed class ContactRepository : CRUDRepository<Backend.Types.View.Contact>
{
    public AddressRepository Address { get; }
    public EmailRepository Email { get; }
    public PhoneRepository Phone { get; }
    public ContactAttributeRepository ContactAttribute { get; }
    public LabelRepository Label { get; }
    public ContactLabelRepository ContactLabel { get; }
    public GenderRepository Gender { get; }

    public ContactRepository(IConnection connection) : base(connection, Options())
    {
        Address = new AddressRepository(connection);
        Email = new EmailRepository(connection);
        Phone = new PhoneRepository(connection);
        ContactAttribute = new ContactAttributeRepository(connection);
        Label = new LabelRepository(connection);
        ContactLabel = new ContactLabelRepository(connection);
        Gender = new GenderRepository(connection);
        AddEnriched(new("Addresses", "Id", "ContactId", Type: EnrichmentPropertyType.List), Address);
        AddEnriched(new("Emails", "Id", "ContactId", Type: EnrichmentPropertyType.List), Email);
        AddEnriched(new("Phones", "Id", "ContactId", Type: EnrichmentPropertyType.List), Phone);
        AddEnriched(new("Attributes", "Id", "ContactId", "Value", Type: EnrichmentPropertyType.Dictionary), ContactAttribute);
        AddLinked(new("Labels", "Id", "Id", Association: new("ContactId", "LabelId")), Label, ContactLabel);
        AddEnriched(new("GenderTypeName", "GenderTypeId", "Id", "Name"), Gender);
    }

    public static IReadonlyRepository<Backend.Types.View.Contact> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.Contact>(connection, Options());
        var Address = new AddressRepository(connection);
        var Email = new EmailRepository(connection);
        var Phone = new PhoneRepository(connection);
        var ContactAttribute = new ContactAttributeRepository(connection);
        var Label = new LabelRepository(connection);
        var ContactLabel = new ContactLabelRepository(connection);
        var Gender = new GenderRepository(connection);
        repo.AddEnriched(new("Addresses", "Id", "ContactId", Type: EnrichmentPropertyType.List), Address);
        repo.AddEnriched(new("Emails", "Id", "ContactId", Type: EnrichmentPropertyType.List), Email);
        repo.AddEnriched(new("Phones", "Id", "ContactId", Type: EnrichmentPropertyType.List), Phone);
        repo.AddEnriched(new("Attributes", "Id", "ContactId", "Value", Type: EnrichmentPropertyType.Dictionary), ContactAttribute);
        repo.AddEnriched(new("Labels", "Id", "Id", Type: EnrichmentPropertyType.List), Label);
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
            ["GenderTypeId"] = "gender_type_id",
            ["Organization"] = "organization",
            ["IsStarred"] = "is_starred",
            ["IsFavorite"] = "is_favorite",
            ["IsDeleted"] = "is_deleted",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "GivenName", "AdditionalName", "Pronunciation", "FamilyName", "NickName", "JobTitle", "GenderTypeId", "Organization", "IsStarred", "IsFavorite", "IsDeleted"],
        CreateProperties = ["Uuid", "Created", "Updated", "Version", "Name", "GivenName", "AdditionalName", "Pronunciation", "FamilyName", "NickName", "JobTitle", "GenderTypeId", "Organization", "IsStarred", "IsFavorite", "IsDeleted"],
        UpdateProperties = ["Uuid", "Created", "Updated", "Version", "Name", "GivenName", "AdditionalName", "Pronunciation", "FamilyName", "NickName", "JobTitle", "GenderTypeId", "Organization", "IsStarred", "IsFavorite", "IsDeleted"],
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
            ["GenderTypeId"] = new("integer", false, false),
            ["Organization"] = new("string", false, true),
            ["IsStarred"] = new("boolean", false, false),
            ["IsFavorite"] = new("boolean", false, false),
            ["IsDeleted"] = new("boolean", false, false),
        }),
    };
}
