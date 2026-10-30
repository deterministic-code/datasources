using Datasources;

namespace Datasources.SqlServer;

internal sealed class SqlServerStoredProcedureStatementRenderer
{
    static string TypeOf(RoutineParameter parameter) => SqlServerDriver.NativeType(parameter);

    static string Params(IReadOnlyList<RoutineParameter> list) =>
        string.Join(",\n", list.Select(parameter => $"  @{parameter.Name} {TypeOf(parameter)}"));

    static string OpenJson(IReadOnlyList<RoutineParameter> columns) =>
        string.Join(", ", columns.Select(column => $"{column.Name} {TypeOf(column)} '$.{column.Name}'"));

    public string Render(CreateCreateStoredProcedureStatement statement) =>
        $"""
        CREATE OR ALTER PROCEDURE {statement.Name}
        {Params(statement.Parameters)}
        AS
        BEGIN
          INSERT INTO {SqlServerDriver.Quote(statement.Table)} ({string.Join(", ", statement.Parameters.Select(parameter => parameter.Name))})
          VALUES ({string.Join(", ", statement.Parameters.Select(parameter => $"@{parameter.Name}"))});
          SELECT CAST(SCOPE_IDENTITY() AS BIGINT) AS {statement.Key};
        END;
        GO
        """;

    public string Render(CreateNestedReadStoredProcedureStatement statement)
    {
        var select = DefaultStatementTextRenderer.Instance.Render(statement.Select);
        var signature = string.Join(",\n", statement.Arguments.Select(argument =>
            $"  @{argument.Parameter.Name} {TypeOf(argument.Parameter)}"));
        return $"""
            CREATE OR ALTER PROCEDURE {statement.Name}
            {signature}
            AS
            BEGIN
              {select};
            END;
            GO
            """;
    }

    public string Render(CreateReadStoredProcedureStatement statement)
    {
        var cols = string.Join(", ", statement.Columns);
        if (statement.ByProperties.Count == 0)
            return $"""
                CREATE OR ALTER PROCEDURE {statement.Name}
                AS
                BEGIN
                  SELECT {cols} FROM {SqlServerDriver.Quote(statement.Table)} ORDER BY {statement.Key};
                END;
                GO
                """;
        var signature = string.Join(", ", statement.ByProperties.Select(column =>
            $"@{column} {TypeOf(statement.Parameters.FirstOrDefault(parameter => parameter.Name == column) ?? new RoutineParameter(column, statement.KeyType))}"));
        var pred = string.Join(" AND ", statement.ByProperties.Select(column => $"{column} = @{column}"));
        return $"""
            CREATE OR ALTER PROCEDURE {statement.Name} {signature}
            AS
            BEGIN
              SELECT {cols} FROM {SqlServerDriver.Quote(statement.Table)} WHERE {pred};
            END;
            GO
            """;
    }

    public string Render(CreateUpdateStoredProcedureStatement statement)
    {
        var sets = statement.Writable.Select(column => $"    {column} = @{column}").ToList();
        if (statement.AuditUpdated is { } audit)
        {
            var auditParam = statement.Parameters.FirstOrDefault(parameter => parameter.Name.Contains("new_updated"));
            if (auditParam is not null) sets.Add($"    {audit} = @{auditParam.Name}");
        }
        var by = statement.ByProperties.Count > 0 ? statement.ByProperties : [statement.Key];
        var pred = string.Join(" AND ", by.Select(column => $"{column} = @{column}"));
        return $"""
            CREATE OR ALTER PROCEDURE {statement.Name}
            {Params(statement.Parameters)}
            AS
            BEGIN
              UPDATE {SqlServerDriver.Quote(statement.Table)}
              SET
            {string.Join(",\n", sets)}
              WHERE {pred};
              SELECT @@ROWCOUNT AS affected;
            END;
            GO
            """;
    }

    public string Render(CreateDeleteStoredProcedureStatement statement)
    {
        var by = statement.ByProperties.Count > 0 ? statement.ByProperties : [statement.Key];
        var signature = string.Join(", ", by.Select(column =>
            $"@{column} {TypeOf(statement.Parameters.FirstOrDefault(parameter => parameter.Name == column) ?? new RoutineParameter(column, statement.KeyType))}"));
        var pred = string.Join(" AND ", by.Select(column => $"{column} = @{column}"));
        return $"""
            CREATE OR ALTER PROCEDURE {statement.Name} {signature}
            AS
            BEGIN
              DELETE FROM {SqlServerDriver.Quote(statement.Table)} WHERE {pred};
              SELECT @@ROWCOUNT AS affected;
            END;
            GO
            """;
    }

    public string Render(CreateBulkCreateStoredProcedureStatement statement)
    {
        var cols = string.Join(", ", statement.Columns.Select(column => column.Name));
        return $"""
            CREATE OR ALTER PROCEDURE {statement.Name}
              @{statement.Payload.Name} NVARCHAR(MAX),
              @ids NVARCHAR(MAX) OUTPUT
            AS
            BEGIN
              INSERT INTO {SqlServerDriver.Quote(statement.Table)} ({cols})
              SELECT {cols}
              FROM OPENJSON(@{statement.Payload.Name}) WITH ({OpenJson(statement.Columns)});
              SET @ids = (SELECT {statement.Key} FROM {SqlServerDriver.Quote(statement.Table)} WHERE {statement.Key} >= SCOPE_IDENTITY() FOR JSON PATH);
            END;
            GO
            """;
    }

    public string Render(CreateBulkUpdateStoredProcedureStatement statement)
    {
        var sets = string.Join(", ", statement.Columns.Where(column => column.Name != statement.Key).Select(column => $"t.{column.Name} = payload_rows.{column.Name}"));
        return $"""
            CREATE OR ALTER PROCEDURE {statement.Name}
              @{statement.Payload.Name} NVARCHAR(MAX)
            AS
            BEGIN
              UPDATE t SET {sets}
              FROM {SqlServerDriver.Quote(statement.Table)} t
              JOIN OPENJSON(@{statement.Payload.Name}) WITH ({OpenJson(statement.Columns)}) AS payload_rows
                ON t.{statement.Key} = payload_rows.{statement.Key};
            END;
            GO
            """;
    }

    public string Render(CreateBulkDeleteStoredProcedureStatement statement) =>
        $"""
        CREATE OR ALTER PROCEDURE {statement.Name}
          @{statement.Payload.Name} NVARCHAR(MAX)
        AS
        BEGIN
          DELETE t FROM {SqlServerDriver.Quote(statement.Table)} t
          JOIN OPENJSON(@{statement.Payload.Name}) WITH ({statement.Key} {SqlServerDriver.NativeSpec(statement.KeyType)} '$.{statement.Key}') AS payload_rows
            ON t.{statement.Key} = payload_rows.{statement.Key};
        END;
        GO
        """;
}
