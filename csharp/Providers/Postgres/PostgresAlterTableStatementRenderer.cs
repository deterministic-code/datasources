using Datasources;

namespace Datasources.Postgres;

internal sealed class PostgresAlterTableStatementRenderer : AlterTableStatementTextRenderer
{
    protected override string Quote(string name) => PostgresDriver.Quote(name);

    protected override string Type(Column column) => PostgresDriver.NativeType(column);

    protected override string ActionSql(AlterTableStatement.IAction action) => action switch
    {
        AlterTableStatement.AlterColumn alter => PostgresAlter(alter.Column),
        _ => base.ActionSql(action)
    };

    string PostgresAlter(Column column)
    {
        var name = Quote(column.Name);
        var defaultSql = column.Default is { } value
            ? $"ALTER COLUMN {name} SET DEFAULT {value.Value}"
            : $"ALTER COLUMN {name} DROP DEFAULT";
        var nullSql = column.NotNull is not null
            ? $"ALTER COLUMN {name} SET NOT NULL"
            : $"ALTER COLUMN {name} DROP NOT NULL";
        return $"ALTER COLUMN {name} TYPE {Type(column)}, {defaultSql}, {nullSql}";
    }
}
