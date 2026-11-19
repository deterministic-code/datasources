using Datasources;

public sealed class SkillTreesRepository : ReadonlyRepository<Backend.Types.View.SkillTree>
{

    public SkillTreesRepository(IConnection connection) : base(connection, Options(connection))
    {
        AddEnriched(new("Children", "Id", "ParentSkillId", Type: EnrichmentPropertyType.List), new ReadonlyRepository<Backend.Types.View.SkillTree>(connection, Options(connection)), new TreeReadonlyEnrichmentStrategy<Backend.Types.View.SkillTree>());
    }

    public static IReadonlyRepository<Backend.Types.View.SkillTree> CreateReadonly(IConnection connection) =>
        new SkillTreesRepository(connection);

    public static ReadonlyRepositoryMappingOptions<Backend.Types.View.SkillTree> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.View.SkillTree>()
            .Add(new()
            {
                Entity = "SkillTree",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["SkillTree"] = "skills" }),
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
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ParentSkillId", "Name"],
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
                Entity = "SkillTree",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["SkillTree"] = "skills" }),
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
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ParentSkillId", "Name"],
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
