using Datasources;

namespace ContactsCrudEnrichedMediumRest.Tests;

public sealed class SchemaTests
{
    [Theory]
    [InlineData("sqlite")]
    [InlineData("mysql")]
    [InlineData("postgres")]
    public void CreatesPhysicalTablesWithForeignKeys(string provider)
    {
        var sql = string.Join("\n", Schema.Tables(provider).Select(table => table.GetText()));
        Assert.Contains("tbl_Contacts", sql);
        Assert.Contains("m_FirstName", sql);
        Assert.Contains($"FOREIGN KEY ({Schema.Ident(provider, "ContactSourceId")}) REFERENCES", sql);
        Assert.Contains($"FOREIGN KEY ({Schema.Ident(provider, "m_ContactTypeId")}) REFERENCES", sql);
        Assert.Contains($"FOREIGN KEY ({Schema.Ident(provider, "m_AddressTypeId")}) REFERENCES", sql);
        Assert.Contains($"FOREIGN KEY ({Schema.Ident(provider, "PhoneTypeId")}) REFERENCES", sql);
        Assert.Contains($"UNIQUE ({Schema.Ident(provider, "ContactId")}, {Schema.Ident(provider, "ContactGroupId")})", sql);
        Assert.DoesNotContain("first_name", sql);
    }

    [Fact]
    public void SqliteUsesIntegerPrimaryKey()
    {
        var source = Schema.Tables("sqlite")[0];
        Assert.True(source.Columns[0].Identity);
        Assert.Contains("CONSTRAINT pk_ContactSource PRIMARY KEY", source.GetText());
    }

    [Fact]
    public void MysqlUsesIdentityFlag() =>
        Assert.True(Schema.Tables("mysql")[0].Columns[0].Identity);

    [Fact]
    public void PostgresUsesIdentityFlag() =>
        Assert.True(Schema.Tables("postgres")[0].Columns[0].Identity);

    [Fact]
    public void LegacyKeyIsThePrimaryKey()
    {
        var sqlite = Schema.Tables("sqlite").Single(table => table.GetText().Contains("legacy_contact")).GetText();
        var mysql = Schema.Tables("mysql").Single(table => table.GetText().Contains("legacy_contact")).GetText();
        Assert.Contains("PRIMARY KEY (\"m_Key\")", sqlite);
        Assert.Contains("PRIMARY KEY (`m_Key`)", mysql);
    }

    [Fact]
    public void QuotesIdentifiers()
    {
        Assert.Equal("\"tbl_Contacts\"", Schema.Ident("sqlite", "tbl_Contacts"));
        Assert.Equal("\"a\"\"b\"", Schema.Ident("postgres", "a\"b"));
        Assert.Equal("`a``b`", Schema.Ident("mysql", "a`b"));
    }

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
