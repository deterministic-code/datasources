using Datasources;
using Datasources.Memory;

namespace Datasources.Tests.Contact;

public class ContactsServiceTests
{
    [Fact]
    public async Task GetFillsNamesAndChildLists()
    {
        var person = new Row { ["name"] = "person" };
        var typesConnection = await Setup(person);
        var typesRepo = CreateLookupRepo<ContactType>(typesConnection);
        var source = new Row { ["name"] = "import" };
        var sourcesConnection = await Setup(source);
        var sourcesRepo = CreateLookupRepo<ContactSource>(sourcesConnection);
        var home = new Row { ["name"] = "Home" };
        var addressTypesConnection = await Setup(home);
        var addressTypesRepo = CreateLookupRepo<AddressType>(addressTypesConnection);
        var work = new Row { ["name"] = "Work" };
        var phoneTypesConnection = await Setup(work);
        var phoneTypesRepo = CreateLookupRepo<PhoneType>(phoneTypesConnection);
        var connection = await Setup();
        var contactsRepo = Repo<Contact>(connection, ContactColumns.ContactEntity);
        var addressesRepo = Repo<Address>(connection);
        var phonesRepo = Repo<Phone>(connection);
        var service = ContactsService.Open(
            contactsRepo, typesRepo, sourcesRepo, addressTypesRepo, phoneTypesRepo, addressesRepo, phonesRepo);
        var other = await service.Create(new Contact
        {
            FirstName = "Other",
            ContactType = new() { Id = Id(person) },
            ContactSource = new() { Id = Id(source) }
        });
        Assert.True(other.Id > 0);
        var stray = await addressesRepo.Create(new Address
        {
            Contact = new() { Id = other.Id },
            Line1 = "other",
            City = "x",
            AddressType = new() { Id = Id(home) }
        });
        Assert.True(stray.Id > 0);
        var created = await service.Create(new Contact
        {
            ContactType = new() { Id = Id(person) },
            ContactSource = new() { Id = Id(source) },
            FirstName = "Ada",
            Addresses = [new Address { Line1 = "1 Main", City = "Paris", AddressType = new() { Id = Id(home) } }],
            Phones = [new Phone { Number = "555", PhoneType = new() { Id = Id(work) } }]
        });
        Assert.True(created.Id > 0);
        var contact = Assert.Single(await Read(service), item => item.FirstName == "Ada");
        Assert.Equal(created.Id, contact.Id);
        Assert.NotEqual(other.Id, contact.Id);
        Assert.Equal("person", contact.ContactType.Name);
        Assert.Equal("import", contact.ContactSource.Name);
        var address = Assert.Single(contact.Addresses);
        Assert.True(address.Id > 0);
        Assert.Equal("1 Main", address.Line1);
        Assert.Equal("Home", address.AddressType.Name);
        var phone = Assert.Single(contact.Phones);
        Assert.True(phone.Id > 0);
        Assert.Equal("555", phone.Number);
        Assert.Equal("Work", phone.PhoneType.Name);
    }

    static ReadonlyRepository<T> CreateLookupRepo<T>(IConnection connection) where T : new() =>
        new(connection, new ReadonlyRepositoryOptions<T>
        {
            EntityMapper = ContactColumns.Snake,
            PropertyMappings = ContactColumns.Columns,
        });

    static CRUDRepository<T> Repo<T>(IConnection connection, IMapper<string, string>? entity = null) where T : new() =>
        new(connection, new CRUDRepositoryOptions<T>
        {
            EntityMapper = entity ?? ContactColumns.Snake,
            PropertyMappings = ContactColumns.Columns,
            Ids = ["Id"],
        });

    static async Task<List<Contact>> Read(CRUDRepository<Contact> contacts)
    {
        var items = new List<Contact>();
        await foreach (var item in contacts.Read())
            items.Add(item);
        return items;
    }

    static async Task<IConnection> Setup(params Row[] rows)
    {
        var connection = new MemoryDriver().Connection(null);
        await connection.Open();
        foreach (var row in rows)
        {
            var response = await connection.CreateCommand().Execute(
                "insert",
                [.. row.Select(pair => new Parameter(pair.Key, pair.Value))]);
            row["id"] = response.Result;
        }
        return connection;
    }

    static int Id(Row row) => (int)((Value.Int)row["id"]).Item;
}
