using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public sealed class AddressRepository : CRUDRepository<Address>
{
    public AddressRepository(IConnection connection) : base(connection, Options()) { }

    static CRUDRepositoryOptions<Address> Options() => new()
    {
        Entity = "address",
        EntityMapper = Names.Tables,
        PropertyMappings = Names.Columns,
        ReadProperties = ["id", "uuid", "created", "updated", "version", "contact_id", "address_type_id", "line1", "line2", "city", "region", "postal_code", "country"],
    };
}
