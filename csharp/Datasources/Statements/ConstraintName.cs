namespace Datasources;

static class ConstraintName
{
    public static string Of(string entity, StatementNaming naming, params string[] parts) =>
        naming.IsVerbatim(entity)
            ? string.Join("_", new[] { entity }.Concat(parts))
            : naming.Letter(naming.Convert(
                string.Join("_", new[] { naming.PhysicalStem(entity) }.Concat(parts)),
                naming.ConstraintNameCasing));
}
