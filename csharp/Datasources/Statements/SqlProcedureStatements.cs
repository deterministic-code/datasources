namespace Datasources;

public static class SqlProcedureStatements
{
    public static IReadOnlyList<IStatement> Of(ProcedureSource source)
    {
        string Name(string stem) => source.RoutineName(stem);
        var plural = source.Plural;
        var payload = new RoutineParameter("payload", "payload");
        var statements = new List<IStatement>
        {
            new CreateCreateStoredProcedureStatement(
                Name($"create_{plural}"), source.Table, source.Columns, source.CreateParameters,
                source.Key, source.KeyType, source.Writable, source.GeneratedKey, source.AuditUpdated),
            new CreateReadStoredProcedureStatement(
                Name($"read_{plural}"), source.Table, source.Columns, [],
                source.Key, source.KeyType, source.Writable, [], source.GeneratedKey, source.AuditUpdated),
        };
        foreach (var group in source.ByGroups)
        {
            var suffix = $"_by_{string.Join("_", group.Logical)}";
            statements.Add(new CreateReadStoredProcedureStatement(
                Name($"read_{plural}{suffix}"), source.Table, source.Columns, group.Parameters,
                source.Key, source.KeyType, source.Writable, group.Columns, source.GeneratedKey, source.AuditUpdated));
            statements.Add(new CreateUpdateStoredProcedureStatement(
                Name($"update_{plural}{suffix}"), source.Table, source.Columns, group.UpdateParameters,
                source.Key, source.KeyType, source.Writable, group.Columns, source.GeneratedKey, source.AuditUpdated));
            statements.Add(new CreateDeleteStoredProcedureStatement(
                Name($"delete_{plural}{suffix}"), source.Table, source.Columns, group.Parameters,
                source.Key, source.KeyType, source.Writable, group.Columns, source.GeneratedKey, source.AuditUpdated));
        }
        statements.Add(new CreateBulkCreateStoredProcedureStatement(
            Name($"bulk_create_{plural}"), source.Table, source.CreateColumns, payload, source.Key, source.KeyType));
        statements.Add(new CreateBulkUpdateStoredProcedureStatement(
            Name($"bulk_update_{plural}"), source.Table, source.UpdateColumns, payload, source.Key, source.KeyType));
        statements.Add(new CreateBulkDeleteStoredProcedureStatement(
            Name($"bulk_delete_{plural}"), source.Table, source.TypedColumns, payload, source.Key, source.KeyType));
        return statements;
    }

    public static string NameOf(IStatement statement) => statement switch
    {
        CreateCreateStoredProcedureStatement s => s.Name,
        CreateReadStoredProcedureStatement s => s.Name,
        CreateUpdateStoredProcedureStatement s => s.Name,
        CreateDeleteStoredProcedureStatement s => s.Name,
        CreateBulkCreateStoredProcedureStatement s => s.Name,
        CreateBulkUpdateStoredProcedureStatement s => s.Name,
        CreateBulkDeleteStoredProcedureStatement s => s.Name,
        _ => throw new InvalidOperationException("procedure statement has no name"),
    };
}
