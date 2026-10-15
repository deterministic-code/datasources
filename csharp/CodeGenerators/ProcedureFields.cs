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

    public static IReadOnlyList<SpecTypeField> WritableNonAudit(SqlTable table)
    {
        var pk = PkOf(table);
        return table.Fields.Where(f => f.Name != pk.Name && f.Name is not "uuid" and not "created" and not "updated").ToList();
    }

    public static IReadOnlyList<SpecTypeField> CreateParams(SqlTable table)
    {
        SpecTypeField[] Field(string name) => table.Fields.FirstOrDefault(f => f.Name == name) is { } found ? [found] : [];
        return [.. Field("uuid"), .. WritableNonAudit(table), .. Field("created"), .. Field("updated")];
    }
}
