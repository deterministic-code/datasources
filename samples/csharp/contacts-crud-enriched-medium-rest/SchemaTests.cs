using Datasources;
using Datasources.Mysql;
using Datasources.Postgres;
using Datasources.Sqlite;

namespace ContactsCrudEnrichedMediumRest.Tests;

public sealed class SchemaTests
{
    [Theory]
    [InlineData("sqlite")]
    [InlineData("mysql")]
    [InlineData("postgres")]
    public void CreatesPhysicalTablesWithForeignKeys(string provider)
    {
        var sql = string.Join("\n", Schema.Tables(provider, Driver(provider)).Select(table => table.GetText()));
        Assert.Contains("tbl_Contacts", sql);
        Assert.Contains("m_FirstName", sql);
        Assert.Contains("FOREIGN KEY (ContactSourceId) REFERENCES", sql);
        Assert.Contains("FOREIGN KEY (m_ContactTypeId) REFERENCES", sql);
        Assert.Contains("FOREIGN KEY (m_AddressTypeId) REFERENCES", sql);
        Assert.Contains("FOREIGN KEY (PhoneTypeId) REFERENCES", sql);
        Assert.Contains("UNIQUE (ContactId, ContactGroupId)", sql);
        Assert.DoesNotContain("first_name", sql);
    }

    [Fact]
    public void SqliteUsesIntegerPrimaryKey() =>
        Assert.Contains("Id INTEGER PRIMARY KEY", Schema.Tables("sqlite", new SqliteDriver())[0].GetText());

    [Fact]
    public void MysqlUsesIdentityFlag() =>
        Assert.True(Schema.Tables("mysql", new MysqlDriver())[0].Columns[0].Identity);

    [Fact]
    public void PostgresUsesIdentityFlag() =>
        Assert.True(Schema.Tables("postgres", new PostgresDriver())[0].Columns[0].Identity);

    [Fact]
    public void LegacyKeyIsThePrimaryKey()
    {
        var sqlite = Schema.Tables("sqlite", new SqliteDriver()).Single(table => table.GetText().Contains("legacy_contact")).GetText();
        var mysql = Schema.Tables("mysql", new MysqlDriver()).Single(table => table.GetText().Contains("legacy_contact")).GetText();
        Assert.Contains("m_Key TEXT PRIMARY KEY", sqlite);
        Assert.Contains("PRIMARY KEY (m_Key)", mysql);
    }

    [Fact]
    public void QuotesIdentifiersAndRejectsUnknownProviders()
    {
        Assert.Equal("\"tbl_Contacts\"", Schema.Ident("sqlite", "tbl_Contacts"));
        Assert.Equal("\"a\"\"b\"", Schema.Ident("postgres", "a\"b"));
        Assert.Equal("`a``b`", Schema.Ident("mysql", "a`b"));
        Assert.Throws<InvalidOperationException>(() => Schema.Tables("oracle", new SqliteDriver()));
    }

    static IDriver Driver(string provider) => provider switch
    {
        "sqlite" => new SqliteDriver(),
        "mysql" => new MysqlDriver(),
        "postgres" => new PostgresDriver(),
        _ => throw new InvalidOperationException($"unknown provider {provider}")
    };

    [Fact]
    public void MapsPhysicalNamesInBothDirections()
    {
        Assert.Equal("m_FirstName", Names.Column("first_name"));
        Assert.Equal("tbl_Contacts", Names.Table("contacts"));
        Assert.Equal("first_name", Names.Columns.ReverseMap("m_FirstName"));
        Assert.Equal("contacts", Names.Tables.ReverseMap("tbl_Contacts"));
        Assert.Equal("zzz", Names.Column("zzz"));
        Assert.Equal("zzz", Names.Table("zzz"));
    }
}
