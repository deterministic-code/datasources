namespace Datasources;

public sealed record CreateTriggerStatement(
    string Table,
    string Name,
    string UpdatedColumn,
    string KeyColumn) : ICreateTriggerStatement
{
    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);

    public static string NameOf(string entity, StatementNaming naming)
    {
        if (naming.IsVerbatim(entity)) return $"trg_{entity}_updated_at";
        var stem = naming.PhysicalStem(entity);
        return naming.Letter(naming.Convert($"trg_{stem}_updated_at", naming.TriggerNameCasing));
    }
}
