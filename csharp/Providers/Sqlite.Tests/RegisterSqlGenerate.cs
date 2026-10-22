using System.Runtime.CompilerServices;
using Datasources.CodeGenerators;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

static class RegisterSqlGenerate
{
    [ModuleInitializer]
    internal static void Register() => SqlDialect.Register("sqlite", new SqliteDriver());
}
