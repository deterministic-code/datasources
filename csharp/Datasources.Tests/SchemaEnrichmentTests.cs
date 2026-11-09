using Datasources;
using Datasources.Memory;

namespace Datasources.Tests;

public class SchemaEnrichmentTests
{
    sealed class NamedType
    {
        public long Id { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class Address
    {
        public long Id { get; set; }
        public long ContactId { get; set; }
        public string Line1 { get; set; } = "";
        public long AddressTypeId { get; set; }
        public string AddressTypeName { get; set; } = "";
    }

    sealed class Setting
    {
        public long Id { get; set; }
        public long ContactId { get; set; }
        public string Key { get; set; } = "";
        public string Value { get; set; } = "";
    }

    sealed class Contact
    {
        public long Id { get; set; }
        public long ContactSourceId { get; set; }
        public string ContactSourceName { get; set; } = "";
        public List<Address> Addresses { get; set; } = [];
        public Dictionary<string, string> Settings { get; set; } = [];
    }

    sealed class BigId
    {
        public long Id { get; set; }
        public string Name { get; set; } = "";
    }

    static readonly IMapper<string, string> Columns = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);

    [Fact]
    public async Task CreateResolvesAddressTypeNameWithoutAddressTypeIdInSchema()
    {
        var connection = await Open();
        var types = Types(connection, "AddressType");
        var addresses = Addresses(connection);
        var home = await types.Create(new NamedType { Name = "Home" });
        addresses.AddEnriched(new("AddressTypeName", "AddressTypeId", "Id", "Name"), types);
        var created = await addresses.Create(new Address { Line1 = "1 Main", AddressTypeName = "Home" });
        Assert.True(created.Id > 0);
        var loaded = await addresses.ReadSingle();
        Assert.Equal("1 Main", loaded!.Line1);
        Assert.Equal("Home", loaded.AddressTypeName);
        Assert.Equal(home.Id, loaded.AddressTypeId);
    }

    [Fact]
    public async Task CreateKeepsAddressTypeIdWhenNameIsEmpty()
    {
        var connection = await Open();
        var types = Types(connection, "AddressType");
        var addresses = Addresses(connection);
        var home = await types.Create(new NamedType { Name = "Home" });
        addresses.AddEnriched(new("AddressTypeName", "AddressTypeId", "Id", "Name"), types);
        var created = await addresses.Create(new Address { Line1 = "1 Main", AddressTypeId = home.Id });
        Assert.True(created.Id > 0);
        Assert.Equal(home.Id, (await addresses.ReadSingle())!.AddressTypeId);
    }

    [Fact]
    public async Task CreateContactStampsChildContactIdAndResolvesNames()
    {
        var (contacts, addresses) = await Graph();
        var created = await contacts.Create(new Contact
        {
            ContactSourceName = "import",
            Addresses = [new Address { Line1 = "1 Main", AddressTypeName = "Home" }],
            Settings = { ["theme"] = "dark" },
        });
        Assert.True(created.Id > 0);
        var loaded = await contacts.ReadSingle();
        Assert.Equal("import", loaded!.ContactSourceName);
        Assert.Equal("dark", loaded.Settings["theme"]);
        var address = Assert.Single(loaded.Addresses);
        Assert.Equal("1 Main", address.Line1);
        Assert.Equal("Home", address.AddressTypeName);
        Assert.Equal(created.Id, address.ContactId);
        Assert.Equal(created.Id, Assert.Single(await addresses.Read().ToListAsync()).ContactId);
    }

    [Fact]
    public async Task CreateUnknownContactSourceNameThrows()
    {
        var (contacts, _) = await Graph();
        var error = await Assert.ThrowsAsync<DatasourceException>(() =>
            contacts.Create(new Contact { ContactSourceName = "Nope" }));
        Assert.Contains("ContactSourceName", error.Message);
        Assert.Contains("Nope", error.Message);
    }

    [Fact]
    public async Task CreateMissingContactSourceNameThrows()
    {
        var (contacts, _) = await Graph();
        var error = await Assert.ThrowsAsync<DatasourceException>(() => contacts.Create(new Contact()));
        Assert.Contains("ContactSourceName", error.Message);
    }

    [Fact]
    public async Task CreateUnknownNestedAddressTypeNameThrows()
    {
        var (contacts, _) = await Graph();
        var error = await Assert.ThrowsAsync<DatasourceException>(() =>
            contacts.Create(new Contact
            {
                ContactSourceName = "import",
                Addresses = [new Address { Line1 = "1 Main", AddressTypeName = "Nope" }],
            }));
        Assert.Contains("AddressTypeName", error.Message);
        Assert.Contains("Nope", error.Message);
    }

    [Fact]
    public async Task CreateKeepsUnknownContactSourceIdWhenNameIsEmpty()
    {
        var (contacts, _) = await Graph();
        var created = await contacts.Create(new Contact { ContactSourceId = 99 });
        Assert.True(created.Id > 0);
        Assert.Equal(99, (await contacts.ReadSingle())!.ContactSourceId);
    }

    [Fact]
    public async Task CreateWritesBigintegerId()
    {
        var connection = await Open();
        var repo = new CRUDRepository<BigId>(connection, new CRUDRepositoryOptions<BigId>
        {
            Ids = ["Id"],
            Schema = new Schema(new Dictionary<string, SchemaColumn>
            {
                ["Id"] = new("biginteger", IsId: true),
                ["Name"] = new("string"),
            }),
        });
        var created = await repo.Create(new BigId { Name = "Ada" });
        Assert.True(created.Id > 0);
        Assert.Equal("Ada", (await repo.ReadSingle())!.Name);
    }

    static async Task<(CRUDRepository<Contact> Contacts, CRUDRepository<Address> Addresses)> Graph()
    {
        var connection = await Open();
        var sources = Types(connection, "ContactSource");
        var types = Types(connection, "AddressType");
        await sources.Create(new NamedType { Name = "import" });
        await types.Create(new NamedType { Name = "Home" });
        var addresses = Addresses(connection);
        addresses.AddEnriched(new("AddressTypeName", "AddressTypeId", "Id", "Name"), types);
        var settings = new CRUDRepository<Setting>(connection, new CRUDRepositoryOptions<Setting>
        {
            Ids = ["Id"],
            PropertyMappings = Columns,
        });
        var contacts = new CRUDRepository<Contact>(connection, new CRUDRepositoryOptions<Contact>
        {
            Ids = ["Id"],
            PropertyMappings = Columns,
            ReadProperties = ["Id", "ContactSourceId"],
            CreateProperties = [],
            Schema = new Schema(new Dictionary<string, SchemaColumn>
            {
                ["Id"] = new("integer", IsId: true),
                ["ContactSourceName"] = new("string"),
            }),
        });
        contacts.AddEnriched(new("Addresses", "Id", "ContactId", Type: EnrichmentPropertyType.List), addresses);
        contacts.AddEnriched(new("Settings", "Id", "ContactId", "Value", EnrichmentPropertyType.Dictionary), settings);
        contacts.AddEnriched(new("ContactSourceName", "ContactSourceId", "Id", "Name"), sources);
        return (contacts, addresses);
    }

    static CRUDRepository<NamedType> Types(IConnection connection, string entity) =>
        new(connection, new CRUDRepositoryOptions<NamedType>
        {
            Entity = entity,
            Ids = ["Id"],
            PropertyMappings = Columns,
        });

    static CRUDRepository<Address> Addresses(IConnection connection) =>
        new(connection, new CRUDRepositoryOptions<Address>
        {
            Ids = ["Id"],
            PropertyMappings = Columns,
            ReadProperties = ["Id", "ContactId", "Line1", "AddressTypeId"],
            CreateProperties = ["ContactId", "Line1"],
            Schema = new Schema(new Dictionary<string, SchemaColumn>
            {
                ["Id"] = new("integer", IsId: true),
                ["ContactId"] = new("integer"),
                ["Line1"] = new("string"),
                ["AddressTypeName"] = new("string"),
            }),
        });

    static async Task<IConnection> Open()
    {
        var connection = new MemoryDriver().Connection(null);
        await connection.Open();
        return connection;
    }
}
