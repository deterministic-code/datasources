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
        var join = new CRUDRepository<Row>(connection, new CRUDRepositoryOptions<Row>
        {
            EntityMapper = ContactColumns.JoinEntity,
            PropertyMappings = ContactColumns.Columns,
        });
        if (middleware is not null)
            join.AddMiddleware(middleware);
        await foreach (var row in join.Read(ContactReads.All().Select))
            rows.Add(row);
        var eager = AsContactsView.FromMapped(new FlatToNestedMapper(ContactsView.TypeMap).Map(rows));
        var items = new List<Contact>();
        await foreach (var item in ContactsServiceOf.Create(connection, middleware).Read())
            items.Add(item);
        return (eager, AsContactsView.FromContacts(items));
    }

    internal static ISelectStatement JoinSelect(IWhere? where = null)
    {
        var select = ContactReads.All().Select;
        return where is null
            ? select
            : new SelectStatement(new()
            {
                Table = select.Table ?? "",
                Fields = select.Fields,
                Joins = select.Joins,
                Where = where,
            });
    }
}
