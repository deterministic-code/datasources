namespace Datasources;

public sealed class BatchCommandIBulkWriteStrategy<T> : IBulkWriteStrategy<T> where T : new()
{
    const int ParameterBudget = 8000;
    const int MaxRows = 250;

    public CRUDRepository<T> Repo { get; set; } = null!;
    public ICommand Command { get; set; } = null!;
    public CRUDRepositoryOptions<T> Options { get; set; } = null!;
    CRUDRepositoryStatements? cached;

    public void Attach(ICommand command, CRUDRepositoryOptions<T> options) =>
        (Command, Options) = (command, options);

    public Task<Response> Create(IReadOnlyList<T> items) =>
        Then(WriteCreates(items), items, WriteMode.Create);

    public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere) =>
        Then(
            byWhere is null ? WriteUpdates(items) : SendEach(items.Select(item => UpdateCommand(item, byWhere))),
            items,
            WriteMode.Update);

    public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere) =>
        Then(
            byWhere is null ? WriteDeletes(items) : SendEach(items.Select(item => DeleteCommand(item, byWhere))),
            items,
            WriteMode.Delete);

    async Task<Response> Then(Task<Response> write, IReadOnlyList<T> items, WriteMode mode)
    {
        var response = await write;
        await BulkWriteEnrichment.After(Repo, Options, items, mode);
        return response;
    }

    async Task<Response> WriteCreates(IReadOnlyList<T> items)
    {
        if (items.Count == 0)
            return Empty();
        var rows = items.Select(MappedInsert).ToList();
        var columns = Columns(rows.Select(row => row.Fields));
        return await Chunked(rows, columns.Count, chunk =>
            Execute(InsertSql(rows[0].Table, columns, chunk)));
    }

    async Task<Response> WriteUpdates(IReadOnlyList<T> items)
    {
        if (items.Count == 0)
            return Empty();
        var rows = items.Select(MappedUpdate).ToList();
        var set = Columns(rows.Select(row => row.Fields));
        var ids = Columns(rows.Select(row => row.Ids));
        return await Chunked(rows, set.Count + ids.Count, chunk =>
            Execute(UpdateSql(rows[0].Table, set, ids, chunk)));
    }

    async Task<Response> WriteDeletes(IReadOnlyList<T> items)
    {
        if (items.Count == 0)
            return Empty();
        var rows = items.Select(MappedDelete).ToList();
        var ids = Columns(rows.Select(row => row.Ids));
        return await Chunked(rows, ids.Count, chunk =>
            Execute(DeleteSql(rows[0].Table, ids, chunk)));
    }

    async Task<Response> Chunked<TRow>(IReadOnlyList<TRow> rows, int width, Func<IReadOnlyList<TRow>, Task<Response>> write)
    {
        var size = Math.Min(MaxRows, Math.Max(1, ParameterBudget / Math.Max(1, width)));
        var affected = 0;
        for (var at = 0; at < rows.Count; at += size)
            affected += (await write(rows.Skip(at).Take(size).ToList())).RowsAffected;
        return new Response(Value.Null.Instance, affected);
    }

    async Task<Response> SendEach(IEnumerable<BulkCommand> commands)
    {
        var affected = 0;
        foreach (var command in commands)
            affected += (await Command.Execute(new Native(command.Text), command.Parameters)).RowsAffected;
        return new Response(Value.Null.Instance, affected);
    }

    Task<Response> Execute((string Sql, IReadOnlyList<Parameter> Parameters) command) =>
        Command.Execute(new Native(command.Sql), command.Parameters);

    (string Table, IReadOnlyList<Parameter> Fields) MappedInsert(T item)
    {
        var record = Reverse(item);
        var statement = Statements().Insert(Fields(record, col => !Ids.Contains(col) || Assigned(record, col)));
        return (statement.Table, statement.Fields);
    }

    (string Table, IReadOnlyList<Parameter> Fields, IReadOnlyList<Parameter> Ids) MappedUpdate(T item)
    {
        var record = Reverse(item);
        var fields = Fields(record, col => !Ids.Contains(col));
        if (fields.Count == 0)
            throw new DatasourceException("Update requires at least one non-Id field");
        var statement = Statements().Update(fields, IdWhere(record), (name, value) => Cell(name, value, true));
        return (statement.Table, statement.Fields, statement.Where?.Parameters ?? []);
    }

    (string Table, IReadOnlyList<Parameter> Ids) MappedDelete(T item)
    {
        var record = Reverse(item);
        var statement = Statements().Delete(IdWhere(record), (name, value) => Cell(name, value, true));
        return (statement.Table, statement.Where?.Parameters ?? []);
    }

    (string Sql, IReadOnlyList<Parameter> Parameters) InsertSql(
        string table,
        IReadOnlyList<string> columns,
        IReadOnlyList<(string Table, IReadOnlyList<Parameter> Fields)> rows)
    {
        var parameters = new List<Parameter>();
        var values = rows.Select((row, index) =>
        {
            var byName = Lookup(row.Fields);
            return "(" + string.Join(", ", columns.Select(column =>
            {
                parameters.Add(new Parameter($"{column}__{index}", ValueOf(byName, column)));
                return $"${column}__{index}";
            })) + ")";
        });
        return ($"INSERT INTO {table} ({string.Join(", ", columns)}) VALUES {string.Join(", ", values)}", parameters);
    }

    (string Sql, IReadOnlyList<Parameter> Parameters) UpdateSql(
        string table,
        IReadOnlyList<string> set,
        IReadOnlyList<string> ids,
        IReadOnlyList<(string Table, IReadOnlyList<Parameter> Fields, IReadOnlyList<Parameter> Ids)> rows)
    {
        var parameters = new List<Parameter>();
        var columns = set.Concat(ids).ToList();
        var source = "(" + string.Join(" UNION ALL ", rows.Select((row, index) =>
        {
            var byName = Lookup(row.Fields.Concat(row.Ids));
            return "SELECT " + string.Join(", ", columns.Select(column =>
            {
                parameters.Add(new Parameter($"{column}__{index}", ValueOf(byName, column)));
                return $"${column}__{index} AS {column}";
            }));
        })) + ") AS data";
        var assigns = string.Join(", ", set.Select(column => Mysql
            ? $"{table}.{column} = data.{column}"
            : $"{column} = data.{column}"));
        var on = string.Join(" AND ", ids.Select(column => $"{table}.{column} = data.{column}"));
        return (Mysql
            ? $"UPDATE {table} JOIN {source} ON {on} SET {assigns}"
            : $"UPDATE {table} SET {assigns} FROM {source} WHERE {on}", parameters);
    }

    (string Sql, IReadOnlyList<Parameter> Parameters) DeleteSql(
        string table,
        IReadOnlyList<string> ids,
        IReadOnlyList<(string Table, IReadOnlyList<Parameter> Ids)> rows)
    {
        if (ids.Count == 0)
            return ($"DELETE FROM {table} WHERE 1 = 1", []);
        var parameters = new List<Parameter>();
        if (ids.Count == 1)
        {
            var column = ids[0];
            var names = rows.Select((row, index) =>
            {
                parameters.Add(new Parameter($"{column}__{index}", ValueOf(Lookup(row.Ids), column)));
                return $"${column}__{index}";
            });
            return ($"DELETE FROM {table} WHERE {column} IN ({string.Join(", ", names)})", parameters);
        }
        var tuples = rows.Select((row, index) =>
        {
            var byName = Lookup(row.Ids);
            return "(" + string.Join(", ", ids.Select(column =>
            {
                parameters.Add(new Parameter($"{column}__{index}", ValueOf(byName, column)));
                return $"${column}__{index}";
            })) + ")";
        });
        return ($"DELETE FROM {table} WHERE ({string.Join(", ", ids)}) IN ({string.Join(", ", tuples)})", parameters);
    }

    BulkCommand UpdateCommand(T item, IWhere? byWhere)
    {
        var record = Reverse(item);
        var fields = Fields(record, col => !Ids.Contains(col));
        if (fields.Count == 0)
            throw new DatasourceException("Update requires at least one non-Id field");
        var statement = Statements().Update(fields, byWhere ?? IdWhere(record), (name, value) => Cell(name, value, byWhere is null));
        return new(DefaultStatementTextRenderer.Instance.Render(statement), statement.Parameters);
    }

    BulkCommand DeleteCommand(T item, IWhere? byWhere)
    {
        var record = Reverse(item);
        var statement = Statements().Delete(byWhere ?? IdWhere(record), (name, value) => Cell(name, value, byWhere is null));
        var parameters = statement.Where?.Parameters is { Count: > 0 } listed
            ? listed
            : DefaultStatementTextRenderer.Instance.Render(statement).Contains('$') ? Fields(record, _ => true) : [];
        return new(DefaultStatementTextRenderer.Instance.Render(statement), parameters);
    }

    CRUDRepositoryStatements Statements()
    {
        if (cached is not null)
            return cached;
        var builder = new CRUDRepositoryBuilder<T>();
        builder.Parts(Command.Connection, Options);
        return cached = builder.CRUDStatements ?? throw new DatasourceException("Statements required");
    }

    IReadOnlyList<string> Ids => Options.Ids;
    bool Mysql => Command.Connection.Provider == "mysql";

    Row Reverse(T item)
    {
        var objects = Options.ObjectConverter ?? new MetaDataRowTypeConverter<T> { ShouldCoerceId = true };
        var row = objects.ReverseConvert(item);
        return Options.RowConverter?.ReverseConvert(row) ?? row;
    }

    IReadOnlyList<Parameter> Fields(Row record, Func<string, bool> include) =>
        [.. record.Where(pair => include(pair.Key) && pair.Value is not Value.Null)
            .OrderBy(pair => pair.Key)
            .Select(pair => new Parameter(pair.Key, Cell(pair.Key, pair.Value, fieldsApplied: true)))];

    Value Cell(string property, Value value, bool fieldsApplied) =>
        fieldsApplied ? value
        : Options.ObjectConverter is MetaDataRowTypeConverter<T> typed
            && typed.Fields.TryGetValue(property, out var field)
            ? field.ReverseConvert(value)
            : value;

    static bool Assigned(Row record, string col) =>
        record.TryGetValue(col, out var value) && value switch
        {
            Value.Null => false,
            Value.Text(var text) => text.Length > 0,
            Value.Int(0) => false,
            _ => true
        };

    IWhere IdWhere(Row record) =>
        new Where(Ids.Select(col =>
            record.TryGetValue(col, out var value)
                ? (IExpression)new Operator.Equals(col, value)
                : throw new DatasourceException($"Missing Id value \"{col}\"")
        ).DefaultIfEmpty(new Operator.Equals(new Literal(1), new Literal(1)))
         .Aggregate((left, right) => new Operator.And(left, right)));

    static List<string> Columns(IEnumerable<IReadOnlyList<Parameter>> rows)
    {
        var seen = new HashSet<string>();
        var columns = new List<string>();
        foreach (var field in rows.SelectMany(row => row))
            if (seen.Add(field.Name))
                columns.Add(field.Name);
        return columns;
    }

    static Dictionary<string, Parameter> Lookup(IEnumerable<Parameter> fields) =>
        fields.GroupBy(field => field.Name).ToDictionary(group => group.Key, group => group.Last());

    static Value ValueOf(IReadOnlyDictionary<string, Parameter> fields, string column) =>
        fields.TryGetValue(column, out var field) ? field.Value : Value.Null.Instance;

    static Response Empty() => new(Value.Null.Instance, 0);
}
