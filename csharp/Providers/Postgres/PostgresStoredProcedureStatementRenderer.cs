using Datasources;

namespace Datasources.Postgres;

internal sealed class PostgresStoredProcedureStatementRenderer
{
    static string Params(IReadOnlyList<RoutineParameter> list) =>
        string.Join(",\n", list.Select(parameter => $"  {parameter.Name} {parameter.Type}"));

    static string Recordset(IReadOnlyList<RoutineParameter> columns) =>
        string.Join(", ", columns.Select(column => $"{column.Name} {column.Type}"));

    public string Render(CreateCreateStoredProcedureStatement statement) =>
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
        """;

    public string Render(CreateNestedReadStoredProcedureStatement statement)
    {
        var select = DefaultStatementTextRenderer.Instance.Render(statement.Select);
        var signature = string.Join(", ", statement.Arguments.Select(argument =>
            $"{argument.Parameter.Name} {argument.Parameter.Type}"));
        return $"""
            CREATE OR REPLACE FUNCTION {statement.Name}({signature}) RETURNS SETOF RECORD
            LANGUAGE sql AS $$ {select}; $$
            """;
    }

    public string Render(CreateReadStoredProcedureStatement statement)
    {
        var aliased = string.Join(", ", statement.Columns.Select(column => $"t.{column}"));
        if (statement.ByProperties.Count == 0)
            return $"""
                CREATE OR REPLACE FUNCTION {statement.Name}() RETURNS SETOF {statement.Table}
                LANGUAGE sql AS $$ SELECT {aliased} FROM {statement.Table} t ORDER BY t.{statement.Key}; $$
                """;
        var signature = string.Join(", ", statement.ByProperties.Select(column =>
            $"{column} {statement.Parameters.FirstOrDefault(parameter => parameter.Name == column)?.Type ?? statement.KeyType}"));
        var pred = string.Join(" AND ", statement.ByProperties.Select(column => $"t.{column} = {statement.Name}.{column}"));
        return $"""
            CREATE OR REPLACE FUNCTION {statement.Name}({signature}) RETURNS SETOF {statement.Table}
            LANGUAGE sql AS $$ SELECT {aliased} FROM {statement.Table} t WHERE {pred}; $$
            """;
    }

    public string Render(CreateUpdateStoredProcedureStatement statement)
    {
        var sets = statement.Writable.Select(column => $"    {column} = {statement.Name}.{column}").ToList();
        if (statement.AuditUpdated is { } audit)
        {
            var auditParam = statement.Parameters.FirstOrDefault(parameter => parameter.Name.Contains("new_updated"));
            if (auditParam is not null) sets.Add($"    {audit} = {statement.Name}.{auditParam.Name}");
        }
        var by = statement.ByProperties.Count > 0 ? statement.ByProperties : [statement.Key];
        var pred = string.Join(" AND ", by.Select(column => $"t.{column} = {statement.Name}.{column}"));
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
              WHERE {pred};
              GET DIAGNOSTICS affected = ROW_COUNT;
              RETURN affected;
            END;
            $$
            """;
    }

    public string Render(CreateDeleteStoredProcedureStatement statement)
    {
        var by = statement.ByProperties.Count > 0 ? statement.ByProperties : [statement.Key];
        var signature = string.Join(", ", by.Select(column =>
            $"{column} {statement.Parameters.FirstOrDefault(parameter => parameter.Name == column)?.Type ?? statement.KeyType}"));
        var pred = string.Join(" AND ", by.Select(column => $"t.{column} = {statement.Name}.{column}"));
        return $"""
            CREATE OR REPLACE FUNCTION {statement.Name}({signature}) RETURNS INT LANGUAGE plpgsql AS $$
            DECLARE
              affected INT;
            BEGIN
              DELETE FROM {statement.Table} t WHERE {pred};
              GET DIAGNOSTICS affected = ROW_COUNT;
              RETURN affected;
            END;
            $$
            """;
    }

    public string Render(CreateBulkCreateStoredProcedureStatement statement)
    {
        var cols = string.Join(", ", statement.Columns.Select(column => column.Name));
        return $"""
            CREATE OR REPLACE PROCEDURE {statement.Name}({statement.Payload.Name} jsonb, INOUT ids jsonb DEFAULT NULL)
            LANGUAGE plpgsql AS $$
            BEGIN
              WITH inserted AS (
                INSERT INTO {statement.Table} ({cols})
                SELECT {cols}
                FROM jsonb_to_recordset({statement.Payload.Name}) AS x({Recordset(statement.Columns)})
                RETURNING {statement.Key}
              )
              SELECT COALESCE(jsonb_agg({statement.Key}), '[]'::jsonb) INTO ids FROM inserted;
            END;
            $$
            """;
    }

    public string Render(CreateBulkUpdateStoredProcedureStatement statement)
    {
        var sets = string.Join(", ", statement.Columns.Where(column => column.Name != statement.Key).Select(column => $"{column.Name} = x.{column.Name}"));
        return $"""
            CREATE OR REPLACE PROCEDURE {statement.Name}({statement.Payload.Name} jsonb)
            LANGUAGE plpgsql AS $$
            BEGIN
              UPDATE {statement.Table} t SET
                {sets}
              FROM jsonb_to_recordset({statement.Payload.Name}) AS x({Recordset(statement.Columns)})
              WHERE t.{statement.Key} = x.{statement.Key};
            END;
            $$
            """;
    }

    public string Render(CreateBulkDeleteStoredProcedureStatement statement) =>
        $"""
        CREATE OR REPLACE PROCEDURE {statement.Name}({statement.Payload.Name} jsonb)
        LANGUAGE plpgsql AS $$
        BEGIN
          DELETE FROM {statement.Table} WHERE {statement.Key} IN (SELECT {statement.Key} FROM jsonb_to_recordset({statement.Payload.Name}) AS x({statement.Key} {statement.KeyType}));
        END;
        $$
        """;
}
