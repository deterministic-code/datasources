// schema-version: 1.0
using System.Collections.Generic;

namespace Backend.Types.View;

/** View SkillTree. */
public class SkillTree : Backend.Types.Datasource.Skill
{
    public List<SkillTree> Children { get; set; } = [];
}
