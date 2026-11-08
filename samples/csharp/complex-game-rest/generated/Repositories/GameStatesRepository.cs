using Datasources;

public sealed class GameStatesRepository : CRUDRepository<Backend.Types.Datasource.GameState>
{
    public GameStatesRepository(IConnection connection) : base(connection, Options(connection)) { }

    public static IReadonlyRepository<Backend.Types.Datasource.GameState> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.GameState>(connection, Options(connection));

    static CRUDRepositoryOptions<Backend.Types.Datasource.GameState> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.Datasource.GameState>()
            .Add(new()
            {
                Entity = "GameState",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["GameState"] = "game_states" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["GameId"] = "game_id",
                    ["UserId"] = "user_id",
                    ["State"] = "state",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "GameId", "UserId", "State"],
                CreateProperties = ["GameId", "UserId", "State"],
                UpdateProperties = ["Uuid", "GameId", "UserId", "State"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["GameId"] = new("biginteger", false, false),
                    ["UserId"] = new("biginteger", false, false),
                    ["State"] = new("string", false, false),
                }),
            })
            .Add("sqlite", new()
            {
                Entity = "GameState",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["GameState"] = "game_states" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["GameId"] = "game_id",
                    ["UserId"] = "user_id",
                    ["State"] = "state",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "GameId", "UserId", "State"],
                CreateProperties = ["Uuid", "Created", "Version", "GameId", "UserId", "State"],
                UpdateProperties = ["Uuid", "Created", "Version", "GameId", "UserId", "State"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["GameId"] = new("biginteger", false, false),
                    ["UserId"] = new("biginteger", false, false),
                    ["State"] = new("string", false, false),
                }),
            })
            .For(connection);
}
