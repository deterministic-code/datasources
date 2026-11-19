// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type Skill. */
public class Skill : Base
{
    public long? ParentSkillId { get; set; }
    public string Name { get; set; } = "";
}
