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
        var mapping = TypeConverter.Mapping<SpecUser>("sqlite", name: spec.Type.Table);
        var built = RepositoryBuilder<SpecUser>.Parts(new()
        {
            Connection = session.Connection,
            Mapping = mapping,
            Ids = spec.Type.Ids,
            AssignIdentityOnAdd = spec.Type.AssignIdentityOnAdd,
        });
        var logs = new List<string>();
        var middleware = new ConsoleCommandMiddleware(logs.Add);
        foreach (var command in new[] { built.Commands.Get, built.Commands.Add, built.Commands.Update, built.Commands.Delete })
            command.Use(middleware);
        var repo = new Repository<SpecUser>(built);
        foreach (var step in spec.Steps)
            await (step switch
            {
                { Create: { } create } => repo.Add(mapping.To(Row(create.Row))),
                { Read: { } read } => repo.GetSingle(WhereBuilder.Equals("id", Value.From(read.Id))),
                { Query: { } query } => Drain(query.Where is { } where ? repo.Get(Where(where)) : repo.Get()),
                { Ready: { } ready } => repo.GetSingle(Where(ready.Where)),
                { Update: { } update } => repo.Update(mapping.To(Row(update.Row))),
                { UpdateBy: { } updateBy } => repo.Update(mapping.To(Row(updateBy.Row)), Where(updateBy.Where)),
                { Delete: { } delete } => repo.Delete(mapping.To(new Dictionary<string, Value> { ["id"] = Value.From(delete.Id) })),
                { DeleteBy: { } deleteBy } => repo.Delete(new SpecUser(), Where(deleteBy.Where)),
                _ => throw new InvalidOperationException($"{spec.Name}: empty step")
            });
        SqlSpec.AssertConsoleLogs(spec.Name, Queries(spec), logs);
    }

    static async Task Drain<T>(IAsyncEnumerable<T> items) { await foreach (var _ in items) { } }

    static IEnumerable<string> Queries(Spec spec)
    {
        var columns = spec.Type.Fields.Keys.OrderBy(key => key).ToArray();
        var ids = spec.Type.Ids.ToHashSet();
        foreach (var step in spec.Steps)
            yield return step switch
            {
                { Create: { } create } => StatementBuilder.CreateInsert(spec.Type.Table, Fields(create.Row, col => !spec.Type.AssignIdentityOnAdd || !ids.Contains(col))).GetText(),
                { Read: { } read } => StatementBuilder.CreateQuery(spec.Type.Table, columns, WhereBuilder.Equals("id", Value.From(read.Id))).GetText(),
                { Query: { } query } => StatementBuilder.CreateQuery(spec.Type.Table, columns, query.Where is { } where ? Where(where) : null).GetText(),
                { Ready: { } ready } => StatementBuilder.CreateQuery(spec.Type.Table, columns, Where(ready.Where)).GetText(),
                { Update: { } update } => StatementBuilder.CreateUpdate(spec.Type.Table, Fields(update.Row, col => !ids.Contains(col)), WhereBuilder.Equals("id", Value.From(update.Row["id"]))).GetText(),
                { UpdateBy: { } updateBy } => StatementBuilder.CreateUpdate(spec.Type.Table, Fields(updateBy.Row, col => !ids.Contains(col)), Where(updateBy.Where)).GetText(),
                { Delete: { } delete } => StatementBuilder.CreateDelete(spec.Type.Table, WhereBuilder.Equals("id", Value.From(delete.Id))).GetText(),
                { DeleteBy: { } deleteBy } => StatementBuilder.CreateDelete(spec.Type.Table, Where(deleteBy.Where)).GetText(),
                _ => throw new InvalidOperationException($"{spec.Name}: empty step")
            };
    }

    static IReadOnlyList<Parameter> Fields(Dictionary<string, object?> row, Func<string, bool> include) =>
        row.Where(pair => include(pair.Key)).OrderBy(pair => pair.Key)
            .Select(pair => new Parameter(pair.Key, Value.From(Coerce(pair.Value)))).ToArray();

    static IWhere Where(Dictionary<string, object?> row) =>
        WhereBuilder.And([.. row.Select(pair => WhereBuilder.Equals(pair.Key, Value.From(Coerce(pair.Value))))]);

    static Dictionary<string, Value> Row(Dictionary<string, object?> row) =>
        row.ToDictionary(pair => pair.Key, pair => Value.From(Coerce(pair.Value)));

    static object? Coerce(object? value) => value switch
    {
        "true" => true,
        "false" => false,
        _ => value
    };

    static SqliteColumn[] Columns(TypeSpec type) =>
        [.. type.Fields.Select(pair => new SqliteColumn(pair.Key, pair.Value, type.Ids.Contains(pair.Key)))];

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
        [YamlMember(Alias = "READ")] public IdOp? Read { get; set; }
        [YamlMember(Alias = "QUERY")] public QueryOp? Query { get; set; }
        [YamlMember(Alias = "READY")] public WhereOp? Ready { get; set; }
        [YamlMember(Alias = "UPDATE")] public RowOp? Update { get; set; }
        [YamlMember(Alias = "UPDATEBY")] public UpdateByOp? UpdateBy { get; set; }
        [YamlMember(Alias = "DELETE")] public IdOp? Delete { get; set; }
        [YamlMember(Alias = "DELETEBY")] public WhereOp? DeleteBy { get; set; }
    }

    sealed class RowOp
    {
        [YamlMember(Alias = "row")] public Dictionary<string, object?> Row { get; set; } = [];
    }

    sealed class IdOp
    {
        [YamlMember(Alias = "id")] public object? Id { get; set; }
    }

    sealed class QueryOp
    {
        [YamlMember(Alias = "where")] public Dictionary<string, object?>? Where { get; set; }
    }

    sealed class WhereOp
    {
        [YamlMember(Alias = "where")] public Dictionary<string, object?> Where { get; set; } = [];
    }

    sealed class UpdateByOp
    {
        [YamlMember(Alias = "row")] public Dictionary<string, object?> Row { get; set; } = [];
        [YamlMember(Alias = "where")] public Dictionary<string, object?> Where { get; set; } = [];
    }
}
