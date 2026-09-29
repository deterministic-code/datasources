namespace Datasources.Tests;

public class DatasourceUrlParserTests
{
    [Fact]
    public void PostgresUrl()
    {
        var url = DatasourceUrlParser.Parse("postgres://ds:ds@127.0.0.1:5432/datasources?schema=public&sslmode=require");
        Assert.NotNull(url);
        Assert.Equal("postgres", url.Provider);
        Assert.Equal("ds", url.Username);
        Assert.Equal("ds", url.Password);
        Assert.Equal("127.0.0.1", url.Host);
        Assert.Equal(5432, url.Port);
        Assert.Equal("/datasources", url.Path);
        Assert.Equal("datasources", url.Datasource);
        Assert.Equal("public", url.Schema);
        Assert.Equal("require", url.Options["sslmode"]);
    }

    [Fact]
    public void PostgresUserWithoutPassword()
    {
        var url = DatasourceUrlParser.Parse("postgres://ds@127.0.0.1/datasources");
        Assert.NotNull(url);
        Assert.Equal("ds", url.Username);
        Assert.Null(url.Password);
        Assert.Null(url.Port);
        Assert.Null(url.Schema);
        Assert.Empty(url.Options);
    }

    [Fact]
    public void SchemaOnly()
    {
        var url = DatasourceUrlParser.Parse("postgres://127.0.0.1/datasources?schema=public");
        Assert.NotNull(url);
        Assert.Equal("public", url.Schema);
        Assert.Empty(url.Options);
    }

    [Fact]
    public void OptionsOnly()
    {
        var url = DatasourceUrlParser.Parse("postgres://127.0.0.1/datasources?sslmode=require");
        Assert.NotNull(url);
        Assert.Null(url.Schema);
        Assert.Equal("require", url.Options["sslmode"]);
    }

    [Fact]
    public void RepeatedOptionKeepsTheLastValue()
    {
        var url = DatasourceUrlParser.Parse("postgres://127.0.0.1/datasources?sslmode=require&sslmode=disable");
        Assert.NotNull(url);
        Assert.Equal("disable", url.Options["sslmode"]);
    }

    [Fact]
    public void FlagOptionIsEmpty()
    {
        var url = DatasourceUrlParser.Parse("postgres://127.0.0.1/datasources?readonly");
        Assert.NotNull(url);
        Assert.Equal("", url.Options["readonly"]);
    }

    [Fact]
    public void MysqlUrl()
    {
        var url = DatasourceUrlParser.Parse("mysql://ds:secret@127.0.0.1:3306/app");
        Assert.NotNull(url);
        Assert.Equal("mysql", url.Provider);
        Assert.Equal("ds", url.Username);
        Assert.Equal("secret", url.Password);
        Assert.Equal("127.0.0.1", url.Host);
        Assert.Equal(3306, url.Port);
        Assert.Equal("app", url.Datasource);
    }

    [Fact]
    public void MysqlHostAndDatasource()
    {
        var url = DatasourceUrlParser.Parse("mysql://db/app");
        Assert.NotNull(url);
        Assert.Equal("db", url.Host);
        Assert.Equal("app", url.Datasource);
        Assert.Null(url.Username);
        Assert.Null(url.Port);
    }

    [Fact]
    public void SqliteRelativeFile()
    {
        var url = DatasourceUrlParser.Parse("sqlite:///app.db");
        Assert.NotNull(url);
        Assert.Equal("sqlite", url.Provider);
        Assert.Equal("/app.db", url.Path);
        Assert.Equal("app.db", url.Datasource);
        Assert.Null(url.Host);
    }

    [Fact]
    public void SqliteAbsoluteFile()
    {
        var url = DatasourceUrlParser.Parse("sqlite:////tmp/app.db");
        Assert.NotNull(url);
        Assert.Equal("//tmp/app.db", url.Path);
        Assert.Equal("/tmp/app.db", url.Datasource);
    }

    [Fact]
    public void SqliteMemory()
    {
        var url = DatasourceUrlParser.Parse("sqlite:///:memory:");
        Assert.NotNull(url);
        Assert.Equal("sqlite", url.Provider);
        Assert.Equal("/:memory:", url.Path);
        Assert.Equal(":memory:", url.Datasource);
    }

    [Fact]
    public void SqliteMemoryWithoutPath()
    {
        var url = DatasourceUrlParser.Parse("sqlite://:memory:");
        Assert.NotNull(url);
        Assert.Equal(":memory:", url.Host);
        Assert.Null(url.Datasource);
    }

    [Fact]
    public void JsonFile()
    {
        var url = DatasourceUrlParser.Parse("json://contacts.json");
        Assert.NotNull(url);
        Assert.Equal("json", url.Provider);
        Assert.Equal("contacts.json", url.Datasource);
        Assert.Null(url.Host);
    }

    [Fact]
    public void PercentEncodedPassword()
    {
        var url = DatasourceUrlParser.Parse("postgres://ds:p%3Ax@127.0.0.1/datasources");
        Assert.NotNull(url);
        Assert.Equal("p:x", url.Password);
    }

    [Fact]
    public void PasswordContainsColon()
    {
        var url = DatasourceUrlParser.Parse("postgres://ds:a:b@127.0.0.1/datasources");
        Assert.NotNull(url);
        Assert.Equal("ds", url.Username);
        Assert.Equal("a:b", url.Password);
    }

    [Fact]
    public void Edges()
    {
        var upper = DatasourceUrlParser.Parse("Postgres://127.0.0.1/db");
        Assert.NotNull(upper);
        Assert.Equal("postgres", upper.Provider);
        var opaque = DatasourceUrlParser.Parse("a.b+c1:file");
        Assert.NotNull(opaque);
        Assert.Equal("a.b+c1", opaque.Provider);
        Assert.Equal("file", opaque.Datasource);
        var slash = DatasourceUrlParser.Parse("postgres://127.0.0.1/");
        Assert.NotNull(slash);
        Assert.Equal("/", slash.Path);
        Assert.Null(slash.Datasource);
        var ipv6 = DatasourceUrlParser.Parse("postgres://[::1]:5432/db");
        Assert.NotNull(ipv6);
        Assert.Equal("::1", ipv6.Host);
        Assert.Equal(5432, ipv6.Port);
        var hostOnly = DatasourceUrlParser.Parse("postgres://[::1]/db");
        Assert.NotNull(hostOnly);
        Assert.Equal("::1", hostOnly.Host);
        Assert.Null(hostOnly.Port);
        var noHost = DatasourceUrlParser.Parse("postgres://ds@/db");
        Assert.NotNull(noHost);
        Assert.Null(noHost.Host);
        var portOnly = DatasourceUrlParser.Parse("postgres://:5432/db");
        Assert.NotNull(portOnly);
        Assert.Null(portOnly.Host);
        Assert.Equal(5432, portOnly.Port);
        var passwordOnly = DatasourceUrlParser.Parse("postgres://:secret@h/db");
        Assert.NotNull(passwordOnly);
        Assert.Null(passwordOnly.Username);
        Assert.Equal("secret", passwordOnly.Password);
        var fragment = DatasourceUrlParser.Parse("postgres://h/db#frag");
        Assert.NotNull(fragment);
        Assert.Equal("db", fragment.Datasource);
        var accented = DatasourceUrlParser.Parse("postgres://dé@h/db");
        Assert.NotNull(accented);
        Assert.Equal("dé", accented.Username);
        var encoded = DatasourceUrlParser.Parse("postgres://d%C3%A9@h/db");
        Assert.NotNull(encoded);
        Assert.Equal("dé", encoded.Username);
        Assert.Null(DatasourceUrlParser.Parse("   "));
        Assert.Null(DatasourceUrlParser.Parse("9abc:file"));
        Assert.Null(DatasourceUrlParser.Parse("postgres://ds:a%zz@h/db"));
        Assert.Null(DatasourceUrlParser.Parse("postgres://[/db"));
        Assert.Null(DatasourceUrlParser.Parse("postgres://[::1]x/db"));
        Assert.Null(DatasourceUrlParser.Parse("postgres://[::1]:no/db"));
        Assert.Null(DatasourceUrlParser.Parse("postgres://[%zz]/db"));
        var emptyUser = DatasourceUrlParser.Parse("postgres://@h/db");
        Assert.NotNull(emptyUser);
        Assert.Null(emptyUser.Username);
        Assert.Equal("h", emptyUser.Host);
        Assert.Null(DatasourceUrlParser.Parse("postgres://ds:a%@h/db"));
        Assert.Null(DatasourceUrlParser.Parse("postgres://%zz@h/db"));
        Assert.Null(DatasourceUrlParser.Parse("postgres://%zz:5432/db"));
        Assert.Null(DatasourceUrlParser.Parse("json://%zz"));
        Assert.Null(DatasourceUrlParser.Parse("postgres://%zz/db"));
        Assert.Null(DatasourceUrlParser.Parse("postgres://h/db?sslmode=%zz"));
        var query = DatasourceUrlParser.Parse("postgres://h/db?&sslmode=on");
        Assert.NotNull(query);
        Assert.Equal("on", query.Options["sslmode"]);
    }

    [Fact]
    public void NotAUrl()
    {
        var blank = DatasourceUrlParser.Parse("");
        var text = DatasourceUrlParser.Parse("not-a-url");
        var connectionString = DatasourceUrlParser.Parse("Host=db;Port=1");
        Assert.Null(blank);
        Assert.Null(text);
        Assert.Null(connectionString);
    }
}
