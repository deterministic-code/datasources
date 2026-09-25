using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public sealed class ContactSourceRepository : CRUDRepository<Lookup>
{
    public ContactSourceRepository(IConnection connection) : base(connection, new CRUDRepositoryOptions<Lookup>
    {
        Entity = "contact_source",
        EntityMapper = Names.Tables,
        ColumnMappings = Names.Columns,
        ReadProperties = ["id", "uuid", "created", "updated", "version", "name", "description"]
    })
    { }
}
