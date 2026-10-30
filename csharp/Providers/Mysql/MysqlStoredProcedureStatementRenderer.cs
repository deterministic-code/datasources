using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlStoredProcedureStatementRenderer
{
    static string TypeOf(RoutineParameter parameter) => MysqlDriver.NativeType(parameter);

    static string Params(IReadOnlyList<RoutineParameter> list) =>
        string.Join(",\n", list.Select(parameter => $"  IN {parameter.Name} {TypeOf(parameter)}"));

    static string Where(IReadOnlyList<string> columns) =>
        string.Join(" AND ", columns.Select(column => $"t.{column} = {column}"));

    static string JsonColumns(IReadOnlyList<RoutineParameter> columns) =>
        string.Join(", ", columns.Select(column => $"{column.Name} {TypeOf(column)} PATH '$.{column.Name}'"));

    static string Procedure(string name, string signature, string body) =>
        $"""
        DROP PROCEDURE IF EXISTS {name};
        GO
        CREATE PROCEDURE {name}({signature})
        BEGIN
        {body}
        END;
        GO
        """;

    public string Render(CreateCreateStoredProcedureStatement statement)
    {
        var generated = statement.GeneratedKey;
        var insertCols = generated
            ? string.Join(", ", statement.Columns)
            : string.Join(", ", statement.Parameters.Select(parameter => parameter.Name));
        var vals = generated
            ? string.Join(", ", statement.Columns.Select(column => column == statement.Key ? "new_id" : column))
            : string.Join(", ", statement.Parameters.Select(parameter => parameter.Name));
        var body = generated
            ? $"""
              DECLARE new_id VARCHAR(36);
              SET new_id = UUID();
              INSERT INTO {MysqlDriver.Quote(statement.Table)} ({insertCols})
              VALUES ({vals});
              SELECT new_id AS {statement.Key};
            """
            : $"""
              INSERT INTO {MysqlDriver.Quote(statement.Table)} ({insertCols})
              VALUES ({vals});
              SELECT LAST_INSERT_ID() AS {statement.Key};
            """;
        return Procedure(statement.Name, $"\n{Params(statement.Parameters)}\n", body);
    }

    public string Render(CreateNestedReadStoredProcedureStatement statement)
    {
        var select = DefaultStatementTextRenderer.Instance.Render(statement.Select);
        var signature = string.Join(", ", statement.Arguments.Select(argument =>
            $"IN {argument.Parameter.Name} {TypeOf(argument.Parameter)}"));
        return Procedure(statement.Name, signature, $"  {select};");
    }

    public string Render(CreateReadStoredProcedureStatement statement)
    {
        var aliased = string.Join(", ", statement.Columns.Select(column => $"t.{column}"));
        if (statement.ByProperties.Count == 0)
            return Procedure(statement.Name, "", $"  SELECT {aliased} FROM {MysqlDriver.Quote(statement.Table)} t ORDER BY t.{statement.Key};");
        var signature = string.Join(", ", statement.ByProperties.Select(column =>
            $"IN {column} {TypeOf(statement.Parameters.FirstOrDefault(parameter => parameter.Name == column) ?? new RoutineParameter(column, statement.KeyType))}"));
        return Procedure(statement.Name, signature, $"  SELECT {aliased} FROM {MysqlDriver.Quote(statement.Table)} t WHERE {Where(statement.ByProperties)};");
    }

    public string Render(CreateUpdateStoredProcedureStatement statement)
    {
        var sets = statement.Writable.Select(column => $"    t.{column}     = {column}").ToList();
        if (statement.AuditUpdated is { } audit)
        {
            var auditParam = statement.Parameters.FirstOrDefault(parameter => parameter.Name.Contains("new_updated"));
            if (auditParam is not null) sets.Add($"    t.{audit} = {auditParam.Name}");
        }
        var by = statement.ByProperties.Count > 0 ? statement.ByProperties : [statement.Key];
        return Procedure(statement.Name, $"\n{Params(statement.Parameters)}\n", $"""
          UPDATE {MysqlDriver.Quote(statement.Table)} t
          SET
        {string.Join(",\n", sets)}
          WHERE {Where(by)};
          SELECT ROW_COUNT() AS affected;
        """);
    }

    public string Render(CreateDeleteStoredProcedureStatement statement)
    {
        var by = statement.ByProperties.Count > 0 ? statement.ByProperties : [statement.Key];
        var signature = string.Join(", ", by.Select(column =>
            $"IN {column} {TypeOf(statement.Parameters.FirstOrDefault(parameter => parameter.Name == column) ?? new RoutineParameter(column, statement.KeyType))}"));
        return Procedure(statement.Name, signature, $"""
          DELETE t FROM {MysqlDriver.Quote(statement.Table)} t WHERE {Where(by)};
          SELECT ROW_COUNT() AS affected;
        """);
    }

    public string Render(CreateBulkCreateStoredProcedureStatement statement)
    {
        var cols = string.Join(", ", statement.Columns.Select(column => column.Name));
        return Procedure(statement.Name, $"IN {statement.Payload.Name} {TypeOf(statement.Payload)}, OUT ids JSON", $"""
          INSERT INTO {MysqlDriver.Quote(statement.Table)} ({cols})
          SELECT {cols}
          FROM JSON_TABLE({statement.Payload.Name}, '$[*]' COLUMNS (
            {JsonColumns(statement.Columns)}
          )) AS payload_rows;
          GET DIAGNOSTICS @n = ROW_COUNT;
          SET @first = LAST_INSERT_ID();
          SET ids = JSON_ARRAY();
          SET @i = 0;
          WHILE @i < @n DO
            SET ids = JSON_ARRAY_APPEND(ids, '$', @first + @i);
            SET @i = @i + 1;
          END WHILE;
        """);
    }

    public string Render(CreateBulkUpdateStoredProcedureStatement statement)
    {
        var sets = string.Join(", ", statement.Columns.Where(column => column.Name != statement.Key).Select(column => $"c.{column.Name} = payload_rows.{column.Name}"));
        return Procedure(statement.Name, $"IN {statement.Payload.Name} {TypeOf(statement.Payload)}", $"""
          UPDATE {MysqlDriver.Quote(statement.Table)} c
          JOIN JSON_TABLE({statement.Payload.Name}, '$[*]' COLUMNS (
            {JsonColumns(statement.Columns)}
          )) AS payload_rows ON c.{statement.Key} = payload_rows.{statement.Key}
          SET {sets};
        """);
    }

    public string Render(CreateBulkDeleteStoredProcedureStatement statement)
    {
        var key = statement.Columns.FirstOrDefault(column => column.Name == statement.Key)
            ?? new RoutineParameter(statement.Key, statement.KeyType);
        return Procedure(statement.Name, $"IN {statement.Payload.Name} {TypeOf(statement.Payload)}",
            $"  DELETE c FROM {MysqlDriver.Quote(statement.Table)} c JOIN JSON_TABLE({statement.Payload.Name}, '$[*]' COLUMNS ({key.Name} {TypeOf(key)} PATH '$.{key.Name}')) AS payload_rows ON c.{statement.Key} = payload_rows.{statement.Key};");
    }
}
