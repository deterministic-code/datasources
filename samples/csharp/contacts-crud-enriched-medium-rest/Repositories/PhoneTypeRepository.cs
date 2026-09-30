using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public sealed class PhoneTypeRepository : CRUDRepository<Lookup>
{
    public PhoneTypeRepository(IConnection connection) : base(connection, new CRUDRepositoryOptions<Lookup>
    {
        Entity = "phone_type",
        EntityMapper = Names.Tables,
        PropertyMappings = Names.Columns,
        ReadProperties = ["id", "uuid", "created", "updated", "version", "name", "description"]
    })
    { }
}
