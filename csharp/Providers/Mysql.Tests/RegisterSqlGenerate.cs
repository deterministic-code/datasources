using System.Runtime.CompilerServices;
using Datasources.CodeGenerators;
using Datasources.Mysql;

namespace Datasources.Mysql.Tests;

static class RegisterSqlGenerate
{
    [ModuleInitializer]
    internal static void Register() => SqlDialect.Register("mysql", new MysqlDriver());
}
