using Datasources;

namespace Datasources.Tests.Contact;

public class ContactsServiceTests
{
    [Fact]
    public async Task GetFillsNamesAndChildLists()
    {
        var contacts = new ListCRUDRepository<Contact>();
        var types = new ListReadonlyRepository<ContactType>(
            new ContactType { id = 2, name = "person" });
        var sources = new ListReadonlyRepository<ContactSource>(
            new ContactSource { id = 3, name = "import" });
        var addresses = new ListCRUDRepository<Address>(
            new Address { id = 11, contact_id = 9, line1 = "other", city = "x" });
        var phones = new ListCRUDRepository<Phone>();
        var service = new ContactsService(contacts, types, sources, addresses, phones);
        await service.Create(new Contact
        {
            contact_type_id = 2,
            contact_source_id = 3,
            first_name = "Ada",
            addresses = [new Address { line1 = "1 Main", city = "Paris" }],
            phones = [new Phone { number = "555" }]
        });
        var items = new List<Contact>();
        await foreach (var item in service.Get())
            items.Add(item);
        var contact = Assert.Single(items);
        Assert.Equal("person", contact.contact_type_name);
        Assert.Equal("import", contact.contact_source_name);
        Assert.Single(contact.addresses);
        Assert.Equal("1 Main", contact.addresses[0].line1);
        Assert.Equal("555", Assert.Single(contact.phones).number);
    }
}
