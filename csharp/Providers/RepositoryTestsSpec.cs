using System.Collections;
using System.Globalization;
using System.Text.RegularExpressions;
using Datasources;
using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;

namespace Datasources.SqlSpecs;

public static class RepositoryTestsSpec
{
    public static IEnumerable<string> Files() =>
        Directory.GetFiles(Path.Combine(SqlSpec.SpecsDir(), "repositories"), "*.yaml");

    public static async Task Run(
        string path,
        Func<Session> open,
        Func<string, string> quote)
    {
        var spec = new DeserializerBuilder()
            .WithNamingConvention(NullNamingConvention.Instance)
            .Build()
            .Deserialize<Spec>(File.ReadAllText(path));
        var session = open();
        await session.Connection.Open();
        var rename = spec.Type.CRUDMapper?.Entity is { Length: > 0 } entity && entity != spec.Type.Table
            ? new Dictionary<string, string> { [spec.Type.Table] = entity }
            : null;
        foreach (var statement in SchemaSpec.Tables(spec.Schema, quote, rename))
            await session.Command.Execute(statement);
        var types = session.Types;
        var fields = FieldsOf(spec.Type, types);
        if (Verbose()) Console.WriteLine($"\n--- {spec.Name} ({session.Connection.Provider}) ---");
        var logs = new List<string>();
        if (spec.Type.Mappings.Count == 0)
            await Run(spec, session, new MetaDataRowTypeConverter<SpecUser>(fields) { ShouldCoerceId = true }, logs, quote);
        else
            await Run(spec, session, new MetaDataRowTypeConverter<MappedUser>(fields) { ShouldCoerceId = true }, logs, quote);
        SqlSpec.AssertConsoleLogs(
            spec.Name,
            Queries(spec, session.Connection.Provider),
            [.. logs.Where(line => line.Contains("[Start]") || line.Contains("[Finish]")).Select(Unquote)]);
    }

    static async Task Run<T>(Spec spec, Session session, MetaDataRowTypeConverter<T> mapping, List<string> logs, Func<string, string> quote)
        where T : new()
    {
        var repo = new CRUDRepository<T>(session.Connection, OptionsOf(spec.Type, mapping, quote)).AddMiddleware(new ConsoleCommandMiddleware(line =>
        {
            logs.Add(line);
            if (Verbose()) Console.WriteLine(line);
        }, null, new Dictionary<string, string> { ["isVerbose"] = Verbose() ? "true" : "false" }));
        foreach (var step in spec.Steps)
            await (step switch
            {
                { Create: { } create } => repo.Create(ToEntity(create.Row, spec.Type, mapping)),
                { Read: { } read } => AssertRead(spec.Name, read, repo.ReadSingle(new Where(new Operator.Equals("id", IdValue(read.Id, spec.Type))))),
                { Query: { } query } => AssertQuery(spec.Name, query, query.Where is { } where ? repo.Read(Where(mapping, where, spec.Type)) : repo.Read()),
                { ReadBy: { } readBy } => AssertRead(spec.Name, readBy, repo.ReadSingle(Where(mapping, readBy.Where, spec.Type))),
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

    static async Task AssertRead<T>(string name, IdOp read, Task<T?> task)
    {
        var actual = await task;
        if (read.HasExpect) AssertRow($"{name} READ", actual, read.Expect);
    }

    static async Task AssertRead<T>(string name, WhereOp readBy, Task<T?> task)
    {
        var actual = await task;
        if (readBy.HasExpect) AssertRow($"{name} READBY", actual, readBy.Expect);
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
        type.CRUDMapper?.Entity ?? type.Table;

    static string Unquote(string text) =>
        Regex.Replace(Regex.Replace(text, "`([^`]+)`", "$1"), "\"([^\"]+)\"", "$1");

    static CRUDRepositoryOptions<T> OptionsOf<T>(TypeSpec type, MetaDataRowTypeConverter<T> mapping, Func<string, string> quote)
        where T : new() =>
        new()
        {
            Entity = type.Table,
            ObjectConverter = mapping,
            Ids = type.Ids,
            ReadProperties = [.. type.Fields.Keys],
            EntityMapper = new TransformMapper<string, string>(
                name => quote(type.CRUDMapper?.Entity ?? name),
                name => name)
        };

    static IEnumerable<string> Queries(Spec spec, string provider)
    {
        var columns = spec.Type.Fields.Keys.OrderBy(key => key).ToArray();
        var ids = spec.Type.Ids.ToHashSet();
        var table = PhysicalTable(spec.Type);
        foreach (var step in spec.Steps)
        {
            var sql = step switch
            {
                { Create: { } create } => InsertSql(table, Flatten(create.Row, spec.Type), ids),
                { Read: { } read } => DefaultStatementTextRenderer.Instance.Render(new SelectStatement(new() { Table = table, Fields = columns, Where = new Where(new Operator.Equals("id", Value.From(read.Id))) })),
                { Query: { } query } => DefaultStatementTextRenderer.Instance.Render(new SelectStatement(new() { Table = table, Fields = columns, Where = query.Where is { } where ? SqlWhere(where) : null })),
                { ReadBy: { } readBy } => DefaultStatementTextRenderer.Instance.Render(new SelectStatement(new() { Table = table, Fields = columns, Where = SqlWhere(readBy.Where) })),
                { Update: { } update } => DefaultStatementTextRenderer.Instance.Render(new UpdateStatement(table, Fields(Flatten(update.Row, spec.Type), col => !ids.Contains(col)), new Where(new Operator.Equals("id", Value.From(Flatten(update.Row, spec.Type)["id"]))))),
                { UpdateBy: { } updateBy } => DefaultStatementTextRenderer.Instance.Render(new UpdateStatement(table, Fields(Flatten(updateBy.Row, spec.Type), col => !ids.Contains(col)), SqlWhere(updateBy.Where))),
                { Delete: { } delete } => DefaultStatementTextRenderer.Instance.Render(new DeleteStatement(table, new Where(new Operator.Equals("id", Value.From(delete.Id))))),
                { DeleteBy: { } deleteBy } => DefaultStatementTextRenderer.Instance.Render(new DeleteStatement(table, SqlWhere(deleteBy.Where))),
                _ => throw new InvalidOperationException($"{spec.Name}: empty step")
            };
            yield return sql;
            if (step.Create is { } created
                && spec.Type.Ids.Count == 1
                && !Assigned(Flatten(created.Row, spec.Type).GetValueOrDefault(spec.Type.Ids[0]))
                && InsertedIdSql(provider, table, spec.Type.Ids[0]) is { Length: > 0 } inserted)
                yield return inserted;
        }
    }

    static string InsertedIdSql(string provider, string table, string column) => provider switch
    {
        "sqlite" => $"SELECT last_insert_rowid() AS {column}",
        "mysql" => $"SELECT LAST_INSERT_ID() AS {column}",
        "postgres" => $"SELECT setval(pg_get_serial_sequence('{table}', '{column}'), COALESCE((SELECT MAX(\"{column}\") FROM {table}), 1)) AS {column}",
        _ => ""
    };

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

    static string InsertSql(string table, Dictionary<string, object?> flat, HashSet<string> ids) =>
        DefaultStatementTextRenderer.Instance.Render(new InsertStatement(table, Fields(flat, col => !ids.Contains(col) || Assigned(flat.GetValueOrDefault(col)))));

    static bool Assigned(object? value) => value switch
    {
        null or "" or 0 or 0L => false,
        _ => true
    };

    static IReadOnlyList<Parameter> Fields(Dictionary<string, object?> row, Func<string, bool> include) =>
        row.Where(pair => include(pair.Key)).OrderBy(pair => pair.Key)
            .Select(pair => new Parameter(pair.Key, Value.From(Coerce(pair.Value)))).ToArray();

    static IWhere Where<T>(MetaDataRowTypeConverter<T> mapping, Dictionary<string, object?> row, TypeSpec type)
        where T : new() =>
        new Where(row.Select(pair =>
        {
            var value = Value.From(Coerce(pair.Value, Canonical(type, pair.Key)));
            var cell = mapping.Fields.TryGetValue(pair.Key, out var field)
                ? field.ReverseConvert(value)
                : value;
            return (IExpression)new Operator.Equals(pair.Key, cell);
        }).DefaultIfEmpty(new Operator.Equals(new Literal(1), new Literal(1)))
          .Aggregate((left, right) => new Operator.And(left, right)));

    static IWhere SqlWhere(Dictionary<string, object?> row) =>
        new Where(row.Select(pair =>
            (IExpression)new Operator.Equals(pair.Key, Value.From(Coerce(pair.Value)))
        ).DefaultIfEmpty(new Operator.Equals(new Literal(1), new Literal(1)))
         .Aggregate((left, right) => new Operator.And(left, right)));

    static T ToEntity<T>(Dictionary<string, object?> row, TypeSpec type, MetaDataRowTypeConverter<T> mapping)
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

    static Value IdValue(object? id, TypeSpec type) =>
        Value.From(Coerce(id, Canonical(type, "id")));

    static object? Coerce(object? value, string? canonical = null) => (canonical, value) switch
    {
        ("bool", "true") => true,
        ("bool", "false") => false,
        ("int" or "integer", var raw) when long.TryParse(
            Convert.ToString(raw, CultureInfo.InvariantCulture),
            NumberStyles.Integer, CultureInfo.InvariantCulture, out var number) => number,
        ("float", var raw) when double.TryParse(
            Convert.ToString(raw, CultureInfo.InvariantCulture),
            NumberStyles.Float, CultureInfo.InvariantCulture, out var number) => number,
        ("uuid", string text) when Guid.TryParse(text, out var guid) => guid,
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

    static bool Verbose() =>
        Environment.GetEnvironmentVariable("VERBOSE") is "1" or "true";

    public sealed record Session(
        IConnection Connection,
        ICommand Command,
        IReadOnlyDictionary<string, TypeConverter<Value, Value>> Types);

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
        [YamlMember(Alias = "schema")] public string Schema { get; set; } = "";
        [YamlMember(Alias = "type")] public TypeSpec Type { get; set; } = new();
        [YamlMember(Alias = "steps")] public List<Step> Steps { get; set; } = [];
    }

    sealed class TypeSpec
    {
        [YamlMember(Alias = "name")] public string Name { get; set; } = "";
        [YamlMember(Alias = "table")] public string Table { get; set; } = "";
        [YamlMember(Alias = "ids")] public List<string> Ids { get; set; } = [];
        [YamlMember(Alias = "fields")] public Dictionary<string, string> Fields { get; set; } = [];
        [YamlMember(Alias = "converters")] public Dictionary<string, string> Converters { get; set; } = [];
        [YamlMember(Alias = "mappings")] public Dictionary<string, Dictionary<string, string>> Mappings { get; set; } = [];
        [YamlMember(Alias = "crudMapper")] public CRUDMapperSpec? CRUDMapper { get; set; }
    }

    sealed class CRUDMapperSpec
    {
        [YamlMember(Alias = "entity")] public string? Entity { get; set; }
        [YamlMember(Alias = "querySuffix")] public string? QuerySuffix { get; set; }
        [YamlMember(Alias = "writeSuffix")] public string? WriteSuffix { get; set; }
    }

    sealed class Step
    {
        [YamlMember(Alias = "CREATE")] public RowOp? Create { get; set; }
        [YamlMember(Alias = "READ")] public IdOp? Read { get; set; }
        [YamlMember(Alias = "QUERY")] public QueryOp? Query { get; set; }
        [YamlMember(Alias = "READBY")] public WhereOp? ReadBy { get; set; }
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
