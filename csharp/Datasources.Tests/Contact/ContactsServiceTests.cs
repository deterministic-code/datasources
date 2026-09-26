using Datasources;
using Datasources.Memory;

namespace Datasources.Tests.Contact;

public class ContactsServiceTests
{
    [Fact]
    public async Task GetFillsNamesAndChildLists()
    {
        var contacts = new ListCRUDRepository<Contact>();
        var typesRepo = new ReadonlyRepository<ContactType>(await Open(new Row { ["id"] = 2, ["name"] = "person" }));
        var sourcesRepo = new ReadonlyRepository<ContactSource>(await Open(new Row { ["id"] = 3, ["name"] = "import" }));
        var addressTypesRepo = new ReadonlyRepository<AddressType>(await Open(new Row { ["id"] = 1, ["name"] = "Home" }));
        var phoneTypesRepo = new ReadonlyRepository<PhoneType>(await Open(new Row { ["id"] = 2, ["name"] = "Work" }));
        var addresses = new ListCRUDRepository<Address>(
            new Address { id = 11, contact_id = 9, line1 = "other", city = "x" });
        var phones = new ListCRUDRepository<Phone>();
        var service = ContactsService.Open(
            contacts, typesRepo, sourcesRepo, addressTypesRepo, phoneTypesRepo, addresses, phones);
        await service.Create(new Contact
        {
            contact_type_id = 2,
            contact_source_id = 3,
            first_name = "Ada",
            addresses = [new Address { line1 = "1 Main", city = "Paris", address_type_id = 1 }],
            phones = [new Phone { number = "555", phone_type_id = 2 }]
        });
        var items = new List<Contact>();
        await foreach (var item in service.Read())
            items.Add(item);
        var contact = Assert.Single(items);
        Assert.Equal("person", contact.contact_type_name);
        Assert.Equal("import", contact.contact_source_name);
        Assert.Single(contact.addresses);
        Assert.Equal("1 Main", contact.addresses[0].line1);
        Assert.Equal("Home", contact.addresses[0].address_type_name);
        Assert.Equal("555", Assert.Single(contact.phones).number);
        Assert.Equal("Work", contact.phones[0].phone_type_name);
    }

    static async Task<IConnection> Open(params Row[] rows)
    {
        var connection = new MemoryProvider().Connection(null);
        await connection.Open();
        foreach (var row in rows)
            await connection.CreateCommand().Execute(
                "insert",
                [.. row.Select(pair => new Parameter(pair.Key, pair.Value))]);
        return connection;
    }
}
