using Datasources;

namespace Datasources.Oracle;

internal sealed class OracleStoredProcedureStatementRenderer
{
    static string Params(IReadOnlyList<RoutineParameter> list) =>
        string.Join(",\n", list.Select(parameter => $"  {parameter.Name} IN {parameter.Type}"));

    static string JsonColumns(IReadOnlyList<RoutineParameter> columns) =>
        string.Join(", ", columns.Select(column => $"{column.Name} {column.Type} PATH '$.{column.Name}'"));

    public string Render(CreateCreateStoredProcedureStatement statement) =>
        $"""
        CREATE OR REPLACE PROCEDURE {statement.Name}(
        {Params(statement.Parameters)}
        ) AS
        BEGIN
          INSERT INTO {statement.Table} ({string.Join(", ", statement.Parameters.Select(parameter => parameter.Name))})
          VALUES ({string.Join(", ", statement.Parameters.Select(parameter => parameter.Name))});
        END;
        """;

    public string Render(CreateReadStoredProcedureStatement statement)
    {
        var aliased = string.Join(", ", statement.Columns.Select(column => $"t.{column}"));
        if (statement.ByProperties.Count == 0)
            return $"""
                CREATE OR REPLACE PROCEDURE {statement.Name} AS
                BEGIN
                  SELECT {aliased} FROM {statement.Table} t ORDER BY t.{statement.Key};
                END;
                """;
        var signature = string.Join(", ", statement.ByProperties.Select(column =>
            $"{column} IN {statement.Parameters.FirstOrDefault(parameter => parameter.Name == column)?.Type ?? statement.KeyType}"));
        var pred = string.Join(" AND ", statement.ByProperties.Select(column => $"t.{column} = {column}"));
        return $"""
            CREATE OR REPLACE PROCEDURE {statement.Name}({signature}) AS
            BEGIN
              SELECT {aliased} FROM {statement.Table} t WHERE {pred};
            END;
            """;
    }

    public string Render(CreateUpdateStoredProcedureStatement statement)
    {
        var sets = statement.Writable.Select(column => $"    t.{column} = {column}").ToList();
        if (statement.AuditUpdated is { } audit)
        {
            var auditParam = statement.Parameters.FirstOrDefault(parameter => parameter.Name.Contains("new_updated"));
            if (auditParam is not null) sets.Add($"    t.{audit} = {auditParam.Name}");
        }
        var by = statement.ByProperties.Count > 0 ? statement.ByProperties : [statement.Key];
        var pred = string.Join(" AND ", by.Select(column => $"t.{column} = {column}"));
        return $"""
            CREATE OR REPLACE PROCEDURE {statement.Name}(
            {Params(statement.Parameters)}
            ) AS
            BEGIN
              UPDATE {statement.Table} t
              SET
            {string.Join(",\n", sets)}
              WHERE {pred};
            END;
            """;
    }

    public string Render(CreateDeleteStoredProcedureStatement statement)
    {
        var by = statement.ByProperties.Count > 0 ? statement.ByProperties : [statement.Key];
        var signature = string.Join(", ", by.Select(column =>
            $"{column} IN {statement.Parameters.FirstOrDefault(parameter => parameter.Name == column)?.Type ?? statement.KeyType}"));
        var pred = string.Join(" AND ", by.Select(column => $"t.{column} = {column}"));
        return $"""
            CREATE OR REPLACE PROCEDURE {statement.Name}({signature}) AS
            BEGIN
              DELETE FROM {statement.Table} t WHERE {pred};
            END;
            """;
    }

    public string Render(CreateBulkCreateStoredProcedureStatement statement)
    {
        var cols = string.Join(", ", statement.Columns.Select(column => column.Name));
        return $"""
            CREATE OR REPLACE PROCEDURE {statement.Name}({statement.Payload.Name} IN CLOB) AS
            BEGIN
              INSERT INTO {statement.Table} ({cols})
              SELECT {cols}
              FROM JSON_TABLE({statement.Payload.Name}, '$[*]' COLUMNS ({JsonColumns(statement.Columns)}));
            END;
            """;
    }

    public string Render(CreateBulkUpdateStoredProcedureStatement statement)
    {
        var sets = string.Join(", ", statement.Columns.Where(column => column.Name != statement.Key).Select(column => $"t.{column.Name} = payload_rows.{column.Name}"));
        return $"""
            CREATE OR REPLACE PROCEDURE {statement.Name}({statement.Payload.Name} IN CLOB) AS
            BEGIN
              MERGE INTO {statement.Table} t
              USING (
                SELECT * FROM JSON_TABLE({statement.Payload.Name}, '$[*]' COLUMNS ({JsonColumns(statement.Columns)}))
              ) payload_rows ON (t.{statement.Key} = payload_rows.{statement.Key})
              WHEN MATCHED THEN UPDATE SET {sets};
            END;
            """;
    }

    public string Render(CreateBulkDeleteStoredProcedureStatement statement) =>
        $"""
        CREATE OR REPLACE PROCEDURE {statement.Name}({statement.Payload.Name} IN CLOB) AS
        BEGIN
          DELETE FROM {statement.Table} t
          WHERE t.{statement.Key} IN (
            SELECT {statement.Key} FROM JSON_TABLE({statement.Payload.Name}, '$[*]' COLUMNS ({statement.Key} {statement.KeyType} PATH '$.{statement.Key}'))
          );
        END;
        """;
}
