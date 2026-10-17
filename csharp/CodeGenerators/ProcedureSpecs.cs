namespace Datasources.CodeGenerators;

public static class ProcedureSpecs
{
    public static IReadOnlyList<ProcedureSpec> Of(string entityName, IReadOnlyList<IReadOnlyList<string>> byGroups, PackCasing casing)
    {
        string Name(string stem) => casing.RoutineName(stem);
        var plural = casing.PluralStem(entityName);
        string Suffix(IReadOnlyList<string> props) => props.Count == 0 ? "" : $"_by_{string.Join("_", props)}";
        return
        [
            new("create", Name($"create_{plural}"), []),
            new("read", Name($"read_{plural}"), []),
            .. byGroups.SelectMany(props => new ProcedureSpec[]
            {
                new("read", Name($"read_{plural}{Suffix(props)}"), props),
                new("update", Name($"update_{plural}{Suffix(props)}"), props),
                new("delete", Name($"delete_{plural}{Suffix(props)}"), props),
            }),
            new("bulkCreate", Name($"bulk_create_{plural}"), []),
            new("bulkUpdate", Name($"bulk_update_{plural}"), []),
            new("bulkDelete", Name($"bulk_delete_{plural}"), []),
        ];
    }
}
