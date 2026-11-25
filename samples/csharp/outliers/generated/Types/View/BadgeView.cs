// schema-version: 1.0
using System.Collections.Generic;

namespace Backend.Types.View;

/** View BadgeView. */
public class BadgeView : Backend.Types.Datasource.Badge
{
    public List<Backend.Types.Datasource.BadgeAlias> Aliases { get; set; } = [];
    public Dictionary<string, string> Settings { get; set; } = [];
    public List<Backend.Types.Datasource.BadgeNode> Nodes { get; set; } = [];
}
