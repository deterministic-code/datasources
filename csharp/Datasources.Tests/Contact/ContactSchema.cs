using Datasources.CodeGenerators;

namespace Datasources.Tests.Contact;

internal static class ContactSchema
{
    static IReadOnlyList<IStatement> Statements(string dialect)
    {
        var files = Files();
        var spec = SpecParser.Parse(files);
        var types = SqlSchema.From(spec);
        var mapping = new SqlMapping(PackCasing.Of(SettingsOf(files[SpecParser.DatasourceYaml])), types);
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
        var dir = Path.Combine(SpecParser.SpecsDir(), "contacts");
        return new Dictionary<string, string>
        {
            [SpecParser.TypesYaml] = File.ReadAllText(Path.Combine(dir, "types.yaml")),
            [SpecParser.DatasourceYaml] = File.ReadAllText(Path.Combine(dir, "datasource.yaml")),
        };
    }

    static IReadOnlyDictionary<string, string> SettingsOf(string yaml)
    {
        var settings = new Dictionary<string, string>();
        Flatten(YamlCursor.FromYaml(yaml).Child("settings").Value, "", settings);
        return settings;
    }

    static void Flatten(object? value, string prefix, Dictionary<string, string> settings)
    {
        if (value is Dictionary<object, object> map)
        {
            foreach (var (key, child) in map)
            {
                if (key is not string name) continue;
                Flatten(child, prefix.Length == 0 ? name : $"{prefix}.{name}", settings);
            }
            return;
        }
        if (prefix.Length == 0) return;
        settings[prefix] = value switch
        {
            bool flag => flag ? "true" : "false",
            _ => Convert.ToString(value) ?? ""
        };
    }
}
