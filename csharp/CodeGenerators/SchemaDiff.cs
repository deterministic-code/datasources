using System.Reflection;
using System.Text.Json;

namespace Datasources.CodeGenerators;

public static class SchemaDiff
{
    public static IReadOnlyList<IStatement> Of(
        string dialect,
        IReadOnlyList<SqlTable> beforeTables,
        SqlMapping beforeMap,
        IReadOnlyList<SqlTable> afterTables,
        SqlMapping afterMap)
    {
        var previous = beforeTables.ToDictionary(table => table.Name, StringComparer.Ordinal);
        var next = afterTables.ToDictionary(table => table.Name, StringComparer.Ordinal);
        var removed = beforeTables.Where(table => !next.ContainsKey(table.Name)).Reverse();
        var added = afterTables.Where(table => !previous.ContainsKey(table.Name));
        var kept = afterTables.Where(table => previous.ContainsKey(table.Name));
        return
        [
            .. removed.SelectMany(table => DropTable(dialect, table, beforeMap)),
            .. kept.SelectMany(table => DiffTable(dialect, previous[table.Name], table, beforeMap, afterMap)),
            .. added.SelectMany(table => CreateTable(dialect, table, afterMap)),
        ];
    }

    static IEnumerable<IStatement> CreateTable(string dialect, SqlTable table, SqlMapping mapping)
    {
        yield return TableStatements.CreateTable(table, mapping);
        foreach (var index in table.Indexes)
            yield return TableStatements.CreateIndex(mapping, table, index);
        if (SqlSchema.HasAuditColumns(table))
            yield return Trigger(dialect, table, mapping);
    }

    static IEnumerable<IStatement> DropTable(string dialect, SqlTable table, SqlMapping mapping)
    {
        if (SqlSchema.HasAuditColumns(table))
            yield return DropTrigger(dialect, table, mapping);
        foreach (var index in table.Indexes.Reverse())
            yield return DropIndex(dialect, mapping, table, index);
        yield return TableStatements.DropTable(table.TableName);
    }

    static IEnumerable<IStatement> DiffTable(
        string dialect,
        SqlTable before,
        SqlTable after,
        SqlMapping beforeMap,
        SqlMapping afterMap)
    {
        if (JsonSerializer.Serialize(IdentityOf(before)) != JsonSerializer.Serialize(IdentityOf(after)))
            throw new InvalidOperationException($"{after.Name}: primary key or identity change is not supported");
        var beforeFields = before.Fields.ToDictionary(field => field.Name, StringComparer.Ordinal);
        var afterFields = after.Fields.ToDictionary(field => field.Name, StringComparer.Ordinal);
        var beforeIndexes = before.Indexes.ToDictionary(index => index.Name, StringComparer.Ordinal);
        var afterIndexes = after.Indexes.ToDictionary(index => index.Name, StringComparer.Ordinal);

        foreach (var index in before.Indexes)
            if (!afterIndexes.TryGetValue(index.Name, out var next) || IndexKey(index) != IndexKey(next))
                yield return DropIndex(dialect, beforeMap, before, index);

        foreach (var field in before.Fields)
        {
            if (UniqueField(before, field) && (!afterFields.TryGetValue(field.Name, out var kept) || !UniqueField(after, kept)))
                yield return new AlterTableStatement(before.TableName, new AlterTableStatement.DropConstraint(
                    TableStatements.UniqueOf(beforeMap, before, field).Name));
            if (field.References is not null || field.ReferencePair is not null)
            {
                var same = afterFields.TryGetValue(field.Name, out var next) && RefsOf(next) == RefsOf(field);
                if (!same)
                    yield return TableStatements.DropForeignKey(beforeMap, before, field);
            }
        }

        if (SqlSchema.HasAuditColumns(before) && !SqlSchema.HasAuditColumns(after))
            yield return DropTrigger(dialect, before, beforeMap);

        foreach (var field in before.Fields.Where(field => !afterFields.ContainsKey(field.Name)))
            yield return new AlterTableStatement(before.TableName, new AlterTableStatement.DropColumn(
                TableStatements.Column(beforeMap, before, field).Name));

        var tableName = after.TableName;
        if (before.TableName != after.TableName)
            yield return new AlterTableStatement(before.TableName, new AlterTableStatement.RenameTable(after.TableName));

        foreach (var field in after.Fields.Where(field => beforeFields.ContainsKey(field.Name)))
        {
            var previous = TableStatements.Column(beforeMap, before, beforeFields[field.Name]);
            var next = TableStatements.Column(afterMap, after, field);
            if (previous.Name != next.Name)
                yield return new AlterTableStatement(tableName, new AlterTableStatement.RenameColumn(previous.Name, next.Name));
            if (Compiled(dialect, previous with { Name = next.Name }) != Compiled(dialect, next))
                yield return new AlterTableStatement(tableName, new AlterTableStatement.AlterColumn(next));
        }

        foreach (var field in after.Fields.Where(field => !beforeFields.ContainsKey(field.Name)))
            yield return new AlterTableStatement(tableName, new AlterTableStatement.AddColumn(
                TableStatements.Column(afterMap, after, field)));

        foreach (var field in after.Fields)
        {
            if (UniqueField(after, field) && (!beforeFields.TryGetValue(field.Name, out var previous) || !UniqueField(before, previous)))
                yield return new AlterTableStatement(tableName, new AlterTableStatement.AddConstraint(
                    TableStatements.UniqueOf(afterMap, after, field)));
            if ((field.References is not null || field.ReferencePair is not null)
                && (!beforeFields.TryGetValue(field.Name, out var prior) || RefsOf(prior) != RefsOf(field)))
                yield return TableStatements.AddForeignKey(afterMap, after, field);
        }

        foreach (var index in after.Indexes)
            if (!beforeIndexes.TryGetValue(index.Name, out var previous) || IndexKey(previous) != IndexKey(index))
                yield return TableStatements.CreateIndex(afterMap, after, index);

        if (!SqlSchema.HasAuditColumns(before) && SqlSchema.HasAuditColumns(after))
            yield return Trigger(dialect, after, afterMap);
    }

    static Column Compiled(string dialect, Column column)
    {
        var native = GenerateSql.Driver(dialect).GetType()
            .GetMethod("NativeType", BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Static, [typeof(Column)]);
        var type = native is null ? column.Type : (string)native.Invoke(null, [column])!;
        return column with { Type = type, Field = null };
    }

    static CreateTriggerStatement Trigger(string dialect, SqlTable table, SqlMapping mapping) =>
        TableStatements.CreateTrigger(table, mapping, SpecTables.PkName(table.AsType(), SqlSchema.OverlayOf(table)) ?? "id");

    static DropTriggerStatement DropTrigger(string dialect, SqlTable table, SqlMapping mapping) =>
        TableStatements.DropTrigger(mapping.TriggerName(table), table.TableName);

    static DropIndexStatement DropIndex(string dialect, SqlMapping mapping, SqlTable table, DatasourceIndex index) =>
        TableStatements.DropIndex(mapping.ConstraintName(table, index.Name, "index"), table.TableName);

    static bool UniqueField(SqlTable table, SpecTypeField field) =>
        SqlSchema.FieldOverlay(table, field.Name)?.IsUnique == true || field.Name == "uuid";

    static string IndexKey(DatasourceIndex index) => JsonSerializer.Serialize(new { index.Name, index.Fields, index.IsUnique });

    static object IdentityOf(SqlTable table) => new { Keys = SqlSchema.TableIdentity(table), Generated = SqlSchema.IsGeneratedIdentity(table) };

    static string RefsOf(SpecTypeField field) => JsonSerializer.Serialize(field.ReferencePair ?? (object?)field.References);
}
