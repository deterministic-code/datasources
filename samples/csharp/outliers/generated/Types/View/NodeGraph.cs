// schema-version: 1.0
using System.Collections.Generic;

namespace Backend.Types.View;

/** View NodeGraph. */
public class NodeGraph : Backend.Types.Datasource.Node
{
    public List<NodeGraph> Edges { get; set; } = [];
}
