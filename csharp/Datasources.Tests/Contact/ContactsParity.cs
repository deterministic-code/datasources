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
        await foreach (var row in join.Read(JoinSelect()))
            rows.Add(row);
        var eager = AsContactsView.FromMapped(new FlatToNestedMapper(ContactsView.TypeMap).Map(rows));
        var items = new List<Contact>();
        await foreach (var item in ContactsServiceOf.Create(connection, middleware).Read())
            items.Add(item);
        return (eager, AsContactsView.FromContacts(items));
    }

    internal static ISelectStatement JoinSelect(IWhere? where = null) =>
        new SelectStatement(new()
        {
            Table = "Contact",
            Fields =
            [
                "Contact.Id AS Id",
                "Contact.FirstName AS FirstName",
                "Contact.LastName AS LastName",
                "Contact.Email AS Email",
                "ContactSource.Name AS ContactSourceName",
                "ContactType.Name AS ContactTypeName",
                "Address.Id AS AddressId",
                "Address.Line1 AS Line1",
                "Address.City AS City",
                "AddressType.Name AS AddressTypeName",
                "Phone.Id AS PhoneId",
                "Phone.Number AS Number",
                "PhoneType.Name AS PhoneTypeName",
            ],
            Joins =
            [
                Join.Inner("ContactSource", "Contact.ContactSourceId", "ContactSource.Id"),
                Join.Inner("ContactType", "Contact.ContactTypeId", "ContactType.Id"),
                Join.Left("Address", "Contact.Id", "Address.ContactId"),
                Join.Left("AddressType", "Address.AddressTypeId", "AddressType.Id"),
                Join.Left("Phone", "Contact.Id", "Phone.ContactId"),
                Join.Left("PhoneType", "Phone.PhoneTypeId", "PhoneType.Id"),
            ],
            Where = where,
        });
}
