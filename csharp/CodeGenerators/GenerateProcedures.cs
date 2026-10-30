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
        var tables = SqlSchema.BuildLive(types, casing.Naming).Where(SqlSchema.HasAuditColumns).ToList();
        if (tables.Count == 0) return [];
        var byGroups = ByGroups.From(types);
        var separator = JoinBatches.Of(settings);
        var bodies = new List<string>();
        var drops = new List<string>();
        foreach (var t in tables)
        {
            var statements = ForTable(dialect, t, casing, byGroups.GetValueOrDefault(t.Name) ?? []);
            var created = statements.Select(statement => GenerateSql.Render(dialect, statement)).ToList();
            drops.AddRange(statements.Select(statement =>
                GenerateSql.Render(dialect, TableStatements.DropRoutine(SqlProcedureStatements.NameOf(statement)))));
            if (created.Any(sql => sql != "")) bodies.Add($"-- {t.Name}\n\n{JoinBatches.Join(created, separator)}");
        }
        if (bodies.Count == 0) return [];
        return
        [
            new($"{dialect}/migrations/{order:0000}_stored_procedures_up.sql", WithNl($"-- Stored procedures\n{JoinBatches.Join(bodies, separator)}", dialect, casing.Naming)),
            new($"{dialect}/migrations/{order:0000}_stored_procedures_down.sql", WithNl($"-- DROP stored procedures\n{JoinBatches.Join(drops, separator)}", dialect, casing.Naming)),
        ];
    }

    public static IReadOnlyList<IStatement> ForTable(
        string dialect,
        SqlTable t,
        PackCasing casing,
        IReadOnlyList<IReadOnlyList<string>> groups)
    {
        var pk = ProcedureFields.PkOf(t);
        string Column(string logical) =>
            CreateTableStatement.ColumnName(SqlSchema.FieldStem(t, logical), casing.Naming);
        RoutineParameter Param(SpecTypeField field) =>
            new(Column(field.Name), field.Type, field.ConverterField());
        var overlay = SqlSchema.OverlayOf(t);
        var audit = t.Fields.FirstOrDefault(f => f.Name == "updated") is { } updated && WriteInclusion.Update(updated, t.AsType(), overlay)
            ? Column("updated")
            : null;
        var byGroups = groups.Select(logical =>
        {
            var parameters = logical.Select(name =>
            {
                var field = t.Fields.FirstOrDefault(f => f.Name == name) ?? pk;
                return Param(field);
            }).ToList();
            var update = parameters
                .Concat(ProcedureFields.UpdateParams(t).Where(f => !logical.Contains(f.Name)).Select(Param))
                .ToList();
            return new ProcedureByGroup(logical, logical.Select(Column).ToList(), parameters, update);
        }).ToList();
        var source = new ProcedureSource
        {
            Plural = casing.PluralStem(t.Name),
            Table = t.TableName,
            Columns = t.Fields.Select(f => Column(f.Name)).ToList(),
            CreateParameters = ProcedureFields.CreateParams(t).Select(Param).ToList(),
            Writable = ProcedureFields.UpdateParams(t).Select(f => Column(f.Name)).ToList(),
            CreateColumns = ProcedureFields.CreateParams(t).Select(Param).ToList(),
            UpdateColumns = ProcedureFields.UpdateParams(t)
                .Where(f => f.Name != pk.Name)
                .Select(Param)
                .Prepend(Param(pk))
                .ToList(),
            TypedColumns = t.Fields.Select(Param).ToList(),
            ByGroups = byGroups,
            Key = Column(pk.Name),
            KeyType = pk.Type,
            AuditUpdated = audit,
            GeneratedKey = pk.Type == "uuid",
            RoutineName = casing.RoutineName,
        };
        return GenerateSql.Driver(dialect).StatementRenderer!.Procedures(source);
    }

    static string WithNl(string text, string dialect, StatementNaming naming)
    {
        var output = GenerateSql.Schema(dialect, naming).ApplyKeywords(text);
        return output.EndsWith('\n') ? output : $"{output}\n";
    }
}
