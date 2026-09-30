using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public sealed class LegacyContactRepository : CRUDRepository<LegacyContact>
{
    public LegacyContactRepository(IConnection connection) : base(connection, Options()) { }

    static CRUDRepositoryOptions<LegacyContact> Options() => new()
    {
        Entity = "legacy_contact",
        Ids = [],
        EntityMapper = Names.Tables,
        PropertyMappings = Names.Columns,
        ReadProperties = ["key", "first_name", "last_name", "email", "imported_at"],
    };
}
