using System.Reflection;

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
        var separator = JoinBatches.Of(settings);
        var bodies = new List<string>();
        var drops = new List<string>();
        foreach (var t in tables)
        {
            var (statements, names) = ForTable(dialect, t, mapping, casing, byGroups.GetValueOrDefault(t.Name) ?? []);
            var created = statements.Select(statement => GenerateSql.Render(dialect, statement)).ToList();
            drops.AddRange(names.Select(name => GenerateSql.Render(dialect, TableStatements.DropRoutine(name))));
            if (created.Any(sql => sql != "")) bodies.Add($"-- {t.Name}\n\n{JoinBatches.Join(created, separator)}");
        }
        if (bodies.Count == 0) return [];
        return
        [
            new($"{dialect}/migrations/{order:0000}_stored_procedures_up.sql", WithNl($"-- Stored procedures\n{JoinBatches.Join(bodies, separator)}", casing, dialect)),
            new($"{dialect}/migrations/{order:0000}_stored_procedures_down.sql", WithNl($"-- DROP stored procedures\n{JoinBatches.Join(drops, separator)}", casing, dialect)),
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
        RoutineParameter Param(SpecTypeField field) =>
            new(Column(field.Name), field.Type, field.ConverterField());
        var columns = t.Fields.Select(f => Column(f.Name)).ToList();
        var writable = ProcedureFields.UpdateParams(t).Select(f => Column(f.Name)).ToList();
        var createdParams = ProcedureFields.CreateParams(t).Select(Param).ToList();
        var typedColumns = t.Fields.Select(Param).ToList();
        var createColumns = ProcedureFields.CreateParams(t).Select(Param).ToList();
        var updateColumns = ProcedureFields.UpdateParams(t)
            .Where(f => f.Name != pk.Name)
            .Select(Param)
            .Prepend(Param(pk))
            .ToList();
        var payload = new RoutineParameter("payload", "payload");
        var key = Column(pk.Name);
        var keyType = pk.Type;
        var overlay = SqlSchema.OverlayOf(t);
        var audit = t.Fields.FirstOrDefault(f => f.Name == "updated") is { } updated && WriteInclusion.Update(updated, t.AsType(), overlay)
            ? Column("updated")
            : null;
        var generatedKey = pk.Type == "uuid";
        var table = t.TableName;
        var statements = new List<IStatement>();
        var names = new List<string>();
        foreach (var proc in ProcedureSpecsOf(dialect, t.Name, groups, casing))
        {
            names.Add(proc.Name);
            var by = proc.ByProperties.Select(Column).ToList();
            var byParams = proc.ByProperties.Select(logical =>
            {
                var field = t.Fields.FirstOrDefault(f => f.Name == logical) ?? pk;
                return Param(field);
            }).ToList();
            var updateParams = byParams
                .Concat(ProcedureFields.UpdateParams(t)
                    .Where(f => !proc.ByProperties.Contains(f.Name))
                    .Select(Param))
                .ToList();
            statements.Add(proc.Op switch
            {
                "create" => TableStatements.CreateCreateStoredProcedure(proc.Name, table, columns, createdParams, key, keyType, writable, generatedKey, audit),
                "read" => TableStatements.CreateReadStoredProcedure(proc.Name, table, columns, byParams, key, keyType, writable, by, generatedKey, audit),
                "update" => TableStatements.CreateUpdateStoredProcedure(proc.Name, table, columns, updateParams, key, keyType, writable, by, generatedKey, audit),
                "delete" => TableStatements.CreateDeleteStoredProcedure(proc.Name, table, columns, byParams, key, keyType, writable, by, generatedKey, audit),
                "bulkCreate" => TableStatements.CreateBulkCreateStoredProcedure(proc.Name, table, createColumns, payload, key, keyType),
                "bulkUpdate" => TableStatements.CreateBulkUpdateStoredProcedure(proc.Name, table, updateColumns, payload, key, keyType),
                "bulkDelete" => TableStatements.CreateBulkDeleteStoredProcedure(proc.Name, table, typedColumns, payload, key, keyType),
                _ => throw new InvalidOperationException($"{proc.Op}: empty step"),
            });
        }
        return (statements, names);
    }

    static IReadOnlyList<ProcedureSpec> ProcedureSpecsOf(
        string dialect,
        string entityName,
        IReadOnlyList<IReadOnlyList<string>> byGroups,
        PackCasing casing)
    {
        var method = GenerateSql.Driver(dialect).GetType()
            .GetMethod("ProcedureSpecs", BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Static)
            ?? throw new InvalidOperationException($"\"{dialect}\" has no ProcedureSpecs.");
        var rows = (IReadOnlyList<(string Op, string Name, IReadOnlyList<string> ByProperties)>)method.Invoke(
            null,
            [entityName, byGroups, (Func<string, string>)casing.RoutineName, (Func<string, string>)casing.PluralStem])!;
        return rows.Select(row => new ProcedureSpec(row.Op, row.Name, row.ByProperties)).ToList();
    }

    static string WithNl(string text, PackCasing casing, string dialect)
    {
        var output = casing.ApplyKeywords(text, dialect);
        return output.EndsWith('\n') ? output : $"{output}\n";
    }
}
