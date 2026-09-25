using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public sealed class PhoneRepository : CRUDRepository<Phone>
{
    public PhoneRepository(IConnection connection) : base(connection, Options(connection.Provider)) { }

    static CRUDRepositoryOptions<Phone> Options(string provider) => new()
    {
        Entity = "phone",
        EntityMapper = Names.Tables,
        ColumnMappings = Names.Columns,
        ReadProperties = ["id", "uuid", "created", "updated", "version", "contact_id", "phone_type_id", "number"],
        Insert = provider == "mysql" ? MysqlCalls.Insert("add_phone") : null,
        Update = provider == "mysql" ? MysqlCalls.Update("update_phone") : null,
        Delete = provider == "mysql" ? MysqlCalls.Delete("delete_phone") : null
    };
}
