using Datasources;

public sealed class PhoneTypeRepository : CRUDRepository<Backend.Types.Datasource.PhoneType>
{
    public PhoneTypeRepository(IConnection connection) : base(connection, Options(connection)) { }

    public static IReadonlyRepository<Backend.Types.Datasource.PhoneType> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.PhoneType>(connection, Options(connection));

    static CRUDRepositoryOptions<Backend.Types.Datasource.PhoneType> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.Datasource.PhoneType>()
            .Add(new()
            {
                Entity = "PhoneType",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["PhoneType"] = "phone_types" }),
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
                Entity = "PhoneType",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["PhoneType"] = "phone_types" }),
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
                Entity = "PhoneType",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["PhoneType"] = "phone_types" }),
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
                Entity = "PhoneType",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["PhoneType"] = "phone_types" }),
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
