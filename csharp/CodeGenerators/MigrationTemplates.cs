using System.Text;

namespace Datasources.CodeGenerators;

public static class MigrationTemplates
{
    public static string Up(string dialect, IReadOnlyList<(string CreateTable, string Indexes, string Trigger)> tables, IReadOnlyList<string> seeds)
    {
        var text = new StringBuilder();
        text.Append("-- Generated initial migration for ").Append(dialect).Append("\n\n");
        text.Append("-- Create added tables\n");
        foreach (var table in tables)
        {
            text.Append(table.CreateTable);
            if (table.Indexes.Length > 0) text.Append('\n').Append(table.Indexes);
            if (table.Trigger.Length > 0) text.Append('\n').Append(table.Trigger);
            text.Append("\n\n");
        }
        if (seeds.Count > 0)
        {
            text.Append("-- Seed data\n");
            foreach (var seed in seeds) text.Append(seed);
        }
        return text.ToString();
    }

    public static string Down(string dialect, IReadOnlyList<string> drops)
    {
        var text = new StringBuilder();
        text.Append("-- Rollback initial migration for ").Append(dialect).Append("\n\n");
        text.Append("-- DROP tables (reverse dependency order)\n");
        foreach (var drop in drops) text.Append(drop).Append('\n');
        return text.ToString();
    }
}
