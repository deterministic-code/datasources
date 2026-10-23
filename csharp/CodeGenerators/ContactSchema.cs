namespace Datasources.CodeGenerators;

public static class ContactSchema
{
    static readonly IReadOnlyDictionary<string, string> Settings =
        new Dictionary<string, string> { ["datasource.pluralize_datatable_names"] = "false" };

    public static IReadOnlyList<IStatement> Statements(string dialect)
    {
        var spec = SpecParser.Parse(Files());
        var types = SqlSchema.From(spec);
        var mapping = new SqlMapping(PackCasing.Of(Settings), types);
        return [.. SqlSchema.BuildLive(types, mapping).SelectMany(table => StatementsOf(dialect, mapping, table))];
    }

    public static async Task Apply(IConnection connection, string dialect, IDriver driver)
    {
        SqlDialect.Register(dialect, driver);
        var command = connection.CreateCommand();
        foreach (var statement in Statements(dialect))
            await command.Execute(statement);
    }

    static IEnumerable<IStatement> StatementsOf(string dialect, SqlMapping mapping, SqlTable table)
    {
        yield return TableStatements.CreateTable(dialect, table, mapping);
        foreach (var index in table.Indexes)
            yield return TableStatements.CreateIndex(dialect, mapping, table, index);
    }

    static IReadOnlyDictionary<string, string> Files()
    {
        var dir = Path.Combine(SpecsDir(), "contacts");
        return new Dictionary<string, string>
        {
            [SpecParser.TypesYaml] = File.ReadAllText(Path.Combine(dir, "types.yaml")),
            [SpecParser.DatasourceYaml] = File.ReadAllText(Path.Combine(dir, "datasource.yaml")),
        };
    }

    static string SpecsDir()
    {
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
            if (File.Exists(Path.Combine(dir.FullName, "specs", "command-crud.yaml")))
                return Path.Combine(dir.FullName, "specs");
        throw new DirectoryNotFoundException("specs");
    }
}
