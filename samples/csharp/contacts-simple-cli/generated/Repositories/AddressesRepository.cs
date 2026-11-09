using Datasources;

public sealed class AddressesRepository : CRUDRepository<Backend.Types.View.Address>
{
    public AddressAttributesRepository AddressAttribute { get; }
    public LabelsRepository Label { get; }
    public AddressLabelsRepository AddressLabel { get; }
    public AddressTypesRepository AddressType { get; }

    public AddressesRepository(IConnection connection) : base(connection, Options())
    {
        AddressAttribute = new AddressAttributesRepository(connection);
        Label = new LabelsRepository(connection);
        AddressLabel = new AddressLabelsRepository(connection);
        AddressType = new AddressTypesRepository(connection);
        AddEnriched(new("Attributes", "Id", "AddressId", "Value", Type: EnrichmentPropertyType.Dictionary), AddressAttribute);
        AddLinked(new("Labels", "Id", "Id", "Name", Association: new("AddressId", "LabelId")), Label, AddressLabel);
        AddEnriched(new("AddressTypeName", "AddressTypeId", "Id", "Name"), AddressType);
        AddEnriched(new("Description", "AddressTypeId", "Id", "Description"), AddressType);
    }

    public static IReadonlyRepository<Backend.Types.View.Address> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.Address>(connection, Options());
        var AddressAttribute = new AddressAttributesRepository(connection);
        var Label = new LabelsRepository(connection);
        var AddressLabel = new AddressLabelsRepository(connection);
        var AddressType = new AddressTypesRepository(connection);
        repo.AddEnriched(new("Attributes", "Id", "AddressId", "Value", Type: EnrichmentPropertyType.Dictionary), AddressAttribute);
        repo.AddEnriched(new("Labels", "Id", "Id", "Name", Type: EnrichmentPropertyType.List), Label);
        repo.AddEnriched(new("AddressTypeName", "AddressTypeId", "Id", "Name"), AddressType);
        repo.AddEnriched(new("Description", "AddressTypeId", "Id", "Description"), AddressType);
        return repo;
    }

    static CRUDRepositoryOptions<Backend.Types.View.Address> Options() => new()
    {
        Entity = "Address",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Address"] = "addresses" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Uuid"] = "uuid",
            ["Created"] = "created",
            ["Updated"] = "updated",
            ["Version"] = "version",
            ["ContactId"] = "contact_id",
            ["DisplayName"] = "display_name",
            ["Line1"] = "line_1",
            ["Line2"] = "line_2",
            ["Line3"] = "line_3",
            ["Neighborhood"] = "neighborhood",
            ["District"] = "district",
            ["City"] = "city",
            ["StateProvince"] = "state_province",
            ["PostalCode"] = "postal_code",
            ["CountryCode"] = "country_code",
            ["FormattedAddress"] = "formatted_address",
            ["Latitude"] = "latitude",
            ["Longitude"] = "longitude",
            ["IsPrimary"] = "is_primary",
            ["AddressTypeId"] = "address_type_id",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "DisplayName", "Line1", "Line2", "Line3", "Neighborhood", "District", "City", "StateProvince", "PostalCode", "CountryCode", "FormattedAddress", "Latitude", "Longitude", "IsPrimary", "AddressTypeId"],
        CreateProperties = ["Uuid", "Created", "Updated", "Version", "ContactId", "DisplayName", "Line1", "Line2", "Line3", "Neighborhood", "District", "City", "StateProvince", "PostalCode", "CountryCode", "FormattedAddress", "Latitude", "Longitude", "IsPrimary"],
        UpdateProperties = ["Uuid", "Created", "Updated", "Version", "ContactId", "DisplayName", "Line1", "Line2", "Line3", "Neighborhood", "District", "City", "StateProvince", "PostalCode", "CountryCode", "FormattedAddress", "Latitude", "Longitude", "IsPrimary"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, true),
            ["Updated"] = new("datetime", false, true),
            ["Version"] = new("binary", false, true),
            ["ContactId"] = new("integer", false, false),
            ["DisplayName"] = new("string", false, true),
            ["Line1"] = new("string", false, false),
            ["Line2"] = new("string", false, true),
            ["Line3"] = new("string", false, true),
            ["Neighborhood"] = new("string", false, true),
            ["District"] = new("string", false, true),
            ["City"] = new("string", false, false),
            ["StateProvince"] = new("string", false, true),
            ["PostalCode"] = new("string", false, true),
            ["CountryCode"] = new("string", false, false),
            ["FormattedAddress"] = new("string", false, true),
            ["Latitude"] = new("integer", true, false),
            ["Longitude"] = new("integer", true, false),
            ["IsPrimary"] = new("boolean", false, true),
            ["AddressTypeName"] = new("string", false, false),
            ["Description"] = new("string", false, true),
        }),
    };
}
