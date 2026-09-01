using System.Collections;
using System.Globalization;
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
        var spec = new DeserializerBuilder()
            .WithNamingConvention(NullNamingConvention.Instance)
            .Build()
            .Deserialize<Spec>(File.ReadAllText(path));
        var session = open();
        await session.Connection.Open();
        await session.Command.Execute(SqliteStatementBuilder.CreateTable(spec.Type.Table, Columns(spec.Type)).GetText());
        _ = SqliteTypes.All;
        var types = DefaultTypeConverters.For(session.Connection.Provider);
        var fields = spec.Type.Fields
            .Where(pair => types.ContainsKey(pair.Value))
            .ToDictionary(pair => pair.Key, pair => types[pair.Value]);
        var mapping = new RowTypeConverter<SpecUser>(types, fields);
        var built = RepositoryBuilder<SpecUser>.Parts(new()
        {
            Connection = session.Connection,
            Table = spec.Type.Table,
            Mapping = mapping,
            Ids = spec.Type.Ids,
            AssignIdentityOnAdd = spec.Type.AssignIdentityOnAdd,
        });
        if (Verbose()) Console.WriteLine($"\n--- {spec.Name} ({session.Connection.Provider}) ---");
        var logs = new List<string>();
        var repo = new Repository<SpecUser>(built).Use(new ConsoleCommandMiddleware(line =>
        {
            logs.Add(line);
            if (Verbose()) Console.WriteLine(line);
        }, null, new Dictionary<string, string> { ["isVerbose"] = Verbose() ? "true" : "false" }));
        foreach (var step in spec.Steps)
            await (step switch
            {
                { Create: { } create } => repo.Add(mapping.Convert(ToRow(create.Row, spec.Type))),
                { Get: { } get } => AssertGet(spec.Name, get, repo.GetSingle(WhereBuilder.Equals("id", Value.From(get.Id)))),
                { Query: { } query } => AssertQuery(spec.Name, query, query.Where is { } where ? repo.Get(Where(mapping, where, spec.Type)) : repo.Get()),
                { GetBy: { } getBy } => AssertGet(spec.Name, getBy, repo.GetSingle(Where(mapping, getBy.Where, spec.Type))),
                { Update: { } update } => AssertAffected($"{spec.Name} UPDATE", update.HasExpect, update.Expect,
                    repo.Update(mapping.Convert(ToRow(update.Row, spec.Type)))),
                { UpdateBy: { } updateBy } => AssertAffected($"{spec.Name} UPDATEBY", updateBy.HasExpect, updateBy.Expect,
                    repo.Update(mapping.Convert(ToRow(updateBy.Row, spec.Type)), Where(mapping, updateBy.Where, spec.Type))),
                { Delete: { } delete } => AssertAffected($"{spec.Name} DELETE", delete.HasExpect, delete.Expect,
                    repo.Delete(mapping.Convert(ToRow(new() { ["id"] = delete.Id }, spec.Type)))),
                { DeleteBy: { } deleteBy } => AssertAffected($"{spec.Name} DELETEBY", deleteBy.HasExpect, deleteBy.Expect,
                    repo.Delete(new SpecUser(), Where(mapping, deleteBy.Where, spec.Type))),
                _ => throw new InvalidOperationException($"{spec.Name}: empty step")
            });
        SqlSpec.AssertConsoleLogs(
            spec.Name,
            Queries(spec),
            [.. logs.Where(line => line.Contains("[Start]") || line.Contains("[Finish]"))]);
    }

    static async Task AssertGet(string name, IdOp get, Task<SpecUser?> task)
    {
        var actual = await task;
        if (get.HasExpect) AssertRow($"{name} GET", actual, get.Expect);
    }

    static async Task AssertGet(string name, WhereOp getBy, Task<SpecUser?> task)
    {
        var actual = await task;
        if (getBy.HasExpect) AssertRow($"{name} GETBY", actual, getBy.Expect);
    }

    static async Task AssertAffected(string name, bool hasExpect, object? expect, Task<Response> task)
    {
        var response = await task;
        if (!hasExpect) return;
        var wanted = Affected(expect);
        if (response.RowsAffected != wanted)
            throw new InvalidOperationException($"{name}: expected {wanted} rows affected got {response.RowsAffected}");
    }

    static int Affected(object? expect)
    {
        var row = AsRow(expect ?? throw new InvalidOperationException("expect.rowsAffected required"));
        return row.TryGetValue("rowsAffected", out var value) && value is not null
            ? Convert.ToInt32(value, CultureInfo.InvariantCulture)
            : throw new InvalidOperationException("expect.rowsAffected required");
    }

    static async Task AssertQuery(string name, QueryOp query, IAsyncEnumerable<SpecUser> items)
    {
        var actual = new List<SpecUser>();
        await foreach (var item in items) actual.Add(item);
        if (!query.HasExpect) return;
        var expect = Rows(query.Expect);
        if (actual.Count != expect.Count)
            throw new InvalidOperationException($"{name} QUERY: expected {expect.Count} rows got {actual.Count}");
        for (var index = 0; index < actual.Count; index++)
            AssertRow($"{name} QUERY[{index}]", actual[index], expect[index]);
    }

    static void AssertRow(string name, SpecUser? actual, object? expect)
    {
        if (expect is null)
        {
            if (actual is not null) throw new InvalidOperationException($"{name}: expected null");
            return;
        }
        if (actual is null) throw new InvalidOperationException($"{name}: expected a row");
        var wanted = AsRow(expect);
        var got = FromUser(actual);
        foreach (var (key, value) in wanted)
            if (!Same(Norm(got.GetValueOrDefault(key)), Norm(value)))
                throw new InvalidOperationException($"{name}: {key}: expected {Norm(value)} got {Norm(got.GetValueOrDefault(key))}");
    }

    static Dictionary<string, object?> FromUser(SpecUser user) => new()
    {
        ["id"] = user.id,
        ["uuid"] = user.uuid.ToString(),
        ["username"] = user.username,
        ["is_enabled"] = user.is_enabled,
        ["balance"] = user.balance.ToString(CultureInfo.InvariantCulture),
        ["location"] = user.location,
        ["created"] = new Value.DateTime(user.created).IsoDateTime(),
        ["version"] = Convert.ToBase64String(user.version),
    };

    static object? Norm(object? value) => value switch
    {
        null => null,
        bool flag => flag,
        "true" or "True" => true,
        "false" or "False" => false,
        byte or sbyte or short or ushort or int or uint or long => Convert.ToInt64(value),
        float or double => Convert.ToDouble(value),
        decimal number => number.ToString(CultureInfo.InvariantCulture),
        Guid guid => guid.ToString(),
        DateTime date => new Value.DateTime(date).IsoDateTime(),
        byte[] bytes => Convert.ToBase64String(bytes),
        string text when text.Contains('T') && DateTime.TryParse(
            text, CultureInfo.InvariantCulture,
            DateTimeStyles.AdjustToUniversal | DateTimeStyles.AssumeUniversal, out var date) =>
            new Value.DateTime(DateTime.SpecifyKind(date, DateTimeKind.Utc)).IsoDateTime(),
        _ => value
    };

    static bool Same(object? got, object? wanted) =>
        Equals(got, wanted)
        || got is IConvertible && wanted is IConvertible
            && decimal.TryParse($"{got}", NumberStyles.Number, CultureInfo.InvariantCulture, out var left)
            && decimal.TryParse($"{wanted}", NumberStyles.Number, CultureInfo.InvariantCulture, out var right)
            && left == right;

    static List<object> Rows(object? expect) => expect switch
    {
        IEnumerable<object> items => [.. items],
        IEnumerable items => [.. items.Cast<object>()],
        _ => throw new InvalidOperationException("QUERY expect must be a list")
    };

    static Dictionary<string, object?> AsRow(object expect) => expect switch
    {
        Dictionary<string, object?> row => row,
        IDictionary<string, object?> row => new(row),
        IDictionary<object, object?> row => row.ToDictionary(pair => $"{pair.Key}", pair => pair.Value),
        IDictionary row => row.Cast<DictionaryEntry>().ToDictionary(pair => $"{pair.Key}", pair => pair.Value),
        _ => throw new InvalidOperationException("expect must be a row")
    };

    static IEnumerable<string> Queries(Spec spec)
    {
        var columns = spec.Type.Fields.Keys.OrderBy(key => key).ToArray();
        var ids = spec.Type.Ids.ToHashSet();
        foreach (var step in spec.Steps)
            yield return step switch
            {
                { Create: { } create } => StatementBuilder.CreateInsert(spec.Type.Table, Fields(create.Row, col => !spec.Type.AssignIdentityOnAdd || !ids.Contains(col))).GetText(),
                { Get: { } get } => StatementBuilder.CreateQuery(spec.Type.Table, columns, WhereBuilder.Equals("id", Value.From(get.Id))).GetText(),
                { Query: { } query } => StatementBuilder.CreateQuery(spec.Type.Table, columns, query.Where is { } where ? SqlWhere(where) : null).GetText(),
                { GetBy: { } getBy } => StatementBuilder.CreateQuery(spec.Type.Table, columns, SqlWhere(getBy.Where)).GetText(),
                { Update: { } update } => StatementBuilder.CreateUpdate(spec.Type.Table, Fields(update.Row, col => !ids.Contains(col)), WhereBuilder.Equals("id", Value.From(update.Row["id"]))).GetText(),
                { UpdateBy: { } updateBy } => StatementBuilder.CreateUpdate(spec.Type.Table, Fields(updateBy.Row, col => !ids.Contains(col)), SqlWhere(updateBy.Where)).GetText(),
                { Delete: { } delete } => StatementBuilder.CreateDelete(spec.Type.Table, WhereBuilder.Equals("id", Value.From(delete.Id))).GetText(),
                { DeleteBy: { } deleteBy } => StatementBuilder.CreateDelete(spec.Type.Table, SqlWhere(deleteBy.Where)).GetText(),
                _ => throw new InvalidOperationException($"{spec.Name}: empty step")
            };
    }

    static IReadOnlyList<Parameter> Fields(Dictionary<string, object?> row, Func<string, bool> include) =>
        row.Where(pair => include(pair.Key)).OrderBy(pair => pair.Key)
            .Select(pair => new Parameter(pair.Key, Value.From(Coerce(pair.Value)))).ToArray();

    static IWhere Where(RowTypeConverter<SpecUser> mapping, Dictionary<string, object?> row, TypeSpec type) =>
        WhereBuilder.And([.. row.Select(pair =>
        {
            var value = Value.From(Coerce(pair.Value, type.Fields.GetValueOrDefault(pair.Key)));
            var cell = mapping.Fields.TryGetValue(pair.Key, out var field)
                ? field.ReverseConvert(value)
                : value;
            return WhereBuilder.Equals(pair.Key, cell);
        })]);

    static IWhere SqlWhere(Dictionary<string, object?> row) =>
        WhereBuilder.And([.. row.Select(pair => WhereBuilder.Equals(pair.Key, Value.From(Coerce(pair.Value))))]);

    static Row ToRow(Dictionary<string, object?> row, TypeSpec type) =>
        new(row.ToDictionary(pair => pair.Key, pair =>
            Value.From(Coerce(pair.Value, type.Fields.GetValueOrDefault(pair.Key)))));

    static object? Coerce(object? value, string? canonical = null) => (canonical, value) switch
    {
        ("bool", "true") => true,
        ("bool", "false") => false,
        ("datetime", string text) when DateTime.TryParse(
            text, CultureInfo.InvariantCulture,
            DateTimeStyles.AdjustToUniversal | DateTimeStyles.AssumeUniversal, out var date) =>
            DateTime.SpecifyKind(date, DateTimeKind.Utc),
        ("binary", string text) => Convert.FromBase64String(text),
        (_, "true") => true,
        (_, "false") => false,
        _ => value
    };

    static SqliteColumn[] Columns(TypeSpec type) =>
        [.. type.Fields.Select(pair => new SqliteColumn(pair.Key, pair.Value, type.Ids.Contains(pair.Key)))];

    static bool Verbose() =>
        Environment.GetEnvironmentVariable("VERBOSE") is "1" or "true";

    public sealed record Session(IConnection Connection, ICommand Command);

    sealed class SpecUser
    {
        public int id { get; set; }
        public Guid uuid { get; set; }
        public string username { get; set; } = "";
        public bool is_enabled { get; set; }
        public decimal balance { get; set; }
        public double location { get; set; }
        public DateTime created { get; set; }
        public byte[] version { get; set; } = [];
    }

    sealed class Spec
    {
        [YamlMember(Alias = "name")] public string Name { get; set; } = "";
        [YamlMember(Alias = "type")] public TypeSpec Type { get; set; } = new();
        [YamlMember(Alias = "steps")] public List<Step> Steps { get; set; } = [];
    }

    sealed class TypeSpec
    {
        [YamlMember(Alias = "name")] public string Name { get; set; } = "";
        [YamlMember(Alias = "table")] public string Table { get; set; } = "";
        [YamlMember(Alias = "ids")] public List<string> Ids { get; set; } = [];
        [YamlMember(Alias = "assignIdentityOnAdd")] public bool AssignIdentityOnAdd { get; set; }
        [YamlMember(Alias = "fields")] public Dictionary<string, string> Fields { get; set; } = [];
    }

    sealed class Step
    {
        [YamlMember(Alias = "CREATE")] public RowOp? Create { get; set; }
        [YamlMember(Alias = "GET")] public IdOp? Get { get; set; }
        [YamlMember(Alias = "QUERY")] public QueryOp? Query { get; set; }
        [YamlMember(Alias = "GETBY")] public WhereOp? GetBy { get; set; }
        [YamlMember(Alias = "UPDATE")] public RowOp? Update { get; set; }
        [YamlMember(Alias = "UPDATEBY")] public UpdateByOp? UpdateBy { get; set; }
        [YamlMember(Alias = "DELETE")] public IdOp? Delete { get; set; }
        [YamlMember(Alias = "DELETEBY")] public WhereOp? DeleteBy { get; set; }
    }

    sealed class RowOp
    {
        [YamlMember(Alias = "row")] public Dictionary<string, object?> Row { get; set; } = [];
        object? expect;
        [YamlMember(Alias = "expect")]
        public object? Expect { get => expect; set { expect = value; HasExpect = true; } }
        public bool HasExpect { get; private set; }
    }

    sealed class IdOp
    {
        [YamlMember(Alias = "id")] public object? Id { get; set; }
        object? expect;
        [YamlMember(Alias = "expect")]
        public object? Expect { get => expect; set { expect = value; HasExpect = true; } }
        public bool HasExpect { get; private set; }
    }

    sealed class QueryOp
    {
        [YamlMember(Alias = "where")] public Dictionary<string, object?>? Where { get; set; }
        object? expect;
        [YamlMember(Alias = "expect")]
        public object? Expect { get => expect; set { expect = value; HasExpect = true; } }
        public bool HasExpect { get; private set; }
    }

    sealed class WhereOp
    {
        [YamlMember(Alias = "where")] public Dictionary<string, object?> Where { get; set; } = [];
        object? expect;
        [YamlMember(Alias = "expect")]
        public object? Expect { get => expect; set { expect = value; HasExpect = true; } }
        public bool HasExpect { get; private set; }
    }

    sealed class UpdateByOp
    {
        [YamlMember(Alias = "row")] public Dictionary<string, object?> Row { get; set; } = [];
        [YamlMember(Alias = "where")] public Dictionary<string, object?> Where { get; set; } = [];
        object? expect;
        [YamlMember(Alias = "expect")]
        public object? Expect { get => expect; set { expect = value; HasExpect = true; } }
        public bool HasExpect { get; private set; }
    }
}
