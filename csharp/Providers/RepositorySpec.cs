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
        await session.Command.Execute(SqliteStatementBuilder.CreateTable(PhysicalTable(spec.Type), Columns(spec.Type)).GetText());
        _ = SqliteTypes.All;
        var types = DefaultTypeConverters.For(session.Connection.Provider);
        var fields = FieldsOf(spec.Type, types);
        if (Verbose()) Console.WriteLine($"\n--- {spec.Name} ({session.Connection.Provider}) ---");
        var logs = new List<string>();
        if (spec.Type.Mappings.Count == 0)
            await Run(spec, session, new RowTypeConverter<SpecUser>(types, fields), logs);
        else
            await Run(spec, session, new RowTypeConverter<MappedUser>(types, fields), logs);
        SqlSpec.AssertConsoleLogs(
            spec.Name,
            Queries(spec),
            [.. logs.Where(line => line.Contains("[Start]") || line.Contains("[Finish]"))]);
    }

    static async Task Run<T>(Spec spec, Session session, RowTypeConverter<T> mapping, List<string> logs)
        where T : new()
    {
        var built = RepositoryBuilder<T>.Parts(new()
        {
            Connection = session.Connection,
            Entity = spec.Type.Table,
            Mapping = mapping,
            StatementMapper = StatementMapperOf(spec.Type, MappingsMapper<T>(spec.Type.Mappings)),
            Ids = spec.Type.Ids,
            AssignIdentityOnAdd = spec.Type.AssignIdentityOnAdd,
            Columns = [.. spec.Type.Fields.Keys],
        });
        var repo = new Repository<T>(built).Use(new ConsoleCommandMiddleware(line =>
        {
            logs.Add(line);
            if (Verbose()) Console.WriteLine(line);
        }, null, new Dictionary<string, string> { ["isVerbose"] = Verbose() ? "true" : "false" }));
        foreach (var step in spec.Steps)
            await (step switch
            {
                { Create: { } create } => repo.Add(ToEntity(create.Row, spec.Type, mapping)),
                { Get: { } get } => AssertGet(spec.Name, get, repo.GetSingle(WhereBuilder.Equals("id", Value.From(get.Id)))),
                { Query: { } query } => AssertQuery(spec.Name, query, query.Where is { } where ? repo.Get(Where(mapping, where, spec.Type)) : repo.Get()),
                { GetBy: { } getBy } => AssertGet(spec.Name, getBy, repo.GetSingle(Where(mapping, getBy.Where, spec.Type))),
                { Update: { } update } => AssertAffected($"{spec.Name} UPDATE", update.HasExpect, update.Expect,
                    repo.Update(ToEntity(update.Row, spec.Type, mapping))),
                { UpdateBy: { } updateBy } => AssertAffected($"{spec.Name} UPDATEBY", updateBy.HasExpect, updateBy.Expect,
                    repo.Update(ToEntity(updateBy.Row, spec.Type, mapping), Where(mapping, updateBy.Where, spec.Type))),
                { Delete: { } delete } => AssertAffected($"{spec.Name} DELETE", delete.HasExpect, delete.Expect,
                    repo.Delete(ToEntity(new() { ["id"] = delete.Id }, spec.Type, mapping))),
                { DeleteBy: { } deleteBy } => AssertAffected($"{spec.Name} DELETEBY", deleteBy.HasExpect, deleteBy.Expect,
                    repo.Delete(new T(), Where(mapping, deleteBy.Where, spec.Type))),
                _ => throw new InvalidOperationException($"{spec.Name}: empty step")
            });
    }

    static async Task AssertGet<T>(string name, IdOp get, Task<T?> task)
    {
        var actual = await task;
        if (get.HasExpect) AssertRow($"{name} GET", actual, get.Expect);
    }

    static async Task AssertGet<T>(string name, WhereOp getBy, Task<T?> task)
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

    static async Task AssertQuery<T>(string name, QueryOp query, IAsyncEnumerable<T> items)
    {
        var actual = new List<T>();
        await foreach (var item in items) actual.Add(item);
        if (!query.HasExpect) return;
        var expect = Rows(query.Expect);
        if (actual.Count != expect.Count)
            throw new InvalidOperationException($"{name} QUERY: expected {expect.Count} rows got {actual.Count}");
        for (var index = 0; index < actual.Count; index++)
            AssertRow($"{name} QUERY[{index}]", actual[index], expect[index]);
    }

    static void AssertRow(string name, object? actual, object? expect)
    {
        if (expect is null)
        {
            if (actual is not null) throw new InvalidOperationException($"{name}: expected null");
            return;
        }
        if (actual is null) throw new InvalidOperationException($"{name}: expected a row");
        foreach (var (key, value) in AsRow(expect))
        {
            var got = actual.GetType().GetProperty(key)?.GetValue(actual);
            if (value is IDictionary)
            {
                AssertRow($"{name}.{key}", got, value);
                continue;
            }
            if (!Same(Norm(got), Norm(value)))
                throw new InvalidOperationException($"{name}: {key}: expected {Norm(value)} got {Norm(got)}");
        }
    }

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

    static string PhysicalTable(TypeSpec type) =>
        type.StatementMapper?.Entity ?? type.Table;

    static string WithStatementSql(string sql, bool write, TypeSpec type) =>
        $"{sql}{(write ? type.StatementMapper?.WriteSuffix : type.StatementMapper?.QuerySuffix) ?? ""}";

    static StatementMapper<T> StatementMapperOf<T>(TypeSpec type, IMapper<T, T> rows)
        where T : new() =>
        new()
        {
            Entity = new TransformMapper<string, string>(
                name => type.StatementMapper?.Entity ?? name,
                name => name),
            Rows = rows,
            Query = Suffix<IQuery>(
                (text, query) => new Query(text, query.Where),
                type.StatementMapper?.QuerySuffix),
            Insert = Suffix<IInsertStatement>(
                (text, _) => new InsertStatement(text),
                type.StatementMapper?.WriteSuffix),
            Update = Suffix<IUpdateStatement>(
                (text, update) => new UpdateStatement(text, update.Where),
                type.StatementMapper?.WriteSuffix),
            Delete = Suffix<IDeleteStatement>(
                (text, delete) => new DeleteStatement(text, delete.Where),
                type.StatementMapper?.WriteSuffix)
        };

    static IMapper<T, T>? Suffix<T>(Func<string, T, T> rewrite, string? suffix)
        where T : IStatement =>
        suffix is null
            ? null
            : new TransformMapper<T, T>(built => rewrite($"{built.GetText()}{suffix}", built), built => built);

    static IEnumerable<string> Queries(Spec spec)
    {
        var columns = spec.Type.Fields.Keys.OrderBy(key => key).ToArray();
        var ids = spec.Type.Ids.ToHashSet();
        var table = PhysicalTable(spec.Type);
        foreach (var step in spec.Steps)
            yield return step switch
            {
                { Create: { } create } => WithStatementSql(StatementBuilder.CreateInsert(table, Fields(Flatten(create.Row, spec.Type), col => !spec.Type.AssignIdentityOnAdd || !ids.Contains(col))).GetText(), true, spec.Type),
                { Get: { } get } => WithStatementSql(StatementBuilder.CreateQuery(table, columns, WhereBuilder.Equals("id", Value.From(get.Id))).GetText(), false, spec.Type),
                { Query: { } query } => WithStatementSql(StatementBuilder.CreateQuery(table, columns, query.Where is { } where ? SqlWhere(where) : null).GetText(), false, spec.Type),
                { GetBy: { } getBy } => WithStatementSql(StatementBuilder.CreateQuery(table, columns, SqlWhere(getBy.Where)).GetText(), false, spec.Type),
                { Update: { } update } => WithStatementSql(StatementBuilder.CreateUpdate(table, Fields(Flatten(update.Row, spec.Type), col => !ids.Contains(col)), WhereBuilder.Equals("id", Value.From(Flatten(update.Row, spec.Type)["id"]))).GetText(), true, spec.Type),
                { UpdateBy: { } updateBy } => WithStatementSql(StatementBuilder.CreateUpdate(table, Fields(Flatten(updateBy.Row, spec.Type), col => !ids.Contains(col)), SqlWhere(updateBy.Where)).GetText(), true, spec.Type),
                { Delete: { } delete } => WithStatementSql(StatementBuilder.CreateDelete(table, WhereBuilder.Equals("id", Value.From(delete.Id))).GetText(), true, spec.Type),
                { DeleteBy: { } deleteBy } => WithStatementSql(StatementBuilder.CreateDelete(table, SqlWhere(deleteBy.Where)).GetText(), true, spec.Type),
                _ => throw new InvalidOperationException($"{spec.Name}: empty step")
            };
    }

    static Dictionary<string, object?> Flatten(Dictionary<string, object?> row, TypeSpec type)
    {
        var dest = new Dictionary<string, object?>(row);
        foreach (var (nested, cols) in type.Mappings)
        {
            if (!dest.TryGetValue(nested, out var value) || value is not IDictionary)
                continue;
            var nest = AsRow(value);
            foreach (var (field, col) in cols)
                dest[col] = nest.GetValueOrDefault(field);
            dest.Remove(nested);
        }
        return dest;
    }

    static IReadOnlyList<Parameter> Fields(Dictionary<string, object?> row, Func<string, bool> include) =>
        row.Where(pair => include(pair.Key)).OrderBy(pair => pair.Key)
            .Select(pair => new Parameter(pair.Key, Value.From(Coerce(pair.Value)))).ToArray();

    static IWhere Where<T>(RowTypeConverter<T> mapping, Dictionary<string, object?> row, TypeSpec type)
        where T : new() =>
        WhereBuilder.And([.. row.Select(pair =>
        {
            var value = Value.From(Coerce(pair.Value, Canonical(type, pair.Key)));
            var cell = mapping.Fields.TryGetValue(pair.Key, out var field)
                ? field.ReverseConvert(value)
                : value;
            return WhereBuilder.Equals(pair.Key, cell);
        })]);

    static IWhere SqlWhere(Dictionary<string, object?> row) =>
        WhereBuilder.And([.. row.Select(pair => WhereBuilder.Equals(pair.Key, Value.From(Coerce(pair.Value))))]);

    static T ToEntity<T>(Dictionary<string, object?> row, TypeSpec type, RowTypeConverter<T> mapping)
        where T : new()
    {
        var flat = row.Where(pair => pair.Value is not IDictionary)
            .ToDictionary(pair => pair.Key, pair => pair.Value);
        var entity = mapping.Convert(ToRow(flat, type));
        foreach (var (nested, cols) in type.Mappings)
        {
            if (!row.TryGetValue(nested, out var value) || value is not IDictionary)
                continue;
            var dest = typeof(T).GetProperty(nested);
            if (dest is null) continue;
            var child = Activator.CreateInstance(dest.PropertyType)
                ?? throw new InvalidOperationException($"Cannot create {dest.PropertyType}");
            var nest = AsRow(value);
            foreach (var (field, _) in cols)
            {
                var prop = dest.PropertyType.GetProperty(field);
                if (prop is null) continue;
                var raw = nest.GetValueOrDefault(field);
                prop.SetValue(child, raw is null ? null : Convert.ChangeType(raw, prop.PropertyType, CultureInfo.InvariantCulture));
            }
            dest.SetValue(entity, child);
        }
        return entity;
    }

    static IMapper<T, T> MappingsMapper<T>(Dictionary<string, Dictionary<string, string>> mappings)
        where T : new() =>
        mappings.Count == 0
            ? new TransformMapper<T, T>(static value => value, static value => value)
            : new TransformMapper<T, T>(
                row =>
                {
                    foreach (var (nested, cols) in mappings)
                    {
                        var dest = typeof(T).GetProperty(nested);
                        if (dest is null) continue;
                        var child = Activator.CreateInstance(dest.PropertyType)
                            ?? throw new InvalidOperationException($"Cannot create {dest.PropertyType}");
                        foreach (var (field, col) in cols)
                        {
                            var source = typeof(T).GetProperty(col)?.GetValue(row);
                            dest.PropertyType.GetProperty(field)?.SetValue(child, source);
                        }
                        dest.SetValue(row, child);
                    }
                    return row;
                },
                user =>
                {
                    foreach (var (nested, cols) in mappings)
                    {
                        var child = typeof(T).GetProperty(nested)?.GetValue(user);
                        foreach (var (field, col) in cols)
                        {
                            var value = child?.GetType().GetProperty(field)?.GetValue(child);
                            typeof(T).GetProperty(col)?.SetValue(user, value);
                        }
                    }
                    return user;
                });

    static Dictionary<string, TypeConverter<Value, Value>> FieldsOf(
        TypeSpec type, IReadOnlyDictionary<string, TypeConverter<Value, Value>> types)
    {
        var fields = type.Fields
            .Where(pair => types.ContainsKey(pair.Value))
            .ToDictionary(pair => pair.Key, pair => types[pair.Value]);
        foreach (var (name, canonical) in type.Converters)
            if (types.TryGetValue(canonical, out var converter))
                fields[name] = converter;
        return fields;
    }

    static Row ToRow(Dictionary<string, object?> row, TypeSpec type) =>
        new(row.ToDictionary(pair => pair.Key, pair =>
            Value.From(Coerce(pair.Value, Canonical(type, pair.Key)))));

    static string? Canonical(TypeSpec type, string key) =>
        type.Converters.GetValueOrDefault(key) ?? type.Fields.GetValueOrDefault(key);

    static object? Coerce(object? value, string? canonical = null) => (canonical, value) switch
    {
        ("bool", "true") => true,
        ("bool", "false") => false,
        ("decimal", string text) => decimal.Parse(text, CultureInfo.InvariantCulture),
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

    sealed class UserType
    {
        public string name { get; set; } = "";
    }

    sealed class MappedUser
    {
        public int id { get; set; }
        public Guid uuid { get; set; }
        public string username { get; set; } = "";
        public string usertypename { get; set; } = "";
        public decimal decimal_text { get; set; }
        public DateTime create_string { get; set; }
        public byte[] version { get; set; } = [];
        public UserType usertype { get; set; } = new();
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
        [YamlMember(Alias = "converters")] public Dictionary<string, string> Converters { get; set; } = [];
        [YamlMember(Alias = "mappings")] public Dictionary<string, Dictionary<string, string>> Mappings { get; set; } = [];
        [YamlMember(Alias = "statementMapper")] public StatementMapperSpec? StatementMapper { get; set; }
    }

    sealed class StatementMapperSpec
    {
        [YamlMember(Alias = "entity")] public string? Entity { get; set; }
        [YamlMember(Alias = "querySuffix")] public string? QuerySuffix { get; set; }
        [YamlMember(Alias = "writeSuffix")] public string? WriteSuffix { get; set; }
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
