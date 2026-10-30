namespace Datasources;

public sealed record CreateTableStatement : ICreateTableStatement
{
    public string Table { get; }
    public IReadOnlyList<Column> Columns { get; }
    public IReadOnlyList<ITableConstraint> Constraints { get; }

    public CreateTableStatement(
        string table,
        IReadOnlyList<Column> columns,
        IReadOnlyList<ITableConstraint>? constraints = null)
    {
        Table = table;
        Columns = columns;
        Constraints = constraints ?? [];
    }

    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);

    public static string TableName(string entity, StatementNaming naming) =>
        naming.IsVerbatim(entity)
            ? entity
            : naming.Letter(naming.Convert(naming.PhysicalStem(entity), naming.TableNameCasing));

    public static string ColumnName(string field, StatementNaming naming) =>
        naming.IsVerbatim(field) ? field : naming.Convert(field, naming.FieldNameCasing);
}
