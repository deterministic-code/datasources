using Datasources;

namespace Datasources.Tests.Contact;

internal static class ContactsServiceOf
{
    public static ContactsService Create(IConnection connection, ICommandMiddleware? middleware = null)
    {
        CRUDRepository<T> Of<T>(string entity, string[] columns) where T : new()
        {
            var repo = new CRUDRepository<T>(connection, new RepositoryOptions<T>
            {
                Entity = entity,
                Columns = columns
            });
            return middleware is null ? repo : repo.AddMiddleware(middleware);
        }
        return new ContactsService(
            Of<Contact>("contacts_base", [
                "id", "uuid", "created", "updated", "version",
                "contact_source_id", "contact_type_id", "first_name", "last_name", "email", "notes"
            ]),
            Of<ContactType>("contact_type", [
                "id", "uuid", "created", "updated", "version", "name", "description"
            ]),
            Of<ContactSource>("contact_source", [
                "id", "uuid", "created", "updated", "version", "name", "description"
            ]),
            Of<AddressType>("address_type", [
                "id", "uuid", "created", "updated", "version", "name", "description"
            ]),
            Of<PhoneType>("phone_type", [
                "id", "uuid", "created", "updated", "version", "name", "description"
            ]),
            Of<Address>("address", [
                "id", "uuid", "created", "updated", "version", "contact_id", "address_type_id",
                "line1", "line2", "city", "region", "postal_code", "country"
            ]),
            Of<Phone>("phone", [
                "id", "uuid", "created", "updated", "version", "contact_id", "phone_type_id", "number"
            ]));
    }
}
