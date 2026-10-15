using Datasources;

namespace Datasources.Postgres;

internal sealed class PostgresCreateRoutineStatementRenderer : CreateRoutineStatementTextRenderer
{
    public override string Render(CreateRoutineStatement statement)
    {
        var aliased = string.Join(", ", statement.Columns.Select(column => $"t.{column}"));
        return statement.Kind switch
        {
            RoutineKind.Create =>
                $"""
                CREATE OR REPLACE FUNCTION {statement.Name}(
                {Params(statement.Parameters)}
                ) RETURNS {statement.KeyType} LANGUAGE plpgsql AS $$
                DECLARE
                  new_id {statement.KeyType};
                BEGIN
                  INSERT INTO {statement.Table} ({string.Join(", ", statement.Parameters.Select(parameter => parameter.Name))})
                  VALUES ({string.Join(", ", statement.Parameters.Select(parameter => parameter.Name))})
                  RETURNING {statement.Key} INTO new_id;
                  RETURN new_id;
                END;
                $$
                """,
            RoutineKind.FindOne =>
                $"""
                CREATE OR REPLACE FUNCTION {statement.Name}({statement.Key} {statement.KeyType}) RETURNS SETOF {statement.Table}
                LANGUAGE sql AS $$ SELECT {aliased} FROM {statement.Table} t WHERE t.{statement.Key} = {statement.Name}.{statement.Key}; $$
                """,
            RoutineKind.FindAll =>
                $"""
                CREATE OR REPLACE FUNCTION {statement.Name}() RETURNS SETOF {statement.Table}
                LANGUAGE sql AS $$ SELECT {aliased} FROM {statement.Table} t ORDER BY t.{statement.Key}; $$
                """,
            RoutineKind.FindBy => FindBy(statement, aliased),
            RoutineKind.Update => Update(statement),
            RoutineKind.Delete =>
                $"""
                CREATE OR REPLACE FUNCTION {statement.Name}({statement.Key} {statement.KeyType}) RETURNS INT LANGUAGE plpgsql AS $$
                DECLARE
                  affected INT;
                BEGIN
                  DELETE FROM {statement.Table} t WHERE t.{statement.Key} = {statement.Name}.{statement.Key};
                  GET DIAGNOSTICS affected = ROW_COUNT;
                  RETURN affected;
                END;
                $$
                """,
            _ => throw new InvalidOperationException($"empty routine kind {statement.Kind}"),
        };
    }

    static string Params(IReadOnlyList<RoutineParameter> list) =>
        string.Join(",\n", list.Select(parameter => $"  {parameter.Name} {parameter.Type}"));

    static string FindBy(CreateRoutineStatement statement, string aliased)
    {
        var by = statement.ByField ?? statement.Key;
        var type = statement.Parameters.Count > 0 ? statement.Parameters[0].Type : statement.KeyType;
        return $"""
            CREATE OR REPLACE FUNCTION {statement.Name}({by} {type}) RETURNS SETOF {statement.Table}
            LANGUAGE sql AS $$ SELECT {aliased} FROM {statement.Table} t WHERE t.{by} = {statement.Name}.{by}; $$
            """;
    }

    static string Update(CreateRoutineStatement statement)
    {
        var sets = statement.Writable.Select(column => $"    {column} = {statement.Name}.{column}").ToList();
        if (statement.AuditUpdated is { } audit)
        {
            var auditParam = statement.Parameters.FirstOrDefault(parameter => parameter.Name.Contains("new_updated"));
            if (auditParam is not null) sets.Add($"    {audit} = {statement.Name}.{auditParam.Name}");
        }
        var by = statement.ByField ?? statement.Key;
        return $"""
            CREATE OR REPLACE FUNCTION {statement.Name}(
            {Params(statement.Parameters)}
            ) RETURNS INT LANGUAGE plpgsql AS $$
            DECLARE
              affected INT;
            BEGIN
              UPDATE {statement.Table} t
              SET
            {string.Join(",\n", sets)}
              WHERE t.{by} = {statement.Name}.{by};
              GET DIAGNOSTICS affected = ROW_COUNT;
              RETURN affected;
            END;
            $$
            """;
    }
}
