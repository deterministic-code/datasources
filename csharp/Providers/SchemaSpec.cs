using Datasources;
using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;

namespace Datasources.SqlSpecs;

public static class SchemaSpec
{
    public static IReadOnlyList<CreateTableStatement> Tables(
        string name,
        Func<string, string> quote,
        IReadOnlyDictionary<string, string>? rename = null) =>
        Load(name).Select(table => Statement(table, quote, rename)).ToArray();

    public static async Task Apply(IConnection connection, string name)
    {
        var command = connection.CreateCommand();
        foreach (var statement in Tables(name, static id => $"\"{id.Replace("\"", "\"\"")}\""))
            await command.Execute(statement);
    }

    static CreateTableStatement Statement(
        TableSpec table,
        Func<string, string> quote,
        IReadOnlyDictionary<string, string>? rename)
    {
        var catalog = Catalog();
        var specs = new List<ColumnSpec>();
        foreach (var fragment in table.Use)
            specs.AddRange(catalog.Fragments.TryGetValue(fragment, out var found)
                ? found
                : throw new InvalidOperationException($"unknown fragment {fragment}"));
        specs.AddRange(table.Columns);
        var keys = specs.Where(column => column.PrimaryKey).Select(column => column.Name).ToArray();
        var constraints = new List<ITableConstraint>();
        if (keys.Length > 0)
            constraints.Add(new PrimaryKey($"pk_{Name(table.Name, rename)}", keys.Select(quote).ToArray()));
        constraints.AddRange(table.Constraints.Select(constraint => constraint.Kind switch
        {
            "primaryKey" => (ITableConstraint)new PrimaryKey(constraint.Name, constraint.Columns.Select(quote).ToArray()),
            "unique" => new UniqueConstraint(constraint.Name, constraint.Columns.Select(quote).ToArray()),
            "check" => new CheckConstraint(constraint.Name, constraint.Expression ?? throw new InvalidOperationException($"{constraint.Name}: empty check")),
            "foreignKey" => new ForeignKey(
                constraint.Name,
                constraint.Columns.Select(quote).ToArray(),
                quote(Name(constraint.References ?? throw new InvalidOperationException($"{constraint.Name}: empty references"), rename)),
                constraint.ReferencesColumns.Select(quote).ToArray()),
            _ => throw new InvalidOperationException($"{constraint.Name}: empty constraint")
        }));
        return new CreateTableStatement(
            quote(Name(table.Name, rename)),
            specs.Select(column => new Column(
                quote(column.Name),
                column.Type,
                column.Default is null ? null : new ColumnDefault(column.Default),
                column.NotNull ? new NotNull() : null,
                column.PrimaryKey && column.Type == "int")).ToArray(),
            constraints);
    }

    static string Name(string logical, IReadOnlyDictionary<string, string>? rename) =>
        rename is not null && rename.TryGetValue(logical, out var physical) ? physical : logical;

    static IReadOnlyList<TableSpec> Load(string name)
    {
        var catalog = Catalog();
        return catalog.Schemas.TryGetValue(name, out var tables)
            ? tables
            : throw new InvalidOperationException($"unknown schema {name}");
    }

    static CatalogFile Catalog() => cache ??= new DeserializerBuilder()
        .WithNamingConvention(CamelCaseNamingConvention.Instance)
        .Build()
        .Deserialize<CatalogFile>(File.ReadAllText(Path.Combine(SpecsDir(), "schemas.yaml")));

    static string SpecsDir()
    {
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
            if (File.Exists(Path.Combine(dir.FullName, "specs", "command-crud.yaml")))
                return Path.Combine(dir.FullName, "specs");
        throw new DirectoryNotFoundException("specs");
    }

    static CatalogFile? cache;

    sealed class CatalogFile
    {
        public Dictionary<string, List<ColumnSpec>> Fragments { get; set; } = [];
        public Dictionary<string, List<TableSpec>> Schemas { get; set; } = [];
    }

    sealed class TableSpec
    {
        public string Name { get; set; } = "";
        public List<string> Use { get; set; } = [];
        public List<ColumnSpec> Columns { get; set; } = [];
        public List<ConstraintSpec> Constraints { get; set; } = [];
    }

    sealed class ColumnSpec
    {
        public string Name { get; set; } = "";
        public string Type { get; set; } = "";
        public bool PrimaryKey { get; set; }
        public bool NotNull { get; set; }
        public string? Default { get; set; }
    }

    sealed class ConstraintSpec
    {
        public string Kind { get; set; } = "";
        public string Name { get; set; } = "";
        public List<string> Columns { get; set; } = [];
        public string? References { get; set; }
        public List<string> ReferencesColumns { get; set; } = [];
        public string? Expression { get; set; }
    }
}
