using Datasources;
using static Datasources.Cmp;

namespace Datasources.Tests.Contact;

internal static class ContactLookupIds
{
    public static async Task<ContactLookups> Resolve(IConnection connection)
    {
        async Task<int> Of<T>(string name) where T : Base, new()
        {
            var row = await new CRUDRepository<T>(connection, new CRUDRepositoryOptions<T>
            {
                EntityMapper = ContactColumns.Snake,
                ColumnMappings = ContactColumns.Columns,
                Ids = ["Id"],
            }).ReadSingle(new Where("Name", eq, name));
            return row?.Id ?? throw new InvalidOperationException($"{typeof(T).Name} '{name}' missing");
        }
        return new ContactLookups(
            await Of<ContactSource>("Web"),
            await Of<ContactType>("Person"),
            await Of<AddressType>("Home"),
            await Of<PhoneType>("Mobile"));
    }
}
