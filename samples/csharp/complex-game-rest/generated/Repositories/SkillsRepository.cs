using Datasources;

public sealed class SkillsRepository : CRUDRepository<Backend.Types.Datasource.Skill>
{
    public SkillsRepository(IConnection connection) : base(connection, Options(connection)) { }

    public static IReadonlyRepository<Backend.Types.Datasource.Skill> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.Skill>(connection, Options(connection));

    public static CRUDRepositoryOptions<Backend.Types.Datasource.Skill> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.Datasource.Skill>()
            .Add(new()
            {
                Entity = "Skill",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Skill"] = "skills" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["ParentSkillId"] = "parent_skill_id",
                    ["Name"] = "name",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ParentSkillId", "Name"],
                CreateProperties = ["ParentSkillId", "Name"],
                UpdateProperties = ["Uuid", "ParentSkillId", "Name"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["ParentSkillId"] = new("biginteger", true, false),
                    ["Name"] = new("string", false, false),
                }),
            })
            .Add("sqlite", new()
            {
                Entity = "Skill",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Skill"] = "skills" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["ParentSkillId"] = "parent_skill_id",
                    ["Name"] = "name",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ParentSkillId", "Name"],
                CreateProperties = ["Uuid", "Created", "Version", "ParentSkillId", "Name"],
                UpdateProperties = ["Uuid", "Created", "Version", "ParentSkillId", "Name"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["ParentSkillId"] = new("biginteger", true, false),
                    ["Name"] = new("string", false, false),
                }),
            })
            .For(connection);
}
