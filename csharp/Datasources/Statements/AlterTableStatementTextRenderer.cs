namespace Datasources;

public class AlterTableStatementTextRenderer : CreateTableStatementTextRenderer
{
    public virtual string Render(AlterTableStatement statement) =>
        $"ALTER TABLE {Quote(statement.Table)} {ActionSql(statement.Action)}";

    protected virtual string ActionSql(AlterTableStatement.IAction action) => action switch
    {
        AlterTableStatement.AddColumn add => $"ADD COLUMN {Column(add.Column)}",
        AlterTableStatement.DropColumn drop => $"DROP COLUMN {Quote(drop.Name)}",
        AlterTableStatement.AlterColumn alter => $"ALTER COLUMN {Quote(alter.Column.Name)} TYPE {Type(alter.Column)}{Default(alter.Column)}{NotNullSql(alter.Column.NotNull)}",
        AlterTableStatement.RenameColumn rename => $"RENAME COLUMN {Quote(rename.From)} TO {Quote(rename.To)}",
        AlterTableStatement.RenameTable rename => $"RENAME TO {Quote(rename.To)}",
        AlterTableStatement.AddConstraint add => $"ADD {Constraint(add.Constraint)}",
        AlterTableStatement.DropConstraint drop => $"DROP CONSTRAINT {Quote(drop.Name)}",
        _ => throw new InvalidOperationException("empty alter")
    };

    protected string NotNullSql(NotNull? value) =>
        value is null ? "" : value.Name is null ? " NOT NULL" : $" CONSTRAINT {Quote(value.Name)} NOT NULL";
}
