using Backend.Types.View;
using Datasources;

await using var ds = Datasource.Default;
var contactsRepo = ds.CreateRepository<Contact>("Contacts");
var ada = await contactsRepo.Create(new Contact()
{
    GivenName = "Ada",
    GenderTypeName = "Female",
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
    $"created {ada.Id} {ada.GivenName} {ada.Emails[0].Address} {ada.Phones[0].Number} {ada.Addresses[0].Line1}");

var alan = await contactsRepo.Create(new Contact()
{
    GivenName = "Alan",
    GenderTypeName = "Male",
    Emails =
    [
        new Email() { EmailTypeName = "Work", Address = "alan@work.example", DisplayName = "Alan", IsPrimary = true },
    ],
    Phones =
    [
        new Phone() { PhoneTypeName = "Work", Number = "555-0200", DisplayName = "Alan office", Extention = "", IsPrimary = true },
    ],
    Addresses =
    [
        new Address() { AddressTypeName = "Work", Line1 = "1 Bletchley Park", City = "Bletchley", CountryCode = "GB" },
    ],
});
Console.WriteLine(
    $"created {alan.Id} {alan.GivenName} {alan.Emails[0].Address} {alan.Phones[0].Number} {alan.Addresses[0].Line1}");

var adaRead = await contactsRepo.ReadSingle(new Where(new Datasources.Operator.Equals("Id", ada.Id)))
    ?? throw new InvalidOperationException("read");
Console.WriteLine(
    $"read {adaRead.Id} {adaRead.GivenName} emails={adaRead.Emails.Count} phones={adaRead.Phones.Count} addresses={adaRead.Addresses.Count}");

var alanRead = await contactsRepo.ReadSingle(new Where(new Datasources.Operator.Equals("Id", alan.Id)))
    ?? throw new InvalidOperationException("read");
Console.WriteLine(
    $"read {alanRead.Id} {alanRead.GivenName} emails={alanRead.Emails.Count} phones={alanRead.Phones.Count} addresses={alanRead.Addresses.Count}");

var adaSaved = await contactsRepo.Update(new Contact()
{
    Id = ada.Id,
    GivenName = "Ada Byron",
    GenderTypeName = "Female",
    Emails =
    [
        new Email()
        {
            Id = ada.Emails[0].Id,
            EmailTypeName = "Personal",
            Address = ada.Emails[0].Address,
            DisplayName = "Ada",
            IsPrimary = true,
        },
        new Email() { EmailTypeName = "Work", Address = "ada@byron.example", DisplayName = "Ada work", IsPrimary = false },
    ],
    Phones =
    [
        new Phone()
        {
            Id = ada.Phones[0].Id,
            PhoneTypeName = "Mobile",
            Number = "555-0111",
            DisplayName = "Ada mobile",
            Extention = "",
            IsPrimary = true,
        },
    ],
    Addresses =
    [
        new Address()
        {
            Id = ada.Addresses[0].Id,
            AddressTypeName = "Home",
            Line1 = ada.Addresses[0].Line1,
            City = "Cambridge",
            CountryCode = "GB",
        },
    ],
});
var adaAfter = await contactsRepo.ReadSingle(new Where(new Datasources.Operator.Equals("Id", ada.Id)))
    ?? throw new InvalidOperationException("update");
Console.WriteLine(
    $"updated {adaAfter.Id} {adaAfter.GivenName} rows={adaSaved.RowsAffected} emails={string.Join(",", adaAfter.Emails.OrderBy(email => email.Address).Select(email => email.Address))} phones={string.Join(",", adaAfter.Phones.OrderBy(phone => phone.Number).Select(phone => phone.Number))} {adaAfter.Addresses[0].City}");

var alanSaved = await contactsRepo.Update(new Contact()
{
    Id = alan.Id,
    GivenName = "Alan",
    GenderTypeName = "Male",
    Emails =
    [
        new Email()
        {
            Id = alan.Emails[0].Id,
            EmailTypeName = "Work",
            Address = "alan@bletchley.example",
            DisplayName = "Alan",
            IsPrimary = true,
        },
    ],
    Phones =
    [
        new Phone()
        {
            Id = alan.Phones[0].Id,
            PhoneTypeName = "Work",
            Number = "555-0201",
            DisplayName = "Alan office",
            Extention = "",
            IsPrimary = true,
        },
    ],
    Addresses =
    [
        new Address()
        {
            Id = alan.Addresses[0].Id,
            AddressTypeName = "Work",
            Line1 = "2 Hut 8",
            City = alan.Addresses[0].City,
            CountryCode = "GB",
        },
    ],
});
var alanAfter = await contactsRepo.ReadSingle(new Where(new Datasources.Operator.Equals("Id", alan.Id)))
    ?? throw new InvalidOperationException("update");
Console.WriteLine(
    $"updated {alanAfter.Id} {alanAfter.GivenName} rows={alanSaved.RowsAffected} emails={string.Join(",", alanAfter.Emails.Select(email => email.Address))} phones={string.Join(",", alanAfter.Phones.Select(phone => phone.Number))} {alanAfter.Addresses[0].Line1}");

var adaRemoved = await contactsRepo.Delete(adaAfter);
if (await contactsRepo.ReadSingle(new Where(new Datasources.Operator.Equals("Id", ada.Id))) is not null)
    throw new InvalidOperationException("delete");
Console.WriteLine($"deleted {ada.Id} rows={adaRemoved.RowsAffected}");

var alanRemoved = await contactsRepo.Delete(alanAfter);
if (await contactsRepo.ReadSingle(new Where(new Datasources.Operator.Equals("Id", alan.Id))) is not null)
    throw new InvalidOperationException("delete");
Console.WriteLine($"deleted {alan.Id} rows={alanRemoved.RowsAffected}");
