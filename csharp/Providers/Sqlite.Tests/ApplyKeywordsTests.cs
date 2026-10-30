namespace Datasources.Sqlite.Tests;

public class ApplyKeywordsTests
{
    static string Lower(string word) => word.ToLowerInvariant();

    [Fact]
    public void SkipsDoubleQuotedIdentifiers()
    {
        Assert.Equal(
            "create table \"CREATE\" (\"TABLE\")",
            SqliteDriver.ApplyKeywords("CREATE TABLE \"CREATE\" (\"TABLE\")", Lower));
        Assert.Equal(
            "select \"a\"\"CREATE\"\"b\" from t",
            SqliteDriver.ApplyKeywords("SELECT \"a\"\"CREATE\"\"b\" FROM t", Lower));
    }

    [Fact]
    public void SkipsStringsAndComments()
    {
        Assert.Equal(
            "insert into t values ('CREATE', 'NOT NULL', 'it''s CREATE')",
            SqliteDriver.ApplyKeywords("INSERT INTO t VALUES ('CREATE', 'NOT NULL', 'it''s CREATE')", Lower));
        Assert.Equal(
            "-- CREATE TABLE stays\ndrop table t;",
            SqliteDriver.ApplyKeywords("-- CREATE TABLE stays\nDROP TABLE t;", Lower));
        Assert.Equal(
            "select /* CREATE TABLE */ 1 from t",
            SqliteDriver.ApplyKeywords("SELECT /* CREATE TABLE */ 1 FROM t", Lower));
    }

    [Fact]
    public void DoesNotCaseKeywordPrefixes() =>
        Assert.Equal("CREATED NOTES ONZONE", SqliteDriver.ApplyKeywords("CREATED NOTES ONZONE", Lower));
}
