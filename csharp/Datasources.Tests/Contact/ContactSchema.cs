using Datasources.CodeGenerators;

namespace Datasources.Tests.Contact;

internal static class ContactSchema
{
    public static async Task Apply(IConnection connection, string dialect, IDriver driver)
    {
        GenerateSql.Register(dialect, driver);
        var command = connection.CreateCommand();
        foreach (var statement in Statements(dialect))
            await command.Execute(statement);
    }

    static IReadOnlyList<IStatement> Statements(string dialect)
    {
        var spec = SpecParser.Parse(Files());
        var naming = PackCasing.Of(new Dictionary<string, string>
        {
            ["datasource.pluralize_datatable_names"] = "false",
        }).Naming;
        var tables = SqlSchema.BuildLive(SqlSchema.From(spec), naming);
        return SchemaDiff.Of(dialect, [], naming, tables, naming);
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
}
