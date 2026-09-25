using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public sealed class ContactTypeRepository : CRUDRepository<Lookup>
{
    public ContactTypeRepository(IConnection connection) : base(connection, new CRUDRepositoryOptions<Lookup>
    {
        Entity = "contact_type",
        EntityMapper = Names.Tables,
        ColumnMappings = Names.Columns,
        ReadProperties = ["id", "uuid", "created", "updated", "version", "name", "description"]
    })
    { }
}
