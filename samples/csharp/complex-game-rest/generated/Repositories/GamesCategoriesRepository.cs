using Datasources;

public sealed class GamesCategoriesRepository : CRUDRepository<Backend.Types.Datasource.GamesCategory>
{
    public GamesCategoriesRepository(IConnection connection) : base(connection, Options(connection)) { }

    public static IReadonlyRepository<Backend.Types.Datasource.GamesCategory> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.GamesCategory>(connection, Options(connection));

    static CRUDRepositoryOptions<Backend.Types.Datasource.GamesCategory> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.Datasource.GamesCategory>()
            .Add(new()
            {
                Entity = "GamesCategory",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["GamesCategory"] = "games_categories" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["GameId"] = "game_id",
                    ["CategoryId"] = "category_id",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "GameId", "CategoryId"],
                CreateProperties = ["GameId", "CategoryId"],
                UpdateProperties = ["Uuid", "GameId", "CategoryId"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["GameId"] = new("biginteger", false, false),
                    ["CategoryId"] = new("biginteger", false, false),
                }),
            })
            .Add("sqlite", new()
            {
                Entity = "GamesCategory",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["GamesCategory"] = "games_categories" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["GameId"] = "game_id",
                    ["CategoryId"] = "category_id",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "GameId", "CategoryId"],
                CreateProperties = ["Uuid", "Created", "Version", "GameId", "CategoryId"],
                UpdateProperties = ["Uuid", "Created", "Version", "GameId", "CategoryId"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["GameId"] = new("biginteger", false, false),
                    ["CategoryId"] = new("biginteger", false, false),
                }),
            })
            .For(connection);
}
