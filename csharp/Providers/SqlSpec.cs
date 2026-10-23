using Datasources;
using YamlDotNet.RepresentationModel;
using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;

namespace Datasources.SqlSpecs;

public static class SqlSpec
{
    public static string SpecsDir() => Datasources.CodeGenerators.SpecParser.SpecsDir();

    public static async Task Run(
        string path,
        Dialect dialect,
        Func<string, Session> openSession,
        Func<ICommand> closedCommand)
    {
        var spec = new DeserializerBuilder()
            .WithNamingConvention(CamelCaseNamingConvention.Instance)
            .Build()
            .Deserialize<Spec>(File.ReadAllText(path));
        var names = spec.Connections is { Count: > 0 } ? spec.Connections : ["default"];
        var target = dialect.Target(names.Count);
        var logs = new List<string>();
        var middleware = new ConsoleCommandMiddleware(logs.Add);
        ICommand Attach(ICommand command) => command.AddMiddleware(middleware);
        var sessions = new Dictionary<string, Session>();
        foreach (var name in names)
        {
            var session = openSession(target);
            await session.Connection.Open();
            Attach(session.Command);
            sessions[name] = session;
        }
        var txs = new Dictionary<string, ITransaction>();
        foreach (var step in spec.Steps)
            await (step switch
            {
                { Close: not null } => sessions[names[0]].Connection.Close(),
                { StartTransaction: { } start } => Start(start),
                { Commit: { } commit } => txs[commit.Transaction ?? "default"].Commit(),
                { Abort: { } abort } => txs[abort.Transaction ?? "default"].Abort(),
                _ => Exec(step)
            });
        AssertConsoleLogs(spec.Name, spec.Steps
            .Select(step => step.Execute?.Sql ?? step.ExecuteReader?.Sql)
            .OfType<string>()
            .Select(sql => sql.Trim()), logs);

        async Task Start(Tx start)
        {
            txs[start.As ?? "default"] = await sessions[start.Connection ?? names[0]].Connection
                .StartTransaction(Iso(start.Isolation));
        }

        async Task Exec(Step step)
        {
            var op = step.Execute ?? step.ExecuteReader ?? throw new InvalidOperationException($"{path}: empty step");
            op.Sql = dialect.ResolveSql(op.Sql);
            var expectedRows = dialect.ResolveRows(op.Expect?.Rows);
            var cmd = op.Open is not false ? sessions[op.Connection ?? names[0]].Command : Attach(closedCommand());
            var label = $"{spec.Name}: {op.Sql.Trim()}";
            try
            {
                if (step.Execute is not null)
                {
                    var response = await cmd.Execute(new Native(op.Sql), Params(op.Parameters));
                    if (op.Expect?.Error is { } error) throw new InvalidOperationException($"{label}: expected {error}");
                    if (op.Expect?.RowsAffected is { } rows) Eq(rows, response.RowsAffected);
                    if (op.Expect?.Result is not null) Eq(op.Expect.Result, response.Result);
                    return;
                }
                var reader = await cmd.ExecuteReader(new Native(op.Sql), Params(op.Parameters));
                try
                {
                    if (op.Expect?.Error is { } err) throw new InvalidOperationException($"{label}: expected {err}");
                    if (expectedRows is not { } expected) return;
                    var actual = new List<Row>();
                    while (await reader.Next()) actual.Add((Row)reader.Current!);
                    Assert.Equal(expected.Count, actual.Count);
                    for (var i = 0; i < expected.Count; i++)
                        foreach (var (key, value) in expected[i]) Eq(value, actual[i][key]);
                }
                finally
                {
                    if (reader is IAsyncDisposable disposable)
                        await disposable.DisposeAsync();
                }
            }
            catch (DatasourceException ex) when (op.Expect?.Error is { } expected)
            {
                Assert.Contains(expected, ex.Message);
            }
        }
    }

    internal static void AssertConsoleLogs(string name, IEnumerable<string> queries, List<string> logs)
    {
        var expected = queries.SelectMany(query => new[] { $"[datasource][Start]-[{query}]", $"[datasource][Finish]-[{query}]" }).ToList();
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
            $"{name}: first logging mismatch at {mismatch}\nexpected: {(mismatch < expected.Count ? expected[mismatch] : "<missing>")}\nactual:   {(mismatch < actual.Count ? actual[mismatch] : "<missing>")}\n--- expected ---\n{string.Join("\n", expected)}\n--- actual ---\n{string.Join("\n", actual)}");
    }

    static string Signature(string line)
    {
        var body = line.Contains("]-") ? line[(line.IndexOf("]-", StringComparison.Ordinal) + 2)..] : line;
        var duration = body.LastIndexOf("] ", StringComparison.Ordinal);
        return duration >= 0 && (body.EndsWith("ms", StringComparison.Ordinal) || body.EndsWith("μs", StringComparison.Ordinal))
            ? body[..(duration + 1)] : body;
    }

    static IsolationLevel Iso(string? name) => name switch
    {
        "readUncommitted" => IsolationLevel.ReadUncommitted,
        "readCommitted" => IsolationLevel.ReadCommitted,
        "repeatableRead" => IsolationLevel.RepeatableRead,
        _ => IsolationLevel.Serializable
    };

    static IReadOnlyList<Parameter> Params(Dictionary<string, object?>? map) =>
        map?.Select(p => new Parameter(p.Key, Value.From(Coerce(p.Value)))).ToList() ?? [];

    static object? Coerce(object? value) =>
        TryNumber(value, out var n) && n == decimal.Truncate(n) && n is >= int.MinValue and <= int.MaxValue ? (int)n : value;

    static void Eq(object? expected, object? actual)
    {
        if (actual is Value actualValue) actual = actualValue.ToObject();
        if (expected is Value expectedValue) expected = expectedValue.ToObject();
        if (TryNumber(expected, out var left) && TryNumber(actual, out var right)) Assert.Equal(left, right);
        else Assert.Equal(expected?.ToString(), actual?.ToString());
    }

    static bool TryNumber(object? value, out decimal number)
    {
        number = 0;
        if (value is null or bool) return false;
        if (value is IConvertible convertible)
            try { number = convertible.ToDecimal(null); return true; }
            catch { }
        return decimal.TryParse(value?.ToString(), out number);
    }

    public sealed record Session(IConnection Connection, ICommand Command);

    public sealed class Dialect
    {
        public ConnectionMeta Connection { get; init; } = new();
        public string Engine { get; init; } = "";
        public Func<CreateTableStatement, string> Render { get; init; } = static statement => DefaultStatementTextRenderer.Instance.Render(statement);
        public string CreateUserTable { get; init; } = "";
        public string CreateUserTableVersioned { get; init; } = "";
        public List<Dictionary<string, Value>> DirtyReadRows { get; init; } = [];

        public static Dialect Load(string engine, Func<CreateTableStatement, string> render)
        {
            var yaml = new YamlStream();
            yaml.Load(new StringReader(File.ReadAllText(Path.Combine(SpecsDir(), "dialects", $"{engine}.yaml"))));
            var map = (YamlMappingNode)yaml.Documents[0].RootNode;
            string Scalar(string key) => ((YamlScalarNode)map.Children[new YamlScalarNode(key)]).Value ?? "";
            var conn = (YamlMappingNode)map.Children[new YamlScalarNode("connection")];
            string Conn(string key) =>
                conn.Children.TryGetValue(new YamlScalarNode(key), out var node) ? ((YamlScalarNode)node).Value ?? "" : "";
            var dirty = map.Children.TryGetValue(new YamlScalarNode("dirtyReadRows"), out var dirtyNode) && dirtyNode is YamlSequenceNode seq
                ? seq.Children.OfType<YamlMappingNode>()
                    .Select(item => item.Children.ToDictionary(
                        kv => ((YamlScalarNode)kv.Key).Value!,
                        kv => Value.From(ParseScalar((YamlScalarNode)kv.Value))))
                    .ToList()
                : [];
            return new Dialect
            {
                Engine = engine,
                Render = render,
                Connection = new ConnectionMeta { Kind = Conn("kind"), Filename = Conn("filename"), Env = Conn("env") },
                CreateUserTable = Scalar("createUserTable"),
                CreateUserTableVersioned = Scalar("createUserTableVersioned"),
                DirtyReadRows = dirty,
            };
        }

        static object? ParseScalar(YamlScalarNode node) =>
            node.Value is null ? null : int.TryParse(node.Value, out var n) ? n : node.Value;

        public string Target(int connections) =>
            Connection.Kind == "url"
                ? Environment.GetEnvironmentVariable(Connection.Env) is { Length: > 0 } url
                    ? url : throw new InvalidOperationException($"{Connection.Env} is not set")
                : connections > 1
                    ? Path.Combine(Path.GetTempPath(), $"ds-{Guid.NewGuid():N}.db")
                    : string.IsNullOrEmpty(Connection.Filename) ? ":memory:" : Connection.Filename;

        public string ResolveSql(string sql) => sql.Trim() switch
        {
            "createUserTable" => Schema(CreateUserTable),
            "createUserTableVersioned" => Schema(CreateUserTableVersioned),
            _ => sql
        };

        string Schema(string name) =>
            string.Join(";\n", SchemaSpec.Tables(name, static id => $"\"{id.Replace("\"", "\"\"")}\"").Select(Render));

        public List<Row>? ResolveRows(object? rows) => rows switch
        {
            "dirtyReadRows" => [.. DirtyReadRows.Select(row => row as Row ?? new Row(row))],
            List<Row> list => list,
            List<Dictionary<string, Value>> list => [.. list.Select(row => row as Row ?? new Row(row))],
            System.Collections.IEnumerable items when rows is not string =>
                [.. items.Cast<object>().Select(ItemRow).OfType<Row>()],
            _ => null
        };

        static Row? ItemRow(object item) => item switch
        {
            Row row => row,
            Dictionary<string, Value> typed => new Row(typed),
            System.Collections.IDictionary map => new Row(map.Keys.Cast<object>().ToDictionary(
                key => key.ToString()!,
                key => Value.From(map[key]))),
            _ => null
        };
    }

    public sealed class ConnectionMeta
    {
        public string Kind { get; init; } = "file";
        public string Filename { get; init; } = "";
        public string Env { get; init; } = "";
    }

    sealed class Spec
    {
        public string Name { get; set; } = "";
        public List<string>? Connections { get; set; }
        public List<Step> Steps { get; set; } = [];
    }

    sealed class Step
    {
        public Op? Execute { get; set; }
        public Op? ExecuteReader { get; set; }
        public Tx? StartTransaction { get; set; }
        public TxEnd? Commit { get; set; }
        public TxEnd? Abort { get; set; }
        public object? Close { get; set; }
    }

    sealed class Tx
    {
        public string? Connection { get; set; }
        public string? Isolation { get; set; }
        public string? As { get; set; }
    }

    sealed class TxEnd
    {
        public string? Transaction { get; set; }
    }

    sealed class Op
    {
        public string Sql { get; set; } = "";
        public string? Connection { get; set; }
        public Dictionary<string, object?>? Parameters { get; set; }
        public bool? Open { get; set; }
        public Expect? Expect { get; set; }
    }

    sealed class Expect
    {
        public int? RowsAffected { get; set; }
        public object? Result { get; set; }
        public object? Rows { get; set; }
        public string? Error { get; set; }
    }
}
