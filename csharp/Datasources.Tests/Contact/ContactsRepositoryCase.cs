using Datasources;

namespace Datasources.Tests.Contact;

internal static class ContactsRepositoryCase
{
    public static async Task MatchesEnrichedToFlatMapper(
        IConnection connection,
        ICommand command,
        string dialect,
        string version = "X''")
    {
        foreach (var sql in ContactSchema.Statements(ContactSchema.Read(dialect)).Concat(SeedContacts.Sql(version: version)))
            await command.Execute(sql);

        var verbose = Environment.GetEnvironmentVariable("VERBOSE") is "1" or "true";
        var (eager, enriched) = await ContactsParity.Run(connection, verbose);
        var mapped = eager.First(contact => contact.Id == 10);
        if (verbose) Console.WriteLine(mapped);

        ExpectContactsView.Of(eager, SeedContacts.Count);
        ExpectContactsView.Of(enriched, SeedContacts.Count);
        Assert.False(string.IsNullOrEmpty(mapped.ContactSourceName));
        Assert.False(string.IsNullOrEmpty(mapped.ContactTypeName));
        Assert.Equal(2, mapped.Addresses.DistinctBy(address => address.Id).Count());
        Assert.Equal(AsContactsView.Of(enriched), AsContactsView.Of(eager));

        var extra = await ExtraContact.Add(ContactsServiceOf.Create(connection));
        Assert.Equal("Extra", extra.first_name);
        Assert.Single(extra.addresses);
        Assert.Equal("9 Extra", extra.addresses[0].line1);
        Assert.Equal(extra.id, extra.addresses[0].contact_id);
        Assert.Equal("555-extra", Assert.Single(extra.phones).number);
        Assert.Equal(extra.id, extra.phones[0].contact_id);
    }
}
