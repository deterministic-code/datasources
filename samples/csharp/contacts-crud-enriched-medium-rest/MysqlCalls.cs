using System.Text.RegularExpressions;
using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public static class MysqlCalls
{
    public static IMapper<IInsertStatement, IInsertStatement> Insert(string procedure) =>
        new TransformMapper<IInsertStatement, IInsertStatement>(
            statement => new InsertStatement(Call(procedure, Placeholders(statement.GetText()))),
            static statement => statement);

    public static IMapper<IUpdateStatement, IUpdateStatement> Update(string procedure) =>
        new TransformMapper<IUpdateStatement, IUpdateStatement>(
            statement => new UpdateStatement(Call(procedure, Placeholders(statement.GetText())), statement.Where),
            static statement => statement);

    public static IMapper<IDeleteStatement, IDeleteStatement> Delete(string procedure) =>
        new TransformMapper<IDeleteStatement, IDeleteStatement>(
            statement =>
            {
                var delete = (DeleteStatement)statement;
                return DeleteStatement.Raw(Call(procedure, delete.Where?.Parameters.Select(parameter => parameter.Name) ?? []), delete.Where);
            },
            static statement => statement);

    public static string Call(string procedure, IEnumerable<string> names) =>
        $"CALL {procedure}({string.Join(", ", names.Select(name => $"${name}"))})";

    static IEnumerable<string> Placeholders(string sql) =>
        Regex.Matches(sql, @"\$([A-Za-z_][A-Za-z0-9_]*)").Select(match => match.Groups[1].Value);
}
