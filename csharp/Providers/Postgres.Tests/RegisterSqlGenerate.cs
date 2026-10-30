using Datasources.CodeGenerators;

namespace Datasources.Postgres.Tests;

public class RegisterSqlGenerate
{
    internal static void Register() => GenerateSql.Register("postgres", new PostgresDriver());
}
