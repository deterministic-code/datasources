namespace Datasources.CodeGenerators;

public static class GenerateProcedures
{
    public static IReadOnlyList<GenerateEntry> ForDialect(
        string dialect,
        IReadOnlyDictionary<string, string> files,
        IReadOnlyDictionary<string, string> settings)
    {
        if (!SqlSchema.UseStoredProcedures(settings)) return [];
        var spec = SpecParser.Parse(files);
        var types = SqlSchema.From(spec);
        var casing = PackCasing.Of(settings);
        var mapping = new SqlMapping(casing, types);
        var tables = SqlSchema.BuildLive(types, mapping).Where(SqlSchema.HasAuditColumns).ToList();
        if (tables.Count == 0) return [];
        var byFields = ByFields.From(types);
        var bodies = new List<string>();
        var drops = new List<string>();
        foreach (var t in tables)
        {
            var fields = byFields.GetValueOrDefault(t.Name) ?? [];
            var pk = ProcedureFields.PkOf(t);
            string Column(string logical) => mapping.ColumnName(t, logical);
            var columns = t.Fields.Select(f => Column(f.Name)).ToList();
            var writable = ProcedureFields.WritableNonAudit(t).Select(f => Column(f.Name)).ToList();
            string ParamType(SpecTypeField field) => SqlDialect.MapColumnType(dialect, field.ConverterField());
            var created = new List<string>();
            foreach (var proc in ProcedureSpecs.Of(t.Name, fields, casing))
            {
                var keyField = proc.ByField is { } by
                    ? t.Fields.FirstOrDefault(f => f.Name == by) ?? pk
                    : pk;
                var paramsList = new List<RoutineParameter>
                {
                    new(Column(keyField.Name), ParamType(keyField)),
                };
                paramsList.AddRange(ProcedureFields.WritableNonAudit(t)
                    .Where(f => f.Name != proc.ByField)
                    .Select(f => new RoutineParameter(Column(f.Name), ParamType(f))));
                if (t.Fields.Any(f => f.Name == "updated"))
                    paramsList.Add(new RoutineParameter(Column("new_updated"), ParamType(t.Fields.First(f => f.Name == "updated"))));
                var createParams = ProcedureFields.CreateParams(t)
                    .Select(f => new RoutineParameter(Column(f.Name), ParamType(f)))
                    .ToList();
                var kind = proc.Op == "updateBy" ? "update" : proc.Op;
                var parameters = proc.Op == "create"
                    ? createParams
                    : proc.Op.StartsWith("update", StringComparison.Ordinal)
                        ? paramsList
                        : proc.Op == "findBy"
                            ? [new RoutineParameter(Column(keyField.Name), ParamType(keyField))]
                            : new List<RoutineParameter>();
                var sql = SqlDialect.RenderSql(dialect, TableStatements.CreateRoutine(
                    proc.Name,
                    Kind(kind),
                    SqlDialect.Q(dialect, t.TableName),
                    columns,
                    parameters,
                    Column(pk.Name),
                    ParamType(pk),
                    writable,
                    proc.ByField is null ? null : Column(proc.ByField),
                    pk.Type == "uuid",
                    t.Fields.Any(f => f.Name == "updated") ? Column("updated") : null));
                if (sql != "") created.Add(sql);
                var drop = SqlDialect.RenderSql(dialect, TableStatements.DropRoutine(proc.Name));
                if (drop != "") drops.Add(drop);
            }
            if (created.Count > 0) bodies.Add($"-- {t.Name}\n\n{string.Join("\n\n", created)}");
        }
        if (bodies.Count == 0) return [];
        return
        [
            new($"{dialect}/migrations/0002_stored_procedures_up.sql", WithNl($"-- Stored procedures\n{string.Join("\n\n", bodies)}", casing)),
            new($"{dialect}/migrations/0002_stored_procedures_down.sql", WithNl($"-- DROP stored procedures\n{string.Join("\n", drops)}", casing)),
        ];
    }

    static RoutineKind Kind(string op) => op switch
    {
        "create" => RoutineKind.Create,
        "findOne" => RoutineKind.FindOne,
        "findAll" => RoutineKind.FindAll,
        "findBy" => RoutineKind.FindBy,
        "update" => RoutineKind.Update,
        "delete" => RoutineKind.Delete,
        _ => throw new InvalidOperationException($"{op}: empty step"),
    };

    static string WithNl(string text, PackCasing casing)
    {
        var output = casing.ApplyKeywords(text);
        return output.EndsWith('\n') ? output : $"{output}\n";
    }
}
