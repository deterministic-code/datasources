using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public sealed class AddressRepository : CRUDRepository<Address>
{
    public AddressRepository(IConnection connection) : base(connection, Options(connection.Provider)) { }

    static CRUDRepositoryOptions<Address> Options(string provider) => new()
    {
        Entity = "address",
        EntityMapper = Names.Tables,
        ColumnMappings = Names.Columns,
        ReadProperties = ["id", "uuid", "created", "updated", "version", "contact_id", "address_type_id", "line1", "line2", "city", "region", "postal_code", "country"],
        Insert = provider == "mysql" ? MysqlCalls.Insert("add_address") : null,
        Update = provider == "mysql" ? MysqlCalls.Update("update_address") : null,
        Delete = provider == "mysql" ? MysqlCalls.Delete("delete_address") : null
    };
}
