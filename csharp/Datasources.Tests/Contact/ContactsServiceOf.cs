using Datasources;

namespace Datasources.Tests.Contact;

internal static class ContactsServiceOf
{
    public static CRUDRepository<Contact> Create(IConnection connection, ICommandMiddleware? middleware = null)
    {
        CRUDRepository<T> Of<T>(IMapper<string, string>? entity = null) where T : new()
        {
            var repo = new CRUDRepository<T>(connection, new CRUDRepositoryOptions<T>
            {
                EntityMapper = entity ?? ContactColumns.Snake,
                PropertyMappings = ContactColumns.Columns,
                Ids = ["Id"],
            });
            return middleware is null ? repo : repo.AddMiddleware(middleware);
        }
        return ContactsService.Open(
            Of<Contact>(ContactColumns.ContactEntity),
            Of<ContactType>(),
            Of<ContactSource>(),
            Of<AddressType>(),
            Of<PhoneType>(),
            Of<Address>(),
            Of<Phone>());
    }
}
