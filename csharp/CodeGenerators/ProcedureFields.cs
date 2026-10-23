namespace Datasources.CodeGenerators;

public static class ProcedureFields
{
    public static SpecTypeField PkOf(SqlTable table)
    {
        var pkName = SpecTables.PkName(table.AsType(), SqlSchema.OverlayOf(table));
        return table.Fields.FirstOrDefault(f => f.Name == pkName)
            ?? table.Fields.FirstOrDefault(f => f.Name == "id")
            ?? new SpecTypeField { Name = "id", Type = "integer", Kind = "primitive", Base = "integer", IsArray = false, IsNullable = false };
    }

    public static IReadOnlyList<SpecTypeField> CreateParams(SqlTable table)
    {
        var type = table.AsType();
        var overlay = SqlSchema.OverlayOf(table);
        return table.Fields.Where(f => WriteInclusion.Create(f, type, overlay)).ToList();
    }

    public static IReadOnlyList<SpecTypeField> UpdateParams(SqlTable table)
    {
        var type = table.AsType();
        var overlay = SqlSchema.OverlayOf(table);
        return table.Fields.Where(f => WriteInclusion.Update(f, type, overlay)).ToList();
    }
}
