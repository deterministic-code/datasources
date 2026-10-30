using Datasources.CodeGenerators;

namespace Datasources.Mysql.Tests;

public class RegisterSqlGenerate
{
    internal static void Register() => GenerateSql.Register("mysql", new MysqlDriver());
}
