using Datasources;

public sealed class ReviewsRepository : CRUDRepository<Backend.Types.Datasource.Review>
{
    public ReviewsRepository(IConnection connection) : base(connection, Options(connection)) { }

    public static IReadonlyRepository<Backend.Types.Datasource.Review> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.Review>(connection, Options(connection));

    static CRUDRepositoryOptions<Backend.Types.Datasource.Review> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.Datasource.Review>()
            .Add(new()
            {
                Entity = "Review",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Review"] = "reviews" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["GameId"] = "game_id",
                    ["UserId"] = "user_id",
                    ["Description"] = "description",
                    ["Rating"] = "rating",
                    ["IsApproved"] = "is_approved",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "GameId", "UserId", "Description", "Rating", "IsApproved"],
                CreateProperties = ["GameId", "UserId", "Description", "Rating", "IsApproved"],
                UpdateProperties = ["Uuid", "GameId", "UserId", "Description", "Rating", "IsApproved"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["GameId"] = new("biginteger", false, false),
                    ["UserId"] = new("biginteger", false, false),
                    ["Description"] = new("string", false, false),
                    ["Rating"] = new("decimal", false, false),
                    ["IsApproved"] = new("boolean", false, true),
                }),
            })
            .Add("sqlite", new()
            {
                Entity = "Review",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Review"] = "reviews" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["GameId"] = "game_id",
                    ["UserId"] = "user_id",
                    ["Description"] = "description",
                    ["Rating"] = "rating",
                    ["IsApproved"] = "is_approved",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "GameId", "UserId", "Description", "Rating", "IsApproved"],
                CreateProperties = ["Uuid", "Created", "Version", "GameId", "UserId", "Description", "Rating", "IsApproved"],
                UpdateProperties = ["Uuid", "Created", "Version", "GameId", "UserId", "Description", "Rating", "IsApproved"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["GameId"] = new("biginteger", false, false),
                    ["UserId"] = new("biginteger", false, false),
                    ["Description"] = new("string", false, false),
                    ["Rating"] = new("decimal", false, false),
                    ["IsApproved"] = new("boolean", false, true),
                }),
            })
            .For(connection);
}
