// schema-version: 1.0
using System.Collections.Generic;

namespace Backend.Types.View;

/** View CategoryTree. */
public class CategoryTree : Backend.Types.Datasource.Category
{
    public List<CategoryTree> Children { get; set; } = [];
}
