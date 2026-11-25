// schema-version: 1.0
using System.Collections.Generic;

namespace Backend.Types.View;

/** View WardView. */
public class WardView : Backend.Types.Datasource.Ward
{
    public List<Backend.Types.Datasource.WardAlias> Aliases { get; set; } = [];
    public Dictionary<string, string> Settings { get; set; } = [];
    public List<Backend.Types.Datasource.WardNode> Nodes { get; set; } = [];
}
