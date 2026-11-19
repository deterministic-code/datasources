// schema-version: 1.0
using System.Collections.Generic;

namespace Backend.Types.View;

/** View ItemGraph. */
public class ItemGraph : Backend.Types.Datasource.Item
{
    public List<ItemGraph> Edges { get; set; } = [];
}
