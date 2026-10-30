namespace Datasources.Mysql;

internal static class SqlProcedureSpecs
{
    internal static IReadOnlyList<(string Op, string Name, IReadOnlyList<string> ByProperties)> Of(
        string entityName,
        IReadOnlyList<IReadOnlyList<string>> byGroups,
        Func<string, string> routineName,
        Func<string, string> pluralStem)
    {
        var plural = pluralStem(entityName);
        string Suffix(IReadOnlyList<string> props) => props.Count == 0 ? "" : $"_by_{string.Join("_", props)}";
        return
        [
            ("create", routineName($"create_{plural}"), []),
            ("read", routineName($"read_{plural}"), []),
            .. byGroups.SelectMany(props => new (string Op, string Name, IReadOnlyList<string> ByProperties)[]
            {
                ("read", routineName($"read_{plural}{Suffix(props)}"), props),
                ("update", routineName($"update_{plural}{Suffix(props)}"), props),
                ("delete", routineName($"delete_{plural}{Suffix(props)}"), props),
            }),
            ("bulkCreate", routineName($"bulk_create_{plural}"), []),
            ("bulkUpdate", routineName($"bulk_update_{plural}"), []),
            ("bulkDelete", routineName($"bulk_delete_{plural}"), []),
        ];
    }
}
