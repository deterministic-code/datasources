using Datasources;

namespace Datasources.Tests.Contact;

internal static class ContactsParity
{
    public static async Task<(IReadOnlyList<ContactView> Eager, IReadOnlyList<ContactView> Enriched)> Run(
        IConnection connection, bool verbose)
    {
        var middleware = new ConsoleCommandMiddleware(options: new Dictionary<string, string>
        {
            ["isVerbose"] = verbose ? "true" : "false"
        });
        var eager = FromMappedContacts.Of(new FlatToTypeMapper(ContactsViewMap.Spec).Collapse(
            await Collect(new Repository<Row>(new RepositoryBuilderOptions<Row>
            {
                Connection = connection,
                Entity = "contacts_base"
            }).Use(middleware).Get(ContactsJoinQuery.Create()))));
        var enriched = FromEnrichedContacts.Of(
            await Collect(ContactsServiceOf.Create(connection, verbose ? middleware : null).Get()));
        return (eager, enriched);
    }

    static async Task<List<T>> Collect<T>(IAsyncEnumerable<T> items)
    {
        var list = new List<T>();
        await foreach (var item in items)
            list.Add(item);
        return list;
    }
}
