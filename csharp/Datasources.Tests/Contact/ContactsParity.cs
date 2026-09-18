using Datasources;

namespace Datasources.Tests.Contact;

internal static class ContactsParity
{
    public static async Task<(IReadOnlyList<ContactView> Eager, IReadOnlyList<ContactView> Enriched)> Run(
        IConnection connection,
        bool verbose = false)
    {
        var middleware = verbose
            ? new ConsoleCommandMiddleware(options: new Dictionary<string, string> { ["isVerbose"] = "true" })
            : null;
        var rows = new List<Row>();
        var join = new Repository<Row>(new RepositoryBuilderOptions<Row>
        {
            Connection = connection,
            Entity = "contacts_base"
        });
        if (middleware is not null)
            join.AddMiddleware(middleware);
        await foreach (var row in join.Get(ContactsJoinQuery.Create()))
            rows.Add(row);
        var eager = AsContactsView.FromMapped(new FlatToTypeMapper(ContactsView.TypeMap).Collapse(rows));
        var items = new List<Contact>();
        await foreach (var item in ContactsServiceOf.Create(connection, middleware).Get())
            items.Add(item);
        return (eager, AsContactsView.FromContacts(items));
    }
}
