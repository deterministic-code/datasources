using Datasources;

public sealed class AddressRepository : CRUDRepository<Backend.Types.View.Address>
{
    public AddressAttributeRepository AddressAttribute { get; }
    public LabelRepository Label { get; }
    public AddressLabelRepository AddressLabel { get; }
    public AddressTypeRepository AddressType { get; }

    public AddressRepository(IConnection connection) : base(connection, Options())
    {
        AddressAttribute = new AddressAttributeRepository(connection);
        Label = new LabelRepository(connection);
        AddressLabel = new AddressLabelRepository(connection);
        AddressType = new AddressTypeRepository(connection);
        AddEnriched(new("Attributes", "Id", "AddressId", "Value", Type: EnrichmentPropertyType.Dictionary), AddressAttribute);
        AddLinked(new("Labels", "Id", "Id", Association: new("AddressId", "LabelId")), Label, AddressLabel);
        AddEnriched(new("AddressTypeName", "AddressTypeId", "Id", "Name"), AddressType);
    }

    public static IReadonlyRepository<Backend.Types.View.Address> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.Address>(connection, Options());
        var AddressAttribute = new AddressAttributeRepository(connection);
        var Label = new LabelRepository(connection);
        var AddressLabel = new AddressLabelRepository(connection);
        var AddressType = new AddressTypeRepository(connection);
        repo.AddEnriched(new("Attributes", "Id", "AddressId", "Value", Type: EnrichmentPropertyType.Dictionary), AddressAttribute);
        repo.AddEnriched(new("Labels", "Id", "Id", Type: EnrichmentPropertyType.List), Label);
        repo.AddEnriched(new("AddressTypeName", "AddressTypeId", "Id", "Name"), AddressType);
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
            ["AddressTypeId"] = "address_type_id",
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
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "AddressTypeId", "DisplayName", "Line1", "Line2", "Line3", "Neighborhood", "District", "City", "StateProvince", "PostalCode", "CountryCode", "FormattedAddress", "Latitude", "Longitude", "IsPrimary"],
        CreateProperties = ["Uuid", "Created", "Updated", "Version", "ContactId", "AddressTypeId", "DisplayName", "Line1", "Line2", "Line3", "Neighborhood", "District", "City", "StateProvince", "PostalCode", "CountryCode", "FormattedAddress", "Latitude", "Longitude", "IsPrimary"],
        UpdateProperties = ["Uuid", "Created", "Updated", "Version", "ContactId", "AddressTypeId", "DisplayName", "Line1", "Line2", "Line3", "Neighborhood", "District", "City", "StateProvince", "PostalCode", "CountryCode", "FormattedAddress", "Latitude", "Longitude", "IsPrimary"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, true),
            ["Updated"] = new("datetime", false, true),
            ["Version"] = new("binary", false, true),
            ["ContactId"] = new("integer", false, false),
            ["AddressTypeId"] = new("integer", false, false),
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
        }),
    };
}
