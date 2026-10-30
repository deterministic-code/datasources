using System.Runtime.CompilerServices;
using Datasources.CodeGenerators;

namespace Datasources.Sqlite.Tests;

public static class RegisterSqlGenerate
{
    [ModuleInitializer]
    internal static void Register() => GenerateSql.Register("sqlite", new SqliteDriver());
}
