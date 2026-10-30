using Datasources;

public sealed class AddressRepository : CRUDRepository<Backend.Types.View.Address>
{
    public AddressTypeRepository AddressType { get; }

    public AddressRepository(IConnection connection) : base(connection, Options(connection))
    {
        AddressType = new AddressTypeRepository(connection);
        AddEnriched(new("AddressTypeName", "AddressTypeId", "Id", "Name"), AddressType);
    }

    public static IReadonlyRepository<Backend.Types.View.Address> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.Address>(connection, Options(connection));
        var AddressType = new AddressTypeRepository(connection);
        repo.AddEnriched(new("AddressTypeName", "AddressTypeId", "Id", "Name"), AddressType);
        return repo;
    }

    static CRUDRepositoryOptions<Backend.Types.View.Address> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.View.Address>()
            .Add(new()
            {
                Entity = "Address",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Address"] = "address_bases" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["ContactId"] = "contact_id",
                    ["AddressTypeId"] = "address_type_id",
                    ["Line1"] = "line1",
                    ["Line2"] = "line2",
                    ["City"] = "city",
                    ["Region"] = "region",
                    ["PostalCode"] = "postal_code",
                    ["Country"] = "country",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "AddressTypeId", "Line1", "Line2", "City", "Region", "PostalCode", "Country"],
                CreateProperties = ["Uuid", "Created", "Updated", "Version", "ContactId", "AddressTypeId", "Line1", "Line2", "City", "Region", "PostalCode", "Country"],
                UpdateProperties = ["Uuid", "Created", "Updated", "Version", "ContactId", "AddressTypeId", "Line1", "Line2", "City", "Region", "PostalCode", "Country"],
            })
            .Add("sqlite", new()
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
                    ["Line1"] = "line1",
                    ["Line2"] = "line2",
                    ["City"] = "city",
                    ["Region"] = "region",
                    ["PostalCode"] = "postal_code",
                    ["Country"] = "country",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "AddressTypeId", "Line1", "Line2", "City", "Region", "PostalCode", "Country"],
                CreateProperties = ["Uuid", "Version", "ContactId", "AddressTypeId", "Line1", "Line2", "City", "Region", "PostalCode", "Country"],
                UpdateProperties = ["Uuid", "Version", "ContactId", "AddressTypeId", "Line1", "Line2", "City", "Region", "PostalCode", "Country"],
            })
            .Add("mysql", new()
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
                    ["Line1"] = "line1",
                    ["Line2"] = "line2",
                    ["City"] = "city",
                    ["Region"] = "region",
                    ["PostalCode"] = "postal_code",
                    ["Country"] = "country",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "AddressTypeId", "Line1", "Line2", "City", "Region", "PostalCode", "Country"],
                CreateProperties = ["ContactId", "AddressTypeId", "Line1", "Line2", "City", "Region", "PostalCode", "Country"],
                UpdateProperties = ["Uuid", "Version", "ContactId", "AddressTypeId", "Line1", "Line2", "City", "Region", "PostalCode", "Country"],
            })
            .Add("postgres", new()
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
                    ["Line1"] = "line1",
                    ["Line2"] = "line2",
                    ["City"] = "city",
                    ["Region"] = "region",
                    ["PostalCode"] = "postal_code",
                    ["Country"] = "country",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "AddressTypeId", "Line1", "Line2", "City", "Region", "PostalCode", "Country"],
                CreateProperties = ["ContactId", "AddressTypeId", "Line1", "Line2", "City", "Region", "PostalCode", "Country"],
                UpdateProperties = ["Uuid", "Version", "ContactId", "AddressTypeId", "Line1", "Line2", "City", "Region", "PostalCode", "Country"],
            })
            .For(connection);
}
