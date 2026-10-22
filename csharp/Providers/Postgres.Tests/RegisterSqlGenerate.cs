using System.Runtime.CompilerServices;
using Datasources.CodeGenerators;
using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

static class RegisterSqlGenerate
{
    [ModuleInitializer]
    internal static void Register() => SqlDialect.Register("postgres", new PostgresDriver());
}
