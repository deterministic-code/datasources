namespace Datasources.CodeGenerators;

public static class GenerateProcedures
{
    public static IReadOnlyList<GenerateEntry> ForDialect(
        string dialect,
        IReadOnlyDictionary<string, string> files,
        IReadOnlyDictionary<string, string> settings,
        int order = 2)
    {
        if (!SqlSchema.UseStoredProcedures(settings)) return [];
        var spec = SpecParser.Parse(files);
        var types = SqlSchema.From(spec);
        var casing = PackCasing.Of(settings);
        var mapping = new SqlMapping(casing, types);
        var tables = SqlSchema.BuildLive(types, mapping).Where(SqlSchema.HasAuditColumns).ToList();
        if (tables.Count == 0) return [];
        var byGroups = ByGroups.From(types);
        var bodies = new List<string>();
        var drops = new List<string>();
        foreach (var t in tables)
        {
            var (statements, names) = ForTable(dialect, t, mapping, casing, byGroups.GetValueOrDefault(t.Name) ?? []);
            var created = statements.Select(statement => SqlDialect.RenderSql(dialect, statement)).Where(sql => sql != "").ToList();
            drops.AddRange(names.Select(name => SqlDialect.RenderSql(dialect, TableStatements.DropRoutine(name))).Where(sql => sql != ""));
            if (created.Count > 0) bodies.Add($"-- {t.Name}\n\n{string.Join("\n\n", created)}");
        }
        if (bodies.Count == 0) return [];
        return
        [
            new($"{dialect}/migrations/{order:0000}_stored_procedures_up.sql", WithNl($"-- Stored procedures\n{string.Join("\n\n", bodies)}", casing)),
            new($"{dialect}/migrations/{order:0000}_stored_procedures_down.sql", WithNl($"-- DROP stored procedures\n{string.Join("\n", drops)}", casing)),
        ];
    }

    public static (IReadOnlyList<IStatement> Statements, IReadOnlyList<string> Names) ForTable(
        string dialect,
        SqlTable t,
        SqlMapping mapping,
        PackCasing casing,
        IReadOnlyList<IReadOnlyList<string>> groups)
    {
        var pk = ProcedureFields.PkOf(t);
        string Column(string logical) => mapping.ColumnName(t, logical);
        var columns = t.Fields.Select(f => Column(f.Name)).ToList();
        var writable = ProcedureFields.WritableNonAudit(t).Select(f => Column(f.Name)).ToList();
        string ParamType(SpecTypeField field) => SqlDialect.MapColumnType(dialect, field.ConverterField());
        var createdParams = ProcedureFields.CreateParams(t).Select(f => new RoutineParameter(Column(f.Name), ParamType(f))).ToList();
        var typedColumns = t.Fields.Select(f => new RoutineParameter(Column(f.Name), ParamType(f))).ToList();
        var payload = new RoutineParameter("payload", PayloadType(dialect));
        var key = Column(pk.Name);
        var keyType = ParamType(pk);
        var audit = t.Fields.Any(f => f.Name == "updated") ? Column("updated") : null;
        var generatedKey = pk.Type == "uuid";
        var quoted = SqlDialect.Q(dialect, t.TableName);
        var statements = new List<IStatement>();
        var names = new List<string>();
        foreach (var proc in ProcedureSpecs.Of(t.Name, groups, casing))
        {
            names.Add(proc.Name);
            var by = proc.ByProperties.Select(Column).ToList();
            var byParams = proc.ByProperties.Select(logical =>
            {
                var field = t.Fields.FirstOrDefault(f => f.Name == logical) ?? pk;
                return new RoutineParameter(Column(field.Name), ParamType(field));
            }).ToList();
            var updateParams = byParams
                .Concat(ProcedureFields.WritableNonAudit(t)
                    .Where(f => !proc.ByProperties.Contains(f.Name))
                    .Select(f => new RoutineParameter(Column(f.Name), ParamType(f))))
                .Concat(audit is null ? [] : [new RoutineParameter(Column("new_updated"), ParamType(t.Fields.First(f => f.Name == "updated")))])
                .ToList();
            statements.Add(proc.Op switch
            {
                "create" => TableStatements.CreateCreateStoredProcedure(proc.Name, quoted, columns, createdParams, key, keyType, writable, generatedKey, audit),
                "read" => TableStatements.CreateReadStoredProcedure(proc.Name, quoted, columns, byParams, key, keyType, writable, by, generatedKey, audit),
                "update" => TableStatements.CreateUpdateStoredProcedure(proc.Name, quoted, columns, updateParams, key, keyType, writable, by, generatedKey, audit),
                "delete" => TableStatements.CreateDeleteStoredProcedure(proc.Name, quoted, columns, byParams, key, keyType, writable, by, generatedKey, audit),
                "bulkCreate" => TableStatements.CreateBulkCreateStoredProcedure(proc.Name, quoted, typedColumns, payload, key, keyType),
                "bulkUpdate" => TableStatements.CreateBulkUpdateStoredProcedure(proc.Name, quoted, typedColumns, payload, key, keyType),
                "bulkDelete" => TableStatements.CreateBulkDeleteStoredProcedure(proc.Name, quoted, typedColumns, payload, key, keyType),
                _ => throw new InvalidOperationException($"{proc.Op}: empty step"),
            });
        }
        return (statements, names);
    }

    static string PayloadType(string dialect) => dialect switch
    {
        "mysql" => "JSON",
        "postgres" => "jsonb",
        "sqlserver" => "NVARCHAR(MAX)",
        "oracle" => "CLOB",
        _ => "JSON",
    };

    static string WithNl(string text, PackCasing casing)
    {
        var output = casing.ApplyKeywords(text);
        return output.EndsWith('\n') ? output : $"{output}\n";
    }
}
