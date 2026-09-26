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
        var typesRepo = new ReadonlyRepository<ContactType>(typesConnection);
        var source = new Row { ["name"] = "import" };
        var sourcesConnection = await Setup(source);
        var sourcesRepo = new ReadonlyRepository<ContactSource>(sourcesConnection);
        var home = new Row { ["name"] = "Home" };
        var addressTypesConnection = await Setup(home);
        var addressTypesRepo = new ReadonlyRepository<AddressType>(addressTypesConnection);
        var work = new Row { ["name"] = "Work" };
        var phoneTypesConnection = await Setup(work);
        var phoneTypesRepo = new ReadonlyRepository<PhoneType>(phoneTypesConnection);
        var connection = await Setup();
        var contactsRepo = new CRUDRepository<Contact>(connection);
        var addressesRepo = new CRUDRepository<Address>(connection);
        var phonesRepo = new CRUDRepository<Phone>(connection);
        var service = ContactsService.Open(
            contactsRepo, typesRepo, sourcesRepo, addressTypesRepo, phoneTypesRepo, addressesRepo, phonesRepo);
        await service.Create(new Contact
        {
            first_name = "Other",
            contact_type_id = Id(person),
            contact_source_id = Id(source)
        });
        var other = Assert.Single(await Read(service));
        Assert.True(other.id > 0);
        await addressesRepo.Create(new Address
        {
            contact_id = other.id,
            line1 = "other",
            city = "x",
            address_type_id = Id(home)
        });
        await service.Create(new Contact
        {
            contact_type_id = Id(person),
            contact_source_id = Id(source),
            first_name = "Ada",
            addresses = [new Address { line1 = "1 Main", city = "Paris", address_type_id = Id(home) }],
            phones = [new Phone { number = "555", phone_type_id = Id(work) }]
        });
        var contact = Assert.Single(await Read(service), item => item.first_name == "Ada");
        Assert.True(contact.id > 0);
        Assert.NotEqual(other.id, contact.id);
        Assert.Equal("person", contact.contact_type_name);
        Assert.Equal("import", contact.contact_source_name);
        var address = Assert.Single(contact.addresses);
        Assert.True(address.id > 0);
        Assert.Equal("1 Main", address.line1);
        Assert.Equal("Home", address.address_type_name);
        var phone = Assert.Single(contact.phones);
        Assert.True(phone.id > 0);
        Assert.Equal("555", phone.number);
        Assert.Equal("Work", phone.phone_type_name);
    }

    static async Task<List<Contact>> Read(CRUDRepository<Contact> contacts)
    {
        var items = new List<Contact>();
        await foreach (var item in contacts.Read())
            items.Add(item);
        return items;
    }

    static async Task<IConnection> Setup(params Row[] rows)
    {
        var connection = new MemoryProvider().Connection(null);
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
