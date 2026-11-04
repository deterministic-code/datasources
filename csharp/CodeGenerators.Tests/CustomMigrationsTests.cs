using Datasources.CodeGenerators;

namespace Datasources.CodeGenerators.Tests;

public class CustomMigrationsTests
{
    [Fact]
    public void CopiesOrder2AfterTheGeneratedSchemaAtThatNumber()
    {
        var entries = CustomMigrations.Of(
            new Dictionary<string, string>
            {
                ["deterministic/custom/sqlite/0002_seed_categories_up.sql"] = "INSERT INTO categories VALUES (1);\n",
                ["deterministic/custom/sqlite/0002_seed_categories_down.sql"] = "DELETE FROM categories WHERE id = 1;\n",
            },
            "sqlite",
            "deterministic");
        var up = entries.Single(entry => entry.Filename.EndsWith("_up.sql")).Filename;
        Assert.Equal("sqlite/migrations/0002_up_custom_002_seed_categories_up.sql", up);
        var names = new[]
        {
            "0001_initial_up.sql",
            "0001_up.sql",
            "0002_stored_procedures_up.sql",
            Path.GetFileName(up),
            "0003_up.sql",
        };
        Assert.Equal(
            new[]
            {
                "0001_initial_up.sql",
                "0001_up.sql",
                "0002_stored_procedures_up.sql",
                "0002_up_custom_002_seed_categories_up.sql",
                "0003_up.sql",
            },
            names.Order(StringComparer.Ordinal));
    }
}
