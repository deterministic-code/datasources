namespace Datasources;

public sealed record AlterTableStatement(string Table, AlterTableStatement.IAction Action) : IAlterTableStatement
{
    public interface IAction;

    public sealed record AddColumn(Column Column) : IAction;
    public sealed record DropColumn(string Name) : IAction;
    public sealed record AlterColumn(Column Column) : IAction;
    public sealed record RenameColumn(string From, string To) : IAction;
    public sealed record RenameTable(string To) : IAction;
    public sealed record AddConstraint(ITableConstraint Constraint) : IAction;
    public sealed record DropConstraint(string Name) : IAction;

    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
