using Datasources;

namespace Datasources.SqlServer;

internal sealed class SqlServerCreateRoutineStatementRenderer : CreateRoutineStatementTextRenderer
{
    public override string Render(CreateRoutineStatement statement)
    {
        var cols = string.Join(", ", statement.Columns);
        return statement.Kind switch
        {
            RoutineKind.Create =>
                $"""
                CREATE OR ALTER PROCEDURE {statement.Name}
                {Params(statement.Parameters)}
                AS
                BEGIN
                  INSERT INTO {statement.Table} ({string.Join(", ", statement.Parameters.Select(parameter => parameter.Name))})
                  VALUES ({string.Join(", ", statement.Parameters.Select(parameter => $"@{parameter.Name}"))});
                  SELECT CAST(SCOPE_IDENTITY() AS BIGINT) AS id;
                END;
                GO
                """,
            RoutineKind.FindOne =>
                $"""
                CREATE OR ALTER PROCEDURE {statement.Name} @{statement.Key} {statement.KeyType}
                AS
                BEGIN
                  SELECT {cols} FROM {statement.Table} WHERE {statement.Key} = @{statement.Key};
                END;
                GO
                """,
            RoutineKind.FindAll =>
                $"""
                CREATE OR ALTER PROCEDURE {statement.Name}
                AS
                BEGIN
                  SELECT {cols} FROM {statement.Table} ORDER BY {statement.Key};
                END;
                GO
                """,
            RoutineKind.FindBy => FindBy(statement, cols),
            RoutineKind.Update => Update(statement),
            RoutineKind.Delete =>
                $"""
                CREATE OR ALTER PROCEDURE {statement.Name} @{statement.Key} {statement.KeyType}
                AS
                BEGIN
                  DELETE FROM {statement.Table} WHERE {statement.Key} = @{statement.Key};
                  SELECT @@ROWCOUNT AS affected;
                END;
                GO
                """,
            _ => throw new InvalidOperationException($"empty routine kind {statement.Kind}"),
        };
    }

    static string Params(IReadOnlyList<RoutineParameter> list) =>
        string.Join(",\n", list.Select(parameter => $"  @{parameter.Name} {parameter.Type}"));

    static string FindBy(CreateRoutineStatement statement, string cols)
    {
        var by = statement.ByField ?? statement.Key;
        var type = statement.Parameters.Count > 0 ? statement.Parameters[0].Type : statement.KeyType;
        return $"""
            CREATE OR ALTER PROCEDURE {statement.Name} @{by} {type}
            AS
            BEGIN
              SELECT {cols} FROM {statement.Table} WHERE {by} = @{by};
            END;
            GO
            """;
    }

    static string Update(CreateRoutineStatement statement)
    {
        var sets = statement.Writable.Select(column => $"    {column} = @{column}").ToList();
        if (statement.AuditUpdated is { } audit)
        {
            var auditParam = statement.Parameters.FirstOrDefault(parameter => parameter.Name.Contains("new_updated"));
            if (auditParam is not null) sets.Add($"    {audit} = @{auditParam.Name}");
        }
        var by = statement.ByField ?? statement.Key;
        return $"""
            CREATE OR ALTER PROCEDURE {statement.Name}
            {Params(statement.Parameters)}
            AS
            BEGIN
              UPDATE {statement.Table}
              SET
            {string.Join(",\n", sets)}
              WHERE {by} = @{by};
              SELECT @@ROWCOUNT AS affected;
            END;
            GO
            """;
    }
}
