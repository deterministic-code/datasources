using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public sealed class PhoneRepository : CRUDRepository<Phone>
{
    public PhoneRepository(IConnection connection) : base(connection, Options()) { }

    static CRUDRepositoryOptions<Phone> Options() => new()
    {
        Entity = "phone",
        EntityMapper = Names.Tables,
        PropertyMappings = Names.Columns,
        ReadProperties = ["id", "uuid", "created", "updated", "version", "contact_id", "phone_type_id", "number"],
    };
}
