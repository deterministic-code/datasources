using Datasources;

public sealed class AddressTypeRepository : CRUDRepository<Backend.Types.Datasource.AddressType>
{
    public AddressTypeRepository(IConnection connection) : base(connection, Options(connection)) { }

    public static IReadonlyRepository<Backend.Types.Datasource.AddressType> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.AddressType>(connection, Options(connection));

    static CRUDRepositoryOptions<Backend.Types.Datasource.AddressType> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.Datasource.AddressType>()
            .Add(new()
            {
                Entity = "AddressType",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["AddressType"] = "address_types" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["Name"] = "name",
                    ["Description"] = "description",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "Description"],
                CreateProperties = ["Uuid", "Created", "Updated", "Version", "Name", "Description"],
                UpdateProperties = ["Uuid", "Created", "Updated", "Version", "Name", "Description"],
            })
            .Add("sqlite", new()
            {
                Entity = "AddressType",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["AddressType"] = "address_types" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["Name"] = "name",
                    ["Description"] = "description",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "Description"],
                CreateProperties = ["Uuid", "Version", "Name", "Description"],
                UpdateProperties = ["Uuid", "Version", "Name", "Description"],
            })
            .Add("mysql", new()
            {
                Entity = "AddressType",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["AddressType"] = "address_types" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["Name"] = "name",
                    ["Description"] = "description",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "Description"],
                CreateProperties = ["Name", "Description"],
                UpdateProperties = ["Uuid", "Version", "Name", "Description"],
            })
            .Add("postgres", new()
            {
                Entity = "AddressType",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["AddressType"] = "address_types" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["Name"] = "name",
                    ["Description"] = "description",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "Description"],
                CreateProperties = ["Name", "Description"],
                UpdateProperties = ["Uuid", "Version", "Name", "Description"],
            })
            .For(connection);
}
