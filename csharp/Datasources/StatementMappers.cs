namespace Datasources;

public sealed class StatementMappers : IMapper<string, string>
{
    readonly IMapper<string, string> entityMappers;
    readonly IMapper<string, string> propertyMappers;

    public StatementMappers(IMapper<string, string> entityMappers, IMapper<string, string> propertyMappers) =>
        (this.entityMappers, this.propertyMappers) = (entityMappers, propertyMappers);

    public string Map(string property)
    {
        var parts = SelectFieldAliases.Segments(property);
        if (parts.Count < 2) return propertyMappers.Map(property);
        var qualifier = parts[^2];
        var name = parts[^1];
        var mapped = propertyMappers.Map(name);
        if (mapped != name && mapped.Contains('.')) return mapped;
        var table = entityMappers.Map(qualifier);
        return table == qualifier && mapped == name ? $"{qualifier}.{name}" : $"{table}.{mapped}";
    }

    public string ReverseMap(string key)
    {
        var reversed = propertyMappers.ReverseMap(key);
        if (reversed != key && propertyMappers.Map(reversed) == key) return reversed;
        var parts = SelectFieldAliases.Segments(key);
        if (parts.Count < 2) return reversed;
        var column = parts[^1];
        var name = propertyMappers.ReverseMap(column);
        return name != column && propertyMappers.Map(name) == column ? name : reversed;
    }

    internal string Column(string property) => Map(property);

    internal string Property(string key) => ReverseMap(key);

    public IStatement? Apply(IStatement? query, Func<string, Value, Value> cell) => query switch
    {
        SelectStatement { Table: { } table } select => new SelectStatement(new()
        {
            Distinct = select.Distinct,
            Table = entityMappers.Map(table),
            Fields = [.. MapSelectFields(table, select.Fields, select.Aliases)],
            Where = Apply(select.Where, cell),
            Joins = [.. select.Joins.Select(join => Apply(join, cell))],
            GroupBy = [.. select.GroupBy.Select(Column)],
            Having = Apply(select.Having, cell),
            OrderBy = [.. select.OrderBy.Select(order => order with { Column = Column(order.Column) })],
            Limit = select.Limit,
            Offset = select.Offset,
        }),
        InsertStatement { Table: { } table } insert => new InsertStatement(
            entityMappers.Map(table),
            [.. insert.Fields.Select(field => field with { Name = Column(field.Name) })]),
        UpdateStatement { Table: { } table } update => new UpdateStatement(
            entityMappers.Map(table),
            [.. update.Fields.Select(field => field with { Name = Column(field.Name) })],
            Apply(update.Where, cell)),
        DeleteStatement { Table: { } table } delete => new DeleteStatement(
            entityMappers.Map(table),
            Apply(delete.Where, cell)),
        SelectInsertedIdStatement inserted => new SelectInsertedIdStatement(
            entityMappers.Map(inserted.Table),
            Column(inserted.Column)),
        _ => query
    };

    public IWhere? Apply(IWhere? where, Func<string, Value, Value> cell) =>
        where is Where clause ? clause.Map(Column, cell) : where;

    Join Apply(Join join, Func<string, Value, Value> cell) =>
        join.On is { } on
            ? join with { Table = entityMappers.Map(join.Table), On = new Where(on).Map(Column, cell).Condition }
            : join with { Table = entityMappers.Map(join.Table) };

    IReadOnlyList<string> MapSelectFields(string table, IReadOnlyList<string> fields, IReadOnlyList<string?> aliases)
    {
        var lookup = SelectFieldAliases.Of(table, fields, aliases);
        return [.. fields.Select((field, i) => MapSelectField(field, lookup[i]))];
    }

    string MapSelectField(string field, string? alias)
    {
        if (SelectFieldAliases.AsIndex(field) >= 0)
            throw new DatasourceException($"Select field \"{field}\" must be a path");
        var mapped = SelectFieldAliases.FunctionOf(field) is { } function
            ? $"{function.Name}({Map(function.Inner)})"
            : Map(field);
        return SelectFieldAliases.Apply(mapped, alias);
    }
}
