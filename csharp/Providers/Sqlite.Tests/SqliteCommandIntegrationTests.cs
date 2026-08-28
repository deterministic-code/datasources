using Datasources;
using Datasources.Sqlite;
using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;

namespace Datasources.Sqlite.Tests;

public class SqliteCommandIntegrationTests
{
    [Fact]
    public async Task SharedSpecs()
    {
        foreach (var path in Directory.GetFiles(SpecsDir(), "*.yaml"))
            await Run(path);
    }

    static string SpecsDir()
    {
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
        {
            var specs = Path.Combine(dir.FullName, "specs", "sqlite");
            if (Directory.Exists(specs)) return specs;
        }
        throw new DirectoryNotFoundException("specs/sqlite");
    }

    static async Task Run(string path)
    {
        var spec = new DeserializerBuilder()
            .WithNamingConvention(CamelCaseNamingConvention.Instance)
            .Build()
            .Deserialize<Spec>(File.ReadAllText(path));
        var names = spec.Connections is { Count: > 0 } ? spec.Connections : ["default"];
        var filename = Shared(spec.Filename, names.Count);
        var sessions = new Dictionary<string, Session>();
        foreach (var name in names)
        {
            var conn = new SqliteConnection(new SqliteConnectionOptions { Filename = filename, Driver = new SqliteDriver() });
            await conn.Open();
            sessions[name] = new Session(conn, new SqliteCommand(conn));
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
            var target = op.Open is not false
                ? sessions[op.Connection ?? names[0]].Command
                : new SqliteCommand(new SqliteConnection(new SqliteConnectionOptions { Driver = new SqliteDriver() }));
            var label = $"{spec.Name}: {op.Sql.Trim()}";
            try
            {
                if (step.Execute is not null)
                {
                    var response = await target.Execute(op.Sql, Params(op.Parameters));
                    if (op.Expect?.Error is { } error) throw new InvalidOperationException($"{label}: expected {error}");
                    if (op.Expect?.RowsAffected is { } rows) Eq(rows, response.RowsAffected);
                    if (op.Expect?.Result is not null) Eq(op.Expect.Result, response.Result);
                }
                else
                {
                    var reader = await target.ExecuteReader(op.Sql, Params(op.Parameters));
                    if (op.Expect?.Error is { } error) throw new InvalidOperationException($"{label}: expected {error}");
                    if (op.Expect?.Rows is { } expected)
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
    }

    static string Shared(string filename, int connections) =>
        connections > 1
            ? Path.Combine(Path.GetTempPath(), $"ds-{Guid.NewGuid():N}.db")
            : filename;

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
        return decimal.TryParse(value.ToString(), out number);
    }

    sealed record Session(SqliteConnection Connection, SqliteCommand Command);

    sealed class Spec
    {
        public string Name { get; set; } = "";
        public string Filename { get; set; } = ":memory:";
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
        public List<Dictionary<string, object?>>? Rows { get; set; }
        public string? Error { get; set; }
    }
}
