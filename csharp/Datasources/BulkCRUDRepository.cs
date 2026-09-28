using System.Collections;

namespace Datasources;

public class BulkCRUDRepository<T> : CRUDRepository<T>, IBulkWrite<T> where T : new()
{
    readonly IBulkWriteStrategy<T> bulk;
    readonly bool useTransaction;

    public BulkCRUDRepository(IConnection connection) : base(connection) =>
        (bulk, useTransaction) = (new SingleCommands<T>(), false);

    public BulkCRUDRepository(IConnection connection, CRUDRepositoryOptions<T> options) :
        this(connection, options, new SingleCommands<T>()) { }

    public BulkCRUDRepository(IConnection connection, CRUDRepositoryOptions<T> options, IBulkWriteStrategy<T> bulk)
        : base(connection, options) =>
        (this.bulk, useTransaction) = (bulk, options.Write?.UseTransaction ?? false);

    public BulkCRUDRepository(BuiltRepository<T> built) : this(built, new SingleCommands<T>()) { }

    public BulkCRUDRepository(BuiltRepository<T> built, IBulkWriteStrategy<T> bulk) : base(built) =>
        (this.bulk, useTransaction) = (bulk, false);

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

    ICommand IBulkWrite<T>.Command => Command();

    async Task<Value> IBulkWrite<T>.CreateOne(T item) => IdOf(await Create(item));

    Task<Response> IBulkWrite<T>.UpdateOne(T item, IWhere? byWhere) => Update(item, byWhere);

    Task<Response> IBulkWrite<T>.DeleteOne(T item, IWhere? byWhere) => Delete(item, byWhere);

    IReadOnlyList<BulkCommand> IBulkWrite<T>.Inserts(IReadOnlyList<T> items) =>
        [.. items.Select(item => Insert(Stored(item)))];

    IReadOnlyList<BulkCommand> IBulkWrite<T>.Updates(IReadOnlyList<T> items, IWhere? byWhere) =>
        [.. items.Select(item => UpdateCommand(Stored(item), byWhere))];

    IReadOnlyList<BulkCommand> IBulkWrite<T>.Deletes(IReadOnlyList<T> items, IWhere? byWhere) =>
        [.. items.Select(item => DeleteCommand(Stored(item), byWhere))];

    IReadOnlyList<Row> IBulkWrite<T>.Rows(IReadOnlyList<T> items) =>
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

    T Stored(T item) =>
        Enriched ? (T)DefaultWriteEnrichmentStrategy<T>.Strip((IEnrichmentHost)this, item!) : item;

    BulkCommand Insert(T stored)
    {
        var fields = Fields(Reverse(stored), col => !Ids.Contains(col));
        return new(Statements.Insert(fields).GetText(), fields);
    }

    BulkCommand UpdateCommand(T item, IWhere? byWhere)
    {
        var record = Reverse(item);
        var where = byWhere is null ? Map(IdWhere(record), fieldsApplied: true) : Map(byWhere, fieldsApplied: false);
        var fields = Fields(record, col => !Ids.Contains(col));
        if (fields.Count == 0)
            throw new DatasourceException("Update requires at least one non-Id field");
        return new(Statements.Update(fields, where).GetText(), UpdateStatement.ParameterList(fields, where));
    }

    BulkCommand DeleteCommand(T item, IWhere? byWhere)
    {
        var record = Reverse(item);
        var where = byWhere is null ? Map(IdWhere(record), fieldsApplied: true) : Map(byWhere, fieldsApplied: false);
        var statement = Statements.Delete(where);
        var parameters = where.Parameters is { Count: > 0 } listed
            ? listed
            : statement.GetText().Contains('$') ? Fields(record, _ => true) : [];
        return new(statement.GetText(), parameters);
    }

    Row RowOf(T item)
    {
        var stored = Reverse(Stored(item));
        var row = new Row();
        foreach (var name in Statements.ReadColumns)
        {
            var match = stored.FirstOrDefault(pair => Column(pair.Key) == name);
            row[name] = match.Key is null ? Value.Null.Instance : match.Value;
        }
        return row;
    }

    Row Reverse(T item) => RowConverter.ReverseConvert(Mapper!.ReverseMap(item));

    IReadOnlyList<Parameter> Fields(Row record, Func<string, bool> include) =>
        [.. record.Where(pair => include(pair.Key) && pair.Value is not Value.Null).OrderBy(pair => pair.Key)
            .Select(pair => new Parameter(Column(pair.Key), Cell(pair.Key, pair.Value, fieldsApplied: true)))];

    IWhere Map(IWhere where, bool fieldsApplied) =>
        where is Where clause
            ? clause.Map(Column, (name, cell) => Cell(name, cell, fieldsApplied))
            : where;

    IWhere IdWhere(Row record) =>
        new Where(Ids.Select(col =>
            record.TryGetValue(col, out var value)
                ? (IExpression)new Operator.Equals(col, value)
                : throw new DatasourceException($"Missing Id value \"{col}\"")
        ).DefaultIfEmpty(new Operator.Equals(new Literal(1), new Literal(1)))
         .Aggregate((left, right) => new Operator.And(left, right)));
}
