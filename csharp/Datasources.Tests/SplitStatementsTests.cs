namespace Datasources.Tests;

public class SplitStatementsTests
{
    [Fact]
    public void SplitsOnSemicolonsOutsideQuotes()
    {
        Assert.Equal(["SELECT 1"], SplitStatements.Split("SELECT 1"));
        Assert.Equal(["SELECT 1 AS id", "SELECT 2 AS id"], SplitStatements.Split("SELECT 1 AS id; SELECT 2 AS id"));
        Assert.Equal(["SELECT ';' AS id", "SELECT 2 AS id"], SplitStatements.Split("SELECT ';' AS id; SELECT 2 AS id"));
    }

    [Fact]
    public void KeepsProcedureBodies()
    {
        var mysql = "CREATE PROCEDURE add_user()\nBEGIN\n  INSERT INTO t VALUES (1);\nEND";
        Assert.Equal([mysql], SplitStatements.Split(mysql));
        var postgres = "CREATE PROCEDURE bulk_create_user(payload jsonb)\nLANGUAGE plpgsql\nAS $$\nBEGIN\n  INSERT INTO t VALUES (1);\nEND;\n$$";
        Assert.Equal([postgres], SplitStatements.Split(postgres));
    }

    [Fact]
    public void ParameterTokenUsesAWordBoundary()
    {
        Assert.True(SplitStatements.UsesParameter("INSERT INTO t (is_enabled) VALUES ($is_enabled__10)", "is_enabled__10"));
        Assert.False(SplitStatements.UsesParameter("INSERT INTO t (is_enabled) VALUES ($is_enabled__10)", "is_enabled__1"));
        Assert.True(SplitStatements.UsesParameter("$is_enabled__1", "is_enabled__1"));
    }
}
