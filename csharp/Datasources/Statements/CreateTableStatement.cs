namespace Datasources;

public sealed record CreateTableStatement : ICreateTableStatement
{
    readonly string text;
    public CreateTableStatement(string text) => this.text = text;
    public CreateTableStatement(
        string table,
        IReadOnlyList<Column> columns,
        IReadOnlyList<ITableConstraint>? constraints = null) =>
        text = $"CREATE TABLE {table} ({string.Join(", ", Parts(columns, constraints))})";
    public string GetText() => text;

    static IEnumerable<string> Parts(IReadOnlyList<Column> columns, IReadOnlyList<ITableConstraint>? constraints) =>
        constraints is null or []
            ? columns.Select(ColumnSql)
            : columns.Select(ColumnSql).Concat(constraints.Select(ConstraintSql));

    static string ColumnSql(Column column) =>
        $"{column.Name} {column.Type}{DefaultSql(column.Default)}{NotNullSql(column.NotNull)}";

    static string DefaultSql(ColumnDefault? value) =>
        value is null ? "" : value.Name is null ? $" DEFAULT {value.Value}" : $" CONSTRAINT {value.Name} DEFAULT {value.Value}";

    static string NotNullSql(NotNull? value) =>
        value is null ? "" : value.Name is null ? " NOT NULL" : $" CONSTRAINT {value.Name} NOT NULL";

    static string ConstraintSql(ITableConstraint constraint) => constraint switch
    {
        PrimaryKey pk => $"CONSTRAINT {pk.Name} PRIMARY KEY ({string.Join(", ", pk.Columns)})",
        ForeignKey fk => ForeignKeySql(fk),
        UniqueConstraint unique => $"CONSTRAINT {unique.Name} UNIQUE ({string.Join(", ", unique.Columns)})",
        CheckConstraint check => $"CONSTRAINT {check.Name} CHECK ({check.Expression})",
        _ => throw new InvalidOperationException("empty constraint")
    };

    static string ForeignKeySql(ForeignKey fk) =>
        $"CONSTRAINT {fk.Name} FOREIGN KEY ({string.Join(", ", fk.Columns)}) REFERENCES {fk.ReferencesTable} ({string.Join(", ", fk.ReferencesColumns)}){ActionSql("DELETE", fk.OnDelete)}{ActionSql("UPDATE", fk.OnUpdate)}";

    static string ActionSql(string kind, ReferentialAction? action) =>
        action is null ? "" : $" ON {kind} {ActionKeyword(action.Value)}";

    static string ActionKeyword(ReferentialAction action) => action switch
    {
        ReferentialAction.Cascade => "CASCADE",
        ReferentialAction.SetNull => "SET NULL",
        ReferentialAction.SetDefault => "SET DEFAULT",
        ReferentialAction.Restrict => "RESTRICT",
        ReferentialAction.NoAction => "NO ACTION",
        _ => throw new InvalidOperationException("empty action")
    };
}
