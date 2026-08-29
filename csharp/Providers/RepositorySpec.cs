using Datasources;
using Datasources.Builders;
using Datasources.Sqlite;
using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;

namespace Datasources.SqlSpecs;

public static class RepositorySpec
{
    public static IEnumerable<string> Files() =>
        Directory.GetFiles(Path.Combine(SqlSpec.SpecsDir(), "repositories"), "*.yaml");

    public static async Task Run(string path, Func<Session> open)
    {
        SqliteConverters.Register();
        var spec = new DeserializerBuilder()
            .WithNamingConvention(NullNamingConvention.Instance)
            .Build()
            .Deserialize<Spec>(File.ReadAllText(path));
        var session = open();
        await session.Connection.Open();
        await session.Command.Execute(SqliteStatementBuilder.CreateTable(spec.Type.Table, Columns(spec.Type)).GetText());
        var mapping = TypeConverter.Mapping<Dictionary<string, object?>>(
            "sqlite",
            schema: Schema(spec.Type),
            names: spec.Type.Fields.Keys.ToDictionary(name => name, name => name),
            name: spec.Type.Table);
        var built = RepositoryBuilder<Dictionary<string, object?>>.Parts(new()
        {
            Connection = session.Connection,
            Mapping = mapping,
            Ids = spec.Type.Ids,
            AssignIdentityOnAdd = spec.Type.AssignIdentityOnAdd,
        });
        var logs = new List<string>();
        var middleware = new ConsoleCommandMiddleware(logs.Add);
        built.Commands.Get.Use(middleware);
        built.Commands.Add.Use(middleware);
        built.Commands.Update.Use(middleware);
        built.Commands.Delete.Use(middleware);
        var repo = new Repository<Dictionary<string, object?>>(built);
        foreach (var step in spec.Steps)
        {
            if (step.Create is { } create) await repo.Add(Row(create.Row));
            else if (step.Read is { } read) await repo.GetSingle(WhereBuilder.Equals("id", read.Id));
            else if (step.Query is { } query)
            {
                await foreach (var _ in query.Where is { } where ? repo.Get(Where(where)) : repo.Get()) { }
            }
            else if (step.Ready is { } ready) await repo.GetSingle(Where(ready.Where));
            else if (step.Update is { } update) await repo.Update(Row(update.Row));
            else if (step.UpdateBy is { } updateBy) await repo.Update(Row(updateBy.Row), Where(updateBy.Where));
            else if (step.Delete is { } delete)
                await repo.Delete(new Dictionary<string, object?> { ["id"] = delete.Id });
            else if (step.DeleteBy is { } deleteBy)
                await repo.Delete(new Dictionary<string, object?>(), Where(deleteBy.Where));
            else throw new InvalidOperationException($"{spec.Name}: empty step");
        }
        AssertLogs(spec, logs);
    }

    static void AssertLogs(Spec spec, List<string> logs)
    {
        var expected = Queries(spec)
            .SelectMany(query => new[] { $"[datasource][Start]-[{query}]", $"[datasource][Finish]-[{query}]" })
            .ToList();
        var actual = logs.Select(Signature).ToList();
        var mismatch = -1;
        for (var i = 0; i < Math.Max(expected.Count, actual.Count); i++)
        {
            if (i >= expected.Count || i >= actual.Count || expected[i] != actual[i])
            {
                mismatch = i;
                break;
            }
        }
        if (mismatch < 0) return;
        Assert.Fail(
            $"{spec.Name}: first logging mismatch at {mismatch}\nexpected: {(mismatch < expected.Count ? expected[mismatch] : "<missing>")}\nactual:   {(mismatch < actual.Count ? actual[mismatch] : "<missing>")}\n--- expected ---\n{string.Join("\n", expected)}\n--- actual ---\n{string.Join("\n", actual)}");
    }

    static IEnumerable<string> Queries(Spec spec)
    {
        var columns = spec.Type.Fields.Keys.OrderBy(key => key).ToArray();
        var ids = spec.Type.Ids.ToHashSet();
        foreach (var step in spec.Steps)
        {
            if (step.Create is { } create)
                yield return StatementBuilder.CreateInsert(spec.Type.Table, Fields(create.Row, col => !spec.Type.AssignIdentityOnAdd || !ids.Contains(col))).GetText();
            else if (step.Read is { } read)
                yield return StatementBuilder.CreateQuery(spec.Type.Table, columns, WhereBuilder.Equals("id", read.Id)).GetText();
            else if (step.Query is { } query)
                yield return StatementBuilder.CreateQuery(spec.Type.Table, columns, query.Where is { } where ? Where(where) : null).GetText();
            else if (step.Ready is { } ready)
                yield return StatementBuilder.CreateQuery(spec.Type.Table, columns, Where(ready.Where)).GetText();
            else if (step.Update is { } update)
                yield return StatementBuilder.CreateUpdate(spec.Type.Table, Fields(update.Row, col => !ids.Contains(col)), WhereBuilder.Equals("id", update.Row["id"])).GetText();
            else if (step.UpdateBy is { } updateBy)
                yield return StatementBuilder.CreateUpdate(spec.Type.Table, Fields(updateBy.Row, col => !ids.Contains(col)), Where(updateBy.Where)).GetText();
            else if (step.Delete is { } delete)
                yield return StatementBuilder.CreateDelete(spec.Type.Table, WhereBuilder.Equals("id", delete.Id)).GetText();
            else if (step.DeleteBy is { } deleteBy)
                yield return StatementBuilder.CreateDelete(spec.Type.Table, Where(deleteBy.Where)).GetText();
        }
    }

    static IReadOnlyList<Parameter> Fields(Dictionary<string, object?> row, Func<string, bool> include) =>
        row.Where(pair => include(pair.Key)).OrderBy(pair => pair.Key)
            .Select(pair => new Parameter(pair.Key, pair.Value)).ToArray();

    static IWhere Where(Dictionary<string, object?> row) =>
        WhereBuilder.And([.. row.Select(pair => WhereBuilder.Equals(pair.Key, pair.Value))]);

    static Dictionary<string, object?> Row(Dictionary<string, object?> row) =>
        row.ToDictionary(pair => pair.Key, pair => Coerce(pair.Value));

    static object? Coerce(object? value) => value switch
    {
        "true" => true,
        "false" => false,
        _ => value
    };

    static Dictionary<string, string> Schema(TypeSpec type) =>
        type.Fields.ToDictionary(pair => pair.Key, pair => pair.Value switch
        {
            "int" => "INTEGER",
            "uuid" => "UUID",
            "string" => "TEXT",
            "bool" => "BOOLEAN",
            "decimal" => "NUMERIC",
            "float" => "REAL",
            "datetime" => "DATETIME",
            "binary" => "BLOB",
            _ => throw new DatasourceException($"Unknown field type \"{pair.Value}\"")
        });

    static SqliteColumn[] Columns(TypeSpec type) =>
        [.. type.Fields.Select(pair => new SqliteColumn(pair.Key, pair.Value, type.Ids.Contains(pair.Key)))];

    static string Signature(string line)
    {
        var body = line.Contains("]-") ? line[(line.IndexOf("]-", StringComparison.Ordinal) + 2)..] : line;
        var duration = body.LastIndexOf("] ", StringComparison.Ordinal);
        return duration >= 0 && (body.EndsWith("ms", StringComparison.Ordinal) || body.EndsWith("μs", StringComparison.Ordinal))
            ? body[..(duration + 1)]
            : body;
    }

    public sealed record Session(IConnection Connection, ICommand Command);

    sealed class Spec
    {
        [YamlMember(Alias = "name")]
        public string Name { get; set; } = "";
        [YamlMember(Alias = "type")]
        public TypeSpec Type { get; set; } = new();
        [YamlMember(Alias = "steps")]
        public List<Step> Steps { get; set; } = [];
    }

    sealed class TypeSpec
    {
        [YamlMember(Alias = "name")]
        public string Name { get; set; } = "";
        [YamlMember(Alias = "table")]
        public string Table { get; set; } = "";
        [YamlMember(Alias = "ids")]
        public List<string> Ids { get; set; } = [];
        [YamlMember(Alias = "assignIdentityOnAdd")]
        public bool AssignIdentityOnAdd { get; set; }
        [YamlMember(Alias = "fields")]
        public Dictionary<string, string> Fields { get; set; } = [];
    }

    sealed class Step
    {
        [YamlMember(Alias = "CREATE")]
        public RowOp? Create { get; set; }
        [YamlMember(Alias = "READ")]
        public IdOp? Read { get; set; }
        [YamlMember(Alias = "QUERY")]
        public QueryOp? Query { get; set; }
        [YamlMember(Alias = "READY")]
        public WhereOp? Ready { get; set; }
        [YamlMember(Alias = "UPDATE")]
        public RowOp? Update { get; set; }
        [YamlMember(Alias = "UPDATEBY")]
        public UpdateByOp? UpdateBy { get; set; }
        [YamlMember(Alias = "DELETE")]
        public IdOp? Delete { get; set; }
        [YamlMember(Alias = "DELETEBY")]
        public WhereOp? DeleteBy { get; set; }
    }

    sealed class RowOp
    {
        [YamlMember(Alias = "row")]
        public Dictionary<string, object?> Row { get; set; } = [];
    }

    sealed class IdOp
    {
        [YamlMember(Alias = "id")]
        public object? Id { get; set; }
    }

    sealed class QueryOp
    {
        [YamlMember(Alias = "where")]
        public Dictionary<string, object?>? Where { get; set; }
    }

    sealed class WhereOp
    {
        [YamlMember(Alias = "where")]
        public Dictionary<string, object?> Where { get; set; } = [];
    }

    sealed class UpdateByOp
    {
        [YamlMember(Alias = "row")]
        public Dictionary<string, object?> Row { get; set; } = [];
        [YamlMember(Alias = "where")]
        public Dictionary<string, object?> Where { get; set; } = [];
    }
}
