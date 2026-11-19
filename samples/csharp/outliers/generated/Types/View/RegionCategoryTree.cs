// schema-version: 1.0
using System.Collections.Generic;

namespace Backend.Types.View;

/** View RegionCategoryTree. */
public class RegionCategoryTree : Backend.Types.Datasource.RegionCategory
{
    public List<RegionCategoryTree> Children { get; set; } = [];
}
