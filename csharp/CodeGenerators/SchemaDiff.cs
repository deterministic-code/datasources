using System.Reflection;
using System.Text.Json;

namespace Datasources.CodeGenerators;

public static class SchemaDiff
{
    public static IReadOnlyList<IStatement> Of(
        string dialect,
        IReadOnlyList<SqlTable> beforeTables,
        StatementNaming beforeNaming,
        IReadOnlyList<SqlTable> afterTables,
        StatementNaming afterNaming,
        IReadOnlyDictionary<string, string>? settings = null)
    {
        var previousByName = beforeTables.ToDictionary(table => table.Name, StringComparer.Ordinal);
        var nextByName = afterTables.ToDictionary(table => table.Name, StringComparer.Ordinal);
        var previousByTable = beforeTables.ToDictionary(table => table.TableName, StringComparer.Ordinal);
        var nextByTable = afterTables.ToDictionary(table => table.TableName, StringComparer.Ordinal);
        var removed = beforeTables.Where(table => PriorOf(nextByName, nextByTable, table) is null).Reverse();
        var added = afterTables.Where(table => PriorOf(previousByName, previousByTable, table) is null);
        var kept = afterTables
            .Select(table => (Before: PriorOf(previousByName, previousByTable, table), After: table))
            .Where(pair => pair.Before is not null);
        return
        [
            .. removed.SelectMany(table => DropTable(dialect, table, beforeNaming, beforeTables)),
            .. kept.SelectMany(pair => DiffTable(dialect, pair.Before!, pair.After, beforeNaming, afterNaming, beforeTables, afterTables, settings)),
            .. added.SelectMany(table => CreateTable(dialect, table, afterNaming, afterTables)),
        ];
    }

    static SqlTable? PriorOf(
        IReadOnlyDictionary<string, SqlTable> byName,
        IReadOnlyDictionary<string, SqlTable> byTable,
        SqlTable table) =>
        byName.TryGetValue(table.Name, out var named) ? named : byTable.GetValueOrDefault(table.TableName);

    static IEnumerable<IStatement> CreateTable(
        string dialect,
        SqlTable table,
        StatementNaming naming,
        IReadOnlyList<SqlTable> tables)
    {
        yield return TableStatements.CreateTable(table, naming, tables);
        foreach (var index in table.Indexes)
            yield return TableStatements.CreateIndex(naming, table, index);
        if (SqlSchema.HasAuditColumns(table))
            yield return Trigger(dialect, table, naming);
    }

    static IEnumerable<IStatement> DropTable(
        string dialect,
        SqlTable table,
        StatementNaming naming,
        IReadOnlyList<SqlTable> tables)
    {
        if (SqlSchema.HasAuditColumns(table))
            yield return DropTrigger(dialect, table, naming);
        yield return TableStatements.DropTable(table.TableName);
    }

    static IEnumerable<IStatement> DiffTable(
        string dialect,
        SqlTable before,
        SqlTable after,
        StatementNaming beforeNaming,
        StatementNaming afterNaming,
        IReadOnlyList<SqlTable> beforeTables,
        IReadOnlyList<SqlTable> afterTables,
        IReadOnlyDictionary<string, string>? settings)
    {
        if (JsonSerializer.Serialize(IdentityOf(before)) != JsonSerializer.Serialize(IdentityOf(after)))
            throw new InvalidOperationException($"{after.Name}: primary key or identity change is not supported");
        if (SqlSchema.RebuildTable(dialect, settings) && NeedsRebuild(dialect, before, after, beforeNaming, afterNaming))
        {
            yield return Rebuild(dialect, before, after, beforeNaming, afterNaming, afterTables);
            yield break;
        }
        var beforeFields = before.Fields.ToDictionary(field => field.Name, StringComparer.Ordinal);
        var afterFields = after.Fields.ToDictionary(field => field.Name, StringComparer.Ordinal);
        var beforeIndexes = before.Indexes.ToDictionary(index => index.Name, StringComparer.Ordinal);
        var afterIndexes = after.Indexes.ToDictionary(index => index.Name, StringComparer.Ordinal);
        var droppedForeignKeys = new HashSet<string>(StringComparer.Ordinal);
        var createdIndexes = new HashSet<string>(StringComparer.Ordinal);

        foreach (var index in before.Indexes)
        {
            if (afterIndexes.TryGetValue(index.Name, out var next) && IndexKey(index) == IndexKey(next))
                continue;
            if (dialect == "mysql")
                foreach (var statement in MysqlIndexDrop(
                    before, after, beforeNaming, afterNaming, beforeTables, afterFields, droppedForeignKeys, createdIndexes, index))
                    yield return statement;
            yield return DropIndex(dialect, beforeNaming, before, index);
        }

        foreach (var field in before.Fields)
        {
            if (UniqueField(before, field) && (!afterFields.TryGetValue(field.Name, out var kept) || !UniqueField(after, kept)))
                yield return new AlterTableStatement(before.TableName, new AlterTableStatement.DropConstraint(
                    TableStatements.UniqueOf(beforeNaming, before, field).Name));
            if (IsForeignKey(field) && !SameRefs(field, afterFields) && droppedForeignKeys.Add(field.Name))
                yield return TableStatements.DropForeignKey(beforeNaming, beforeTables, before, field);
        }

        if (SqlSchema.HasAuditColumns(before) && !SqlSchema.HasAuditColumns(after))
            yield return DropTrigger(dialect, before, beforeNaming);

        foreach (var field in before.Fields.Where(field => !afterFields.ContainsKey(field.Name)))
            yield return new AlterTableStatement(before.TableName, new AlterTableStatement.DropColumn(
                TableStatements.Column(beforeNaming, before, field).Name));

        var tableName = after.TableName;
        if (before.TableName != after.TableName)
            yield return new AlterTableStatement(before.TableName, new AlterTableStatement.RenameTable(after.TableName));

        foreach (var field in after.Fields.Where(field => beforeFields.ContainsKey(field.Name)))
        {
            var previous = TableStatements.Column(beforeNaming, before, beforeFields[field.Name]);
            var next = TableStatements.Column(afterNaming, after, field);
            if (previous.Name != next.Name)
                yield return new AlterTableStatement(tableName, new AlterTableStatement.RenameColumn(previous.Name, next.Name));
            if (Compiled(dialect, previous with { Name = next.Name }) != Compiled(dialect, next))
                yield return new AlterTableStatement(tableName, new AlterTableStatement.AlterColumn(next));
        }

        foreach (var field in after.Fields.Where(field => !beforeFields.ContainsKey(field.Name)))
            yield return new AlterTableStatement(tableName, new AlterTableStatement.AddColumn(
                TableStatements.Column(afterNaming, after, field)));

        foreach (var field in after.Fields)
        {
            if (UniqueField(after, field) && (!beforeFields.TryGetValue(field.Name, out var previous) || !UniqueField(before, previous)))
                yield return new AlterTableStatement(tableName, new AlterTableStatement.AddConstraint(
                    TableStatements.UniqueOf(afterNaming, after, field)));
            if ((field.References is not null || field.ReferencePair is not null)
                && (!beforeFields.TryGetValue(field.Name, out var prior) || RefsOf(prior) != RefsOf(field)))
                yield return TableStatements.AddForeignKey(afterNaming, afterTables, after, field);
        }

        foreach (var index in after.Indexes)
            if ((!beforeIndexes.TryGetValue(index.Name, out var previous) || IndexKey(previous) != IndexKey(index))
                && createdIndexes.Add(index.Name))
                yield return TableStatements.CreateIndex(afterNaming, after, index);

        if (!SqlSchema.HasAuditColumns(before) && SqlSchema.HasAuditColumns(after))
            yield return Trigger(dialect, after, afterNaming);
    }

    static bool NeedsRebuild(
        string dialect,
        SqlTable before,
        SqlTable after,
        StatementNaming beforeNaming,
        StatementNaming afterNaming)
    {
        var beforeFields = before.Fields.ToDictionary(field => field.Name, StringComparer.Ordinal);
        var afterFields = after.Fields.ToDictionary(field => field.Name, StringComparer.Ordinal);
        foreach (var field in after.Fields.Where(field => beforeFields.ContainsKey(field.Name)))
        {
            var previous = TableStatements.Column(beforeNaming, before, beforeFields[field.Name]);
            var next = TableStatements.Column(afterNaming, after, field);
            if (Compiled(dialect, previous with { Name = next.Name }) != Compiled(dialect, next))
                return true;
        }
        return before.Fields.Concat(after.Fields).Any(field =>
        {
            if (field.References is null && field.ReferencePair is null) return false;
            var same = afterFields.TryGetValue(field.Name, out var next)
                && beforeFields.TryGetValue(field.Name, out var prior)
                && RefsOf(next) == RefsOf(prior);
            return !same;
        });
    }

    static RebuildTableStatement Rebuild(
        string dialect,
        SqlTable before,
        SqlTable after,
        StatementNaming beforeNaming,
        StatementNaming afterNaming,
        IReadOnlyList<SqlTable> afterTables)
    {
        var beforeFields = before.Fields.ToDictionary(field => field.Name, StringComparer.Ordinal);
        var columns = after.Fields.Select(field =>
        {
            var next = TableStatements.Column(afterNaming, after, field);
            if (!beforeFields.TryGetValue(field.Name, out var prior))
            {
                if (next.NotNull is not null && next.Default is null)
                    throw new InvalidOperationException($"{after.Name}.{field.Name}: new NOT NULL column with no default");
                return new RebuildColumn(next.Name, Literal: next.Default?.Value);
            }
            var previous = TableStatements.Column(beforeNaming, before, prior);
            var prevType = Compiled(dialect, previous with { Name = next.Name }).Type;
            var nextType = Compiled(dialect, next).Type;
            return new RebuildColumn(next.Name, previous.Name, prevType == nextType ? null : nextType);
        }).ToList();
        IStatement[] recreate =
        [
            .. after.Indexes.Select(index => TableStatements.CreateIndex(afterNaming, after, index)),
            .. SqlSchema.HasAuditColumns(after) ? [Trigger(dialect, after, afterNaming)] : Array.Empty<IStatement>(),
        ];
        return new RebuildTableStatement(before.TableName, TableStatements.CreateTable(after, afterNaming, afterTables), columns, recreate);
    }

    static Column Compiled(string dialect, Column column)
    {
        var native = GenerateSql.Driver(dialect).GetType()
            .GetMethod("NativeType", BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Static, [typeof(Column)]);
        var type = native is null ? column.Type : (string)native.Invoke(null, [column])!;
        return column with { Type = type, Field = null };
    }

    static CreateTriggerStatement Trigger(string dialect, SqlTable table, StatementNaming naming) =>
        TableStatements.CreateTrigger(table, naming, SpecTables.PkName(table.AsType(), SqlSchema.OverlayOf(table)));

    static DropTriggerStatement DropTrigger(string dialect, SqlTable table, StatementNaming naming) =>
        TableStatements.DropTrigger(CreateTriggerStatement.NameOf(SqlSchema.TableStem(table), naming), table.TableName);

    static DropIndexStatement DropIndex(string dialect, StatementNaming naming, SqlTable table, DatasourceIndex index) =>
        TableStatements.DropIndex(IndexPhysical(naming, table, index), table.TableName);

    static IEnumerable<IStatement> MysqlIndexDrop(
        SqlTable before,
        SqlTable after,
        StatementNaming beforeNaming,
        StatementNaming afterNaming,
        IReadOnlyList<SqlTable> beforeTables,
        Dictionary<string, SpecTypeField> afterFields,
        HashSet<string> droppedForeignKeys,
        HashSet<string> createdIndexes,
        DatasourceIndex index)
    {
        var droppedPhysical = IndexPhysical(beforeNaming, before, index);
        foreach (var field in before.Fields.Where(item => IndexSupports(index, item)))
        {
            if (!SameRefs(field, afterFields))
            {
                if (droppedForeignKeys.Add(field.Name))
                    yield return TableStatements.DropForeignKey(beforeNaming, beforeTables, before, field);
                continue;
            }
            var supporting = after.Indexes.Where(item =>
                IndexSupports(item, field) && IndexPhysical(afterNaming, after, item) != droppedPhysical).ToList();
            if (PkSupports(after, field)
                || afterFields.TryGetValue(field.Name, out var kept) && UniqueField(after, kept)
                || supporting.Count > 0)
            {
                foreach (var added in supporting)
                    if (!before.Indexes.Any(prior => prior.Name == added.Name && IndexKey(prior) == IndexKey(added))
                        && createdIndexes.Add(added.Name))
                        yield return TableStatements.CreateIndex(afterNaming, after, added);
                continue;
            }
            var support = new DatasourceIndex
            {
                Name = SupportingIndexName(afterNaming, after, droppedPhysical, field.Name),
                Fields = [field.Name],
                IsUnique = false,
            };
            createdIndexes.Add(support.Name);
            yield return TableStatements.CreateIndex(afterNaming, after, support);
        }
    }

    static string IndexPhysical(StatementNaming naming, SqlTable table, DatasourceIndex index) =>
        CreateIndexStatement.NameOf(SqlSchema.TableStem(table), naming, index.Name, "index");

    static string SupportingIndexName(StatementNaming naming, SqlTable table, string droppedPhysical, string field)
    {
        foreach (var candidate in new[] { field, $"{field}_fk" })
            if (IndexPhysical(naming, table, new DatasourceIndex { Name = candidate, Fields = [field], IsUnique = false }) != droppedPhysical)
                return candidate;
        return $"{field}_supporting";
    }

    static bool IsForeignKey(SpecTypeField field) =>
        field.References is not null || field.ReferencePair is not null;

    static bool SameRefs(SpecTypeField field, IReadOnlyDictionary<string, SpecTypeField> afterFields) =>
        afterFields.TryGetValue(field.Name, out var next) && RefsOf(next) == RefsOf(field);

    static bool IndexSupports(DatasourceIndex index, SpecTypeField field) =>
        IsForeignKey(field) && index.Fields.Count > 0 && index.Fields[0] == field.Name;

    static bool PkSupports(SqlTable table, SpecTypeField field)
    {
        var keys = SqlSchema.TableIdentity(table);
        return keys.Count > 0 && keys[0] == field.Name;
    }

    static bool UniqueField(SqlTable table, SpecTypeField field) =>
        SqlSchema.FieldOverlay(table, field.Name)?.IsUnique == true || field.Name == "uuid";

    static string IndexKey(DatasourceIndex index) => JsonSerializer.Serialize(new { index.Name, index.Fields, index.IsUnique });

    static object IdentityOf(SqlTable table) => new { Keys = SqlSchema.TableIdentity(table), Generated = SqlSchema.IsGeneratedIdentity(table) };

    static string RefsOf(SpecTypeField field) => JsonSerializer.Serialize(field.ReferencePair ?? (object?)field.References);
}
