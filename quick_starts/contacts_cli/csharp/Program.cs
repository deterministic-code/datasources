using Backend.Types.View;
using Datasources;

var ds = Datasource.Default;
await (await ds.CreateConnection()).Open();
var contactsRepo = ds.CreateRepository<Contact>("Contacts");
var created = await contactsRepo.Create(new Contact()
{
    GivenName = "Ada",
    Emails =
    [
        new Email() { EmailTypeName = "Personal", Address = "ada@example.com", DisplayName = "Ada", IsPrimary = true },
    ],
    Phones =
    [
        new Phone() { PhoneTypeName = "Mobile", Number = "555-0100", DisplayName = "Ada mobile", Extention = "", IsPrimary = true },
    ],
    Addresses =
    [
        new Address() { AddressTypeName = "Home", Line1 = "12 Analytical Engine", City = "London", CountryCode = "GB" },
    ],
});
Console.WriteLine(
    $"created {created.Id} {created.GivenName} {created.Emails[0].Address} {created.Phones[0].Number} {created.Addresses[0].Line1}");

var loaded = await contactsRepo.ReadSingle(new Where(new Datasources.Operator.Equals("Id", created.Id)))
    ?? throw new InvalidOperationException("read");
Console.WriteLine(
    $"read {loaded.Id} {loaded.GivenName} emails={loaded.Emails.Count} phones={loaded.Phones.Count} addresses={loaded.Addresses.Count}");

await contactsRepo.Update(new Contact()
{
    Id = created.Id,
    GivenName = "Ada Byron",
    Emails = created.Emails,
    Phones = created.Phones,
    Addresses =
    [
        new Address()
        {
            Id = created.Addresses[0].Id,
            AddressTypeName = "Home",
            Line1 = created.Addresses[0].Line1,
            City = "Cambridge",
            CountryCode = "GB",
        },
    ],
});
var after = await contactsRepo.ReadSingle(new Where(new Datasources.Operator.Equals("Id", created.Id)))
    ?? throw new InvalidOperationException("update");
Console.WriteLine($"updated {after.Id} {after.GivenName} {after.Addresses[0].City}");

await contactsRepo.Delete(after);
if (await contactsRepo.ReadSingle(new Where(new Datasources.Operator.Equals("Id", created.Id))) is not null)
    throw new InvalidOperationException("delete");
Console.WriteLine("deleted");
