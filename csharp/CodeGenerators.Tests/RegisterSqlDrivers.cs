using System.Runtime.CompilerServices;
using Datasources.CodeGenerators;
using Datasources.Mysql;
using Datasources.Oracle;
using Datasources.Postgres;
using Datasources.Sqlite;
using Datasources.SqlServer;

namespace Datasources.CodeGenerators.Tests;

static class RegisterSqlDrivers
{
    [ModuleInitializer]
    internal static void Register()
    {
        SqlDialect.Register("sqlite", new SqliteDriver());
        SqlDialect.Register("mysql", new MysqlDriver());
        SqlDialect.Register("postgres", new PostgresDriver());
        SqlDialect.Register("sqlserver", new SqlServerDriver());
        SqlDialect.Register("oracle", new OracleDriver());
    }
}
