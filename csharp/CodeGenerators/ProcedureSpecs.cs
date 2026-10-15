namespace Datasources.CodeGenerators;

public static class ProcedureSpecs
{
    public static IReadOnlyList<ProcedureSpec> Of(string entityName, IReadOnlyList<string> byFields, PackCasing casing)
    {
        string Name(string stem) => casing.RoutineName(stem);
        return
        [
            new("create", Name($"create_{entityName}")),
            new("findOne", Name($"find_{entityName}")),
            new("findAll", Name($"find_{casing.PluralTableName(entityName)}")),
            .. byFields.Select(bf => new ProcedureSpec("findBy", Name($"find_{entityName}_by_{bf}"), bf)),
            new("update", Name($"update_{entityName}")),
            .. byFields.Select(bf => new ProcedureSpec("updateBy", Name($"update_{entityName}_by_{bf}"), bf)),
            new("delete", Name($"delete_{entityName}")),
        ];
    }
}
