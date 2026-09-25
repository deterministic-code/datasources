using Datasources;

namespace Datasources.Tests.Contact;

internal static class ContactLookupIds
{
    public static async Task<ContactLookups> Resolve(IConnection connection)
    {
        async Task<int> Of<T>(string entity, string name) where T : Base, new()
        {
            var row = await new CRUDRepository<T>(connection, new CRUDRepositoryOptions<T>
            {
                Entity = entity,
                ReadProperties = ["id", "name"]
            }).ReadSingle(new Where(new Operator.Equals("name", name)));
            return row?.id ?? throw new InvalidOperationException($"{entity} '{name}' missing");
        }
        return new ContactLookups(
            await Of<ContactSource>("contact_source", "Web"),
            await Of<ContactType>("contact_type", "Person"),
            await Of<AddressType>("address_type", "Home"),
            await Of<PhoneType>("phone_type", "Mobile"));
    }
}
