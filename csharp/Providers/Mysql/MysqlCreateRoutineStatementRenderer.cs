using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlCreateRoutineStatementRenderer : CreateRoutineStatementTextRenderer
{
    public override string Render(CreateRoutineStatement statement)
    {
        var aliased = string.Join(", ", statement.Columns.Select(column => $"t.{column}"));
        var cols = string.Join(", ", statement.Parameters.Select(parameter => parameter.Name));
        return statement.Kind switch
        {
            RoutineKind.Create => Create(statement, cols),
            RoutineKind.FindOne =>
                $"""
                DROP PROCEDURE IF EXISTS {statement.Name};
                GO
                CREATE PROCEDURE {statement.Name}(IN {statement.Key} {statement.KeyType})
                BEGIN
                  SELECT {aliased} FROM {statement.Table} t WHERE t.{statement.Key} = {statement.Key};
                END;
                GO
                """,
            RoutineKind.FindAll =>
                $"""
                DROP PROCEDURE IF EXISTS {statement.Name};
                GO
                CREATE PROCEDURE {statement.Name}()
                BEGIN
                  SELECT {aliased} FROM {statement.Table} t ORDER BY t.{statement.Key};
                END;
                GO
                """,
            RoutineKind.FindBy => FindBy(statement, aliased),
            RoutineKind.Update => Update(statement),
            RoutineKind.Delete =>
                $"""
                DROP PROCEDURE IF EXISTS {statement.Name};
                GO
                CREATE PROCEDURE {statement.Name}(IN {statement.Key} {statement.KeyType})
                BEGIN
                  DELETE t FROM {statement.Table} t WHERE t.{statement.Key} = {statement.Key};
                  SELECT ROW_COUNT() AS affected;
                END;
                GO
                """,
            _ => throw new InvalidOperationException($"empty routine kind {statement.Kind}"),
        };
    }

    static string Params(IReadOnlyList<RoutineParameter> list) =>
        string.Join(",\n", list.Select(parameter => $"  IN {parameter.Name} {parameter.Type}"));

    static string Create(CreateRoutineStatement statement, string cols)
    {
        var generated = statement.GeneratedKey;
        var insertCols = generated
            ? string.Join(", ", statement.Columns)
            : cols;
        var vals = generated
            ? string.Join(", ", statement.Columns.Select(column => column == statement.Key ? "new_id" : column))
            : string.Join(", ", statement.Parameters.Select(parameter => parameter.Name));
        return generated
            ? $"""
            DROP PROCEDURE IF EXISTS {statement.Name};
            GO
            CREATE PROCEDURE {statement.Name}(
            {Params(statement.Parameters)}
            )
            BEGIN
              DECLARE new_id VARCHAR(36);
              SET new_id = UUID();
              INSERT INTO {statement.Table} ({insertCols})
              VALUES ({vals});
              SELECT new_id AS {statement.Key};
            END;
            GO
            """
            : $"""
            DROP PROCEDURE IF EXISTS {statement.Name};
            GO
            CREATE PROCEDURE {statement.Name}(
            {Params(statement.Parameters)}
            )
            BEGIN
              INSERT INTO {statement.Table} ({cols})
              VALUES ({vals});
              SELECT LAST_INSERT_ID() AS {statement.Key};
            END;
            GO
            """;
    }

    static string FindBy(CreateRoutineStatement statement, string aliased)
    {
        var by = statement.ByField ?? statement.Key;
        var type = statement.Parameters.Count > 0 ? statement.Parameters[0].Type : statement.KeyType;
        return $"""
            DROP PROCEDURE IF EXISTS {statement.Name};
            GO
            CREATE PROCEDURE {statement.Name}(IN {by} {type})
            BEGIN
              SELECT {aliased} FROM {statement.Table} t WHERE t.{by} = {by};
            END;
            GO
            """;
    }

    static string Update(CreateRoutineStatement statement)
    {
        var sets = statement.Writable.Select(column => $"    t.{column}     = {column}").ToList();
        if (statement.AuditUpdated is { } audit)
        {
            var auditParam = statement.Parameters.FirstOrDefault(parameter => parameter.Name.Contains("new_updated"));
            if (auditParam is not null) sets.Add($"    t.{audit} = {auditParam.Name}");
        }
        var by = statement.ByField ?? statement.Key;
        return $"""
            DROP PROCEDURE IF EXISTS {statement.Name};
            GO
            CREATE PROCEDURE {statement.Name}(
            {Params(statement.Parameters)}
            )
            BEGIN
              UPDATE {statement.Table} t
              SET
            {string.Join(",\n", sets)}
              WHERE t.{by} = {by};
              SELECT ROW_COUNT() AS affected;
            END;
            GO
            """;
    }
}
