using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public sealed class AddressTypeRepository : CRUDRepository<Lookup>
{
    public AddressTypeRepository(IConnection connection) : base(connection, new CRUDRepositoryOptions<Lookup>
    {
        Entity = "address_type",
        EntityMapper = Names.Tables,
        PropertyMappings = Names.Columns,
        ReadProperties = ["id", "uuid", "created", "updated", "version", "name", "description"]
    })
    { }
}
