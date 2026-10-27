namespace Datasources;

public class CreateTableStatementTextRenderer
{
    public virtual string Render(CreateTableStatement statement) =>
        $"CREATE TABLE {Quote(statement.Table)} ({string.Join(", ", Parts(statement))}){Suffix()}";

    protected virtual string Quote(string name) => name;

    protected virtual string Suffix() => "";

    protected virtual string Type(Column column) => column.Type;

    protected virtual string Default(Column column) =>
        column.Default is null ? ""
        : column.Default.Name is null ? $" DEFAULT {column.Default.Value}"
        : $" CONSTRAINT {Quote(column.Default.Name)} DEFAULT {column.Default.Value}";

    protected virtual string Identity(Column column) => "";

    protected virtual string Column(Column column) =>
        $"{Quote(column.Name)} {Type(column)}{Identity(column)}{Default(column)}{NotNullSql(column.NotNull)}";

    protected virtual string Constraint(ITableConstraint constraint) => constraint switch
    {
        PrimaryKey pk => $"CONSTRAINT {Quote(pk.Name)} PRIMARY KEY ({Quoted(pk.Columns)})",
        ForeignKey fk => ForeignKeySql(fk),
        UniqueConstraint unique => $"CONSTRAINT {Quote(unique.Name)} UNIQUE ({Quoted(unique.Columns)})",
        CheckConstraint check => $"CONSTRAINT {Quote(check.Name)} CHECK ({check.Expression})",
        _ => throw new InvalidOperationException("empty constraint")
    };

    IEnumerable<string> Parts(CreateTableStatement statement) =>
        statement.Constraints.Count == 0
            ? statement.Columns.Select(Column)
            : statement.Columns.Select(Column).Concat(statement.Constraints.Select(Constraint));

    protected virtual string NotNullSql(NotNull? value) =>
        value is null ? "" : value.Name is null ? " NOT NULL" : $" CONSTRAINT {Quote(value.Name)} NOT NULL";

    string ForeignKeySql(ForeignKey fk) =>
        $"CONSTRAINT {Quote(fk.Name)} FOREIGN KEY ({Quoted(fk.Columns)}) REFERENCES {Quote(fk.ReferencesTable)} ({Quoted(fk.ReferencesColumns)}){ActionSql("DELETE", fk.OnDelete)}{ActionSql("UPDATE", fk.OnUpdate)}";

    string Quoted(IReadOnlyList<string> names) => string.Join(", ", names.Select(Quote));

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
