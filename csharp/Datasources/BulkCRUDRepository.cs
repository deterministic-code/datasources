using System.Collections;

namespace Datasources;

public class BulkCRUDRepository<T> : CRUDRepository<T> where T : new()
{
    readonly IBulkWriteStrategy<T> bulk;
    readonly bool useTransaction;
    readonly CRUDRepositoryStatements crudStatements;

    public BulkCRUDRepository(IConnection connection) : base(connection) =>
        (bulk, useTransaction, crudStatements) = (
            new SingleCommandIBulkWriteStrategy<T>(),
            false,
            WriteStatements(connection, CreateCRUDRepositoryOptions.FromType<T>()));

    public BulkCRUDRepository(IConnection connection, CRUDRepositoryOptions<T> options) :
        this(connection, options, new SingleCommandIBulkWriteStrategy<T>()) { }

    public BulkCRUDRepository(IConnection connection, CRUDRepositoryOptions<T> options, IBulkWriteStrategy<T> bulk)
        : base(connection, options) =>
        (this.bulk, useTransaction, crudStatements) = (
            bulk,
            options.Write?.UseTransaction ?? false,
            WriteStatements(connection, options));

    public Task<Response> Create(IReadOnlyList<T> items) =>
        InTransaction(() => bulk.Create(items, this));

    public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere = null) =>
        InTransaction(() => bulk.Update(items, byWhere, this));

    public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere = null) =>
        InTransaction(() => bulk.Delete(items, byWhere, this));

    async Task<TResult> InTransaction<TResult>(Func<Task<TResult>> write)
    {
        if (!useTransaction) return await write();
        var work = new UnitOfWorkTransaction(Command().Connection);
        await work.Begin();
        try
        {
            var result = await write();
            await work.Commit();
            return result;
        }
        catch
        {
            await work.Abort();
            throw;
        }
    }

    public Task<Response> BulkCreate(IReadOnlyList<T> items) => Create(items);

    public Task<Response> BulkUpdate(IReadOnlyList<T> items, IWhere? byWhere = null) =>
        Update(items, byWhere);

    public Task<Response> BulkDelete(IReadOnlyList<T> items, IWhere? byWhere = null) =>
        Delete(items, byWhere);

    public new ICommand Command() => base.Command();

    public async Task<Value> CreateOne(T item) => IdOf(await Create(item));

    public Task<Response> UpdateOne(T item, IWhere? byWhere) => Update(item, byWhere);

    public Task<Response> DeleteOne(T item, IWhere? byWhere) => Delete(item, byWhere);

    public IReadOnlyList<BulkCommand> Inserts(IReadOnlyList<T> items) =>
        [.. items.Select(Insert)];

    public IReadOnlyList<BulkCommand> Updates(IReadOnlyList<T> items, IWhere? byWhere) =>
        [.. items.Select(item => UpdateCommand(item, byWhere))];

    public IReadOnlyList<BulkCommand> Deletes(IReadOnlyList<T> items, IWhere? byWhere) =>
        [.. items.Select(item => DeleteCommand(item, byWhere))];

    public IReadOnlyList<Row> Rows(IReadOnlyList<T> items) =>
        [.. items.Select(RowOf)];

    Value IdOf(T item)
    {
        if (Ids.Count != 1) return Value.Null.Instance;
        var name = Ids[0];
        if (item is IDictionary dict)
            return dict.Contains(name) ? Value.From(dict[name]) : Value.Null.Instance;
        return typeof(T).GetProperty(name)?.GetValue(item) is { } value
            ? Value.From(value)
            : Value.Null.Instance;
    }

    static CRUDRepositoryStatements WriteStatements(IConnection connection, CRUDRepositoryOptions<T> options)
    {
        var builder = new CRUDRepositoryBuilder<T>();
        builder.Parts(connection, options);
        return builder.CRUDStatements ?? throw new DatasourceException("Statements required");
    }

    BulkCommand Insert(T item)
    {
        var record = Reverse(item);
        var statement = crudStatements.Insert(Fields(record, col => !Ids.Contains(col) || Assigned(record, col)));
        return new(DefaultStatementTextRenderer.Instance.Render(statement), statement.Fields);
    }

    BulkCommand UpdateCommand(T item, IWhere? byWhere)
    {
        var record = Reverse(item);
        var fields = Fields(record, col => !Ids.Contains(col));
        if (fields.Count == 0)
            throw new DatasourceException("Update requires at least one non-Id field");
        var statement = crudStatements.Update(fields, byWhere ?? IdWhere(record), (name, value) => Cell(name, value, byWhere is null));
        return new(DefaultStatementTextRenderer.Instance.Render(statement), statement.Parameters);
    }

    BulkCommand DeleteCommand(T item, IWhere? byWhere)
    {
        var record = Reverse(item);
        var statement = crudStatements.Delete(byWhere ?? IdWhere(record), (name, value) => Cell(name, value, byWhere is null));
        var parameters = statement.Where?.Parameters is { Count: > 0 } listed
            ? listed
            : DefaultStatementTextRenderer.Instance.Render(statement).Contains('$') ? Fields(record, _ => true) : [];
        return new(DefaultStatementTextRenderer.Instance.Render(statement), parameters);
    }

    Row RowOf(T item)
    {
        var stored = Reverse(item);
        var row = new Row();
        foreach (var name in crudStatements.ReadColumns)
        {
            var match = stored.FirstOrDefault(pair => Column(pair.Key) == name && !EnrichmentInto(pair.Key));
            row[name] = match.Key is null ? Value.Null.Instance : match.Value;
        }
        return row;
    }

    Row Reverse(T item)
    {
        var row = ObjectConverter.ReverseConvert(item);
        return RowConverter?.ReverseConvert(row) ?? row;
    }

    IReadOnlyList<Parameter> Fields(Row record, Func<string, bool> include) =>
        [.. record.Where(pair => include(pair.Key) && !EnrichmentInto(pair.Key) && pair.Value is not Value.Null)
            .OrderBy(pair => pair.Key)
            .Select(pair => new Parameter(pair.Key, Cell(pair.Key, pair.Value, fieldsApplied: true)))];

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
}
