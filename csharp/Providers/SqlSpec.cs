using Datasources;
using YamlDotNet.RepresentationModel;
using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;

namespace Datasources.SqlSpecs;

public static class SqlSpec
{
    public static string SpecsDir()
    {
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
        {
            var specs = Path.Combine(dir.FullName, "specs");
            if (File.Exists(Path.Combine(specs, "command-crud.yaml"))) return specs;
        }
        throw new DirectoryNotFoundException("specs");
    }

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
        ICommand Attach(ICommand command) => command.Use(middleware);
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
        {
            if (step.Close is not null) { await sessions[names[0]].Connection.Close(); continue; }
            if (step.StartTransaction is { } start)
            {
                var session = sessions[start.Connection ?? names[0]];
                txs[start.As ?? "default"] = await session.Connection.StartTransaction(Iso(start.Isolation));
                continue;
            }
            if (step.Commit is { } commit) { await txs[commit.Transaction ?? "default"].Commit(); continue; }
            if (step.Abort is { } abort) { await txs[abort.Transaction ?? "default"].Abort(); continue; }
            var op = step.Execute ?? step.ExecuteReader ?? throw new InvalidOperationException($"{path}: empty step");
            op.Sql = dialect.ResolveSql(op.Sql);
            var expectedRows = dialect.ResolveRows(op.Expect?.Rows);
            var targetCmd = op.Open is not false
                ? sessions[op.Connection ?? names[0]].Command
                : Attach(closedCommand());
            var label = $"{spec.Name}: {op.Sql.Trim()}";
            try
            {
                if (step.Execute is not null)
                {
                    var response = await targetCmd.Execute(op.Sql, Params(op.Parameters));
                    if (op.Expect?.Error is { } error) throw new InvalidOperationException($"{label}: expected {error}");
                    if (op.Expect?.RowsAffected is { } rows) Eq(rows, response.RowsAffected);
                    if (op.Expect?.Result is not null) Eq(op.Expect.Result, response.Result);
                }
                else
                {
                    var reader = await targetCmd.ExecuteReader(op.Sql, Params(op.Parameters));
                    if (op.Expect?.Error is { } error) throw new InvalidOperationException($"{label}: expected {error}");
                    if (expectedRows is { } expected)
                    {
                        var actual = new List<IReadOnlyDictionary<string, object?>>();
                        while (await reader.Next())
                            actual.Add((IReadOnlyDictionary<string, object?>)reader.Current!);
                        Assert.Equal(expected.Count, actual.Count);
                        for (var i = 0; i < expected.Count; i++)
                            foreach (var (key, value) in expected[i])
                                Eq(value, actual[i][key]);
                    }
                }
            }
            catch (DatasourceException ex) when (op.Expect?.Error is { } expected)
            {
                Assert.Contains(expected, ex.Message);
            }
        }
        AssertLogs(spec, logs);
    }

    static void AssertLogs(Spec spec, List<string> logs)
    {
        var expected = spec.Steps
            .Select(step => step.Execute?.Sql ?? step.ExecuteReader?.Sql)
            .Where(sql => sql is not null)
            .Select(sql => sql!.Trim())
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

    static string Signature(string line)
    {
        var body = line.Contains("]-") ? line[(line.IndexOf("]-", StringComparison.Ordinal) + 2)..] : line;
        var duration = body.LastIndexOf("] ", StringComparison.Ordinal);
        return duration >= 0 && (body.EndsWith("ms", StringComparison.Ordinal) || body.EndsWith("μs", StringComparison.Ordinal))
            ? body[..(duration + 1)]
            : body;
    }

    static IsolationLevel Iso(string? name) => name switch
    {
        "readUncommitted" => IsolationLevel.ReadUncommitted,
        "readCommitted" => IsolationLevel.ReadCommitted,
        "repeatableRead" => IsolationLevel.RepeatableRead,
        _ => IsolationLevel.Serializable
    };

    static IReadOnlyList<Parameter> Params(Dictionary<string, object?>? map) =>
        map?.Select(p => new Parameter(p.Key, Coerce(p.Value))).ToList() ?? [];

    static object? Coerce(object? value) =>
        TryNumber(value, out var n) && n == decimal.Truncate(n) && n is >= int.MinValue and <= int.MaxValue ? (int)n : value;

    static void Eq(object? expected, object? actual)
    {
        if (TryNumber(expected, out var left) && TryNumber(actual, out var right))
            Assert.Equal(left, right);
        else
            Assert.Equal(expected?.ToString(), actual?.ToString());
    }

    static bool TryNumber(object? value, out decimal number)
    {
        number = 0;
        if (value is null or bool) return false;
        if (value is IConvertible convertible)
        {
            try { number = convertible.ToDecimal(null); return true; }
            catch { /* fall through */ }
        }
        return decimal.TryParse(value?.ToString(), out number);
    }

    public sealed record Session(IConnection Connection, ICommand Command);

    public sealed class Dialect
    {
        public ConnectionMeta Connection { get; init; } = new();
        public string CreateUserTable { get; init; } = "";
        public string CreateUserTableVersioned { get; init; } = "";
        public List<Dictionary<string, object?>> DirtyReadRows { get; init; } = [];

        public static Dialect Load(string engine)
        {
            var root = SpecsDir();
            var raw = File.ReadAllText(Path.Combine(root, "dialects", $"{engine}.yaml"));
            var yaml = new YamlStream();
            yaml.Load(new StringReader(raw));
            var map = (YamlMappingNode)yaml.Documents[0].RootNode;
            string Scalar(string key) => ((YamlScalarNode)map.Children[new YamlScalarNode(key)]).Value ?? "";
            var conn = (YamlMappingNode)map.Children[new YamlScalarNode("connection")];
            string Conn(string key) =>
                conn.Children.TryGetValue(new YamlScalarNode(key), out var node)
                    ? ((YamlScalarNode)node).Value ?? ""
                    : "";
            var dirty = new List<Dictionary<string, object?>>();
            if (map.Children.TryGetValue(new YamlScalarNode("dirtyReadRows"), out var dirtyNode)
                && dirtyNode is YamlSequenceNode seq)
            {
                foreach (var item in seq.Children.OfType<YamlMappingNode>())
                {
                    var row = new Dictionary<string, object?>();
                    foreach (var (k, v) in item.Children)
                        row[((YamlScalarNode)k).Value!] = ParseScalar((YamlScalarNode)v);
                    dirty.Add(row);
                }
            }
            return new Dialect
            {
                Connection = new ConnectionMeta
                {
                    Kind = Conn("kind"),
                    Filename = Conn("filename"),
                    Env = Conn("env"),
                },
                CreateUserTable = Scalar("createUserTable"),
                CreateUserTableVersioned = Scalar("createUserTableVersioned"),
                DirtyReadRows = dirty,
            };
        }

        static object? ParseScalar(YamlScalarNode node)
        {
            var text = node.Value;
            if (text is null) return null;
            if (int.TryParse(text, out var n)) return n;
            return text;
        }

        public string Target(int connections)
        {
            if (Connection.Kind == "url")
            {
                var url = Environment.GetEnvironmentVariable(Connection.Env);
                return string.IsNullOrEmpty(url)
                    ? throw new InvalidOperationException($"{Connection.Env} is not set")
                    : url;
            }
            return connections > 1
                ? Path.Combine(Path.GetTempPath(), $"ds-{Guid.NewGuid():N}.db")
                : (string.IsNullOrEmpty(Connection.Filename) ? ":memory:" : Connection.Filename);
        }

        public string ResolveSql(string sql)
        {
            var key = sql.Trim();
            return key switch
            {
                "createUserTable" => CreateUserTable,
                "createUserTableVersioned" => CreateUserTableVersioned,
                _ => sql
            };
        }

        public List<Dictionary<string, object?>>? ResolveRows(object? rows)
        {
            if (rows is string name && name == "dirtyReadRows") return DirtyReadRows;
            if (rows is List<Dictionary<string, object?>> list) return list;
            if (rows is not System.Collections.IEnumerable items || rows is string) return null;
            var result = new List<Dictionary<string, object?>>();
            foreach (var item in items)
            {
                if (item is Dictionary<string, object?> typed) result.Add(typed);
                else if (item is System.Collections.IDictionary map)
                {
                    var row = new Dictionary<string, object?>();
                    foreach (System.Collections.DictionaryEntry entry in map)
                        row[entry.Key!.ToString()!] = entry.Value;
                    result.Add(row);
                }
            }
            return result;
        }
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
