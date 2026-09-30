namespace Datasources;

public sealed class StatementMappings
{
    readonly IMapper<string, string> entityMappers;
    readonly IMapper<string, string> propertyMappers;

    public StatementMappings(IMapper<string, string> entityMappers, IMapper<string, string> propertyMappers) =>
        (this.entityMappers, this.propertyMappers) = (entityMappers, propertyMappers);

    public string Column(string property)
    {
        var dot = LastDot(property);
        if (dot < 0) return propertyMappers.Map(property);
        var qualifier = property[..dot];
        var name = property[(dot + 1)..];
        var mapped = propertyMappers.Map(name);
        if (mapped != name && mapped.Contains('.')) return mapped;
        var table = entityMappers.Map(qualifier);
        return table == qualifier && mapped == name ? property : $"{table}.{mapped}";
    }

    public string Property(string key)
    {
        var reversed = propertyMappers.ReverseMap(key);
        if (reversed != key && propertyMappers.Map(reversed) == key) return reversed;
        var dot = LastDot(key);
        if (dot < 0) return reversed;
        var column = key[(dot + 1)..];
        var name = propertyMappers.ReverseMap(column);
        return name != column && propertyMappers.Map(name) == column ? name : reversed;
    }

    public IStatement? Apply(IStatement? query, Func<string, Value, Value> cell) => query switch
    {
        SelectStatement { Table: { } table } select => new SelectStatement(new()
        {
            Distinct = select.Distinct,
            Table = entityMappers.Map(table),
            Fields = [.. select.Fields.Select(Column)],
            Where = Apply(select.Where, cell),
            Joins = [.. select.Joins.Select(join => Apply(join, cell))],
            GroupBy = [.. select.GroupBy.Select(Column)],
            Having = Apply(select.Having, cell),
            OrderBy = [.. select.OrderBy.Select(order => order with { Column = Column(order.Column) })],
            Limit = select.Limit,
            Offset = select.Offset,
        }),
        _ => query
    };

    public IWhere? Apply(IWhere? where, Func<string, Value, Value> cell) =>
        where is Where clause ? clause.Map(Column, cell) : where;

    Join Apply(Join join, Func<string, Value, Value> cell) =>
        join.On is { } on
            ? join with { Table = entityMappers.Map(join.Table), On = new Where(on).Map(Column, cell).Condition }
            : join with { Table = entityMappers.Map(join.Table) };

    static int LastDot(string name)
    {
        var quoted = false;
        var dot = -1;
        for (var i = 0; i < name.Length; i++)
        {
            if (name[i] == '"') quoted = !quoted;
            if (name[i] == '.' && !quoted) dot = i;
        }
        return dot;
    }
}
