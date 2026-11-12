using Datasources;

namespace Datasources.Memory;

internal sealed class MemoryStatementRenderer(StatementTextRenderer text) : StatementRenderer(text)
{
    public override string Visit(InsertStatement statement) =>
        string.IsNullOrEmpty(statement.Table) ? "insert" : base.Visit(statement);

    public override string Visit(UpdateStatement statement)
    {
        if (!string.IsNullOrEmpty(statement.Table))
            return base.Visit(statement);
        var where = statement.Where is null ? "" : Text.Render(statement.Where);
        if (string.IsNullOrWhiteSpace(where)) return "update";
        return where.TrimStart().StartsWith('{')
            ? $"update {where}"
            : base.Visit(new UpdateStatement("mem", statement.Fields, statement.Where));
    }

    public override string Visit(DeleteStatement statement)
    {
        if (!string.IsNullOrEmpty(statement.Table))
            return base.Visit(statement);
        var where = statement.Where is null ? "" : Text.Render(statement.Where);
        if (string.IsNullOrWhiteSpace(where)) return "delete";
        return where.TrimStart().StartsWith('{')
            ? $"delete {where}"
            : base.Visit(new DeleteStatement("mem", statement.Where));
    }

    public override string Visit(SelectStatement statement)
    {
        var where = statement.Where is null ? "" : Text.Render(statement.Where);
        if (where.TrimStart().StartsWith('{')) return where;
        return statement.Table is { Length: > 0 } ? base.Visit(statement) : "";
    }

    public override string Visit(SelectInsertedIdStatement statement) =>
        $"{{\"__inserted\":\"{statement.Table}\",\"__column\":\"{statement.Column}\"}}";
}
