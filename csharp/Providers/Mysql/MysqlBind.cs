using System.Text.RegularExpressions;

namespace Datasources.Mysql;

public static class MysqlBind
{
    static readonly Regex Name = new(@"\$([A-Za-z_][A-Za-z0-9_]*)", RegexOptions.Compiled);

    public static string WithoutReturning(string sql) =>
        Regex.Replace(sql, @"\s+RETURNING\b[^;]*$", "", RegexOptions.IgnoreCase);

    public static string ToMysql(string sql) => Name.Replace(WithoutReturning(sql), "@$1");

    public static string IsolationSql(IsolationLevel isolation) => isolation switch
    {
        IsolationLevel.ReadUncommitted => "READ UNCOMMITTED",
        IsolationLevel.ReadCommitted => "READ COMMITTED",
        _ => "REPEATABLE READ"
    };
}
