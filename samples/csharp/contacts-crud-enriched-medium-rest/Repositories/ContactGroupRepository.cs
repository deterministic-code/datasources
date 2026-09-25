using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public sealed class ContactGroupRepository : CRUDRepository<ContactGroup>
{
    public ContactGroupRepository(IConnection connection) : base(connection, new CRUDRepositoryOptions<ContactGroup>
    {
        Entity = "contact_groups",
        EntityMapper = Names.Tables,
        ColumnMappings = Names.Columns,
        ReadProperties = ["id", "uuid", "created", "updated", "version", "name", "description"]
    })
    { }
}
