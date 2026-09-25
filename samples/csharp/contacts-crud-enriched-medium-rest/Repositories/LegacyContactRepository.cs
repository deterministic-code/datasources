using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public sealed class LegacyContactRepository : CRUDRepository<LegacyContact>
{
    public LegacyContactRepository(IConnection connection) : base(connection, Options(connection.Provider)) { }

    static CRUDRepositoryOptions<LegacyContact> Options(string provider) => new()
    {
        Entity = "legacy_contact",
        Ids = [],
        EntityMapper = Names.Tables,
        ColumnMappings = Names.Columns,
        ReadProperties = ["key", "first_name", "last_name", "email", "imported_at"],
        Insert = provider == "mysql" ? MysqlCalls.Insert("add_legacy_contact") : null,
        Update = provider == "mysql" ? MysqlCalls.Update("update_legacy_contact") : null,
        Delete = provider == "mysql" ? MysqlCalls.Delete("delete_legacy_contact") : null
    };
}
