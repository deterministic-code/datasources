using Datasources;
using Datasources.Sqlite;
using Datasources.SqlSpecs;

namespace Datasources.Sqlite.Tests;

[Collection("DatasourceFactory")]
public class ReadPipelineIntegrationTests
{
    public ReadPipelineIntegrationTests()
    {
        DatasourceFactory.Reset();
        Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "sqlite:///:memory:");
        Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", "sqlite");
    }

    public class User
    {
        public long Id { get; set; }
        public Guid Uuid { get; set; }
        public string Username { get; set; } = "";
        public bool IsEnabled { get; set; }
        public DateTime Created { get; set; }
    }

    [Fact]
    public async Task MapsAndConvertsUsersThroughTheReadPipeline()
    {
        var ada = Guid.Parse("11111111-1111-1111-1111-111111111111");
        var grace = Guid.Parse("22222222-2222-2222-2222-222222222222");
        var lin = Guid.Parse("33333333-3333-3333-3333-333333333333");
        var ds = Datasource.Default;
        var command = await ds.CreateCommand();
        await SchemaSpec.Apply(command.Connection, "pipeline-users");
        await command.Execute(
            "INSERT INTO users (id, uuid, name, is_enabled, created) VALUES ($id, $uuid, $name, $is_enabled, $created)",
            [
                new Parameter("id", 1),
                new Parameter("uuid", ada.ToString()),
                new Parameter("name", "ada"),
                new Parameter("is_enabled", 1),
                new Parameter("created", "2001-01-01T00:00:00.000Z"),
            ]);
        await command.Execute(
            "INSERT INTO users (id, uuid, name, is_enabled, created) VALUES ($id, $uuid, $name, $is_enabled, $created)",
            [
                new Parameter("id", 2),
                new Parameter("uuid", grace.ToString()),
                new Parameter("name", "grace"),
                new Parameter("is_enabled", 1),
                new Parameter("created", "2002-02-02T00:00:00.000Z"),
            ]);
        await command.Execute(
            "INSERT INTO users (id, uuid, name, is_enabled, created) VALUES ($id, $uuid, $name, $is_enabled, $created)",
            [
                new Parameter("id", 3),
                new Parameter("uuid", lin.ToString()),
                new Parameter("name", "lin"),
                new Parameter("is_enabled", 0),
                new Parameter("created", "2003-03-03T00:00:00.000Z"),
            ]);

        var propertyToColumnMapper = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Uuid"] = "uuid",
            ["Username"] = "name",
            ["IsEnabled"] = "is_enabled",
            ["Created"] = "created",
        });

        await using var reader = await command.ExecuteReader(
            "SELECT id, uuid, name, is_enabled, created FROM users");
        var unmapped = await reader.Read().ToListAsync();

        var columnMappedRows = unmapped
            .Select(row => new Row(row.Select(cell =>
                KeyValuePair.Create(propertyToColumnMapper.ReverseMap(cell.Key), cell.Value))))
            .ToList();

        var rowConverter = RowTypeConverter.From(new SqliteDriver().RowTypeConverters, new Dictionary<string, string>
        {
            ["Uuid"] = "uuid",
            ["IsEnabled"] = "boolean",
            ["Created"] = "datetime",
        });
        var convertedRows = columnMappedRows.Select(rowConverter.Convert).ToList();

        var objectConverter = new MetaDataRowTypeConverter<User>();
        var finalEntities = convertedRows.Select(objectConverter.Convert).ToList();

        var finalManualEntities = convertedRows.Select(row => new User
        {
            Id = (long)row["Id"].ToObject()!,
            Uuid = (Guid)row["Uuid"].ToObject()!,
            Username = (string)row["Username"].ToObject()!,
            IsEnabled = (bool)row["IsEnabled"].ToObject()!,
            Created = (DateTime)row["Created"].ToObject()!,
        }).ToList();

        Assert.Equal(3, unmapped.Count);
        Assert.Equal(new Value.Int(1), unmapped[0]["id"]);
        Assert.Equal(new Value.Text(ada.ToString()), unmapped[0]["uuid"]);
        Assert.Equal(new Value.Text("ada"), unmapped[0]["name"]);
        Assert.True(unmapped[0]["is_enabled"] is Value.Bool or Value.Int);
        Assert.True(unmapped[0]["created"] is Value.Text);

        Assert.Equal(new Value.Int(1), columnMappedRows[0]["Id"]);
        Assert.Equal(new Value.Text("ada"), columnMappedRows[0]["Username"]);
        Assert.Equal(new Value.Guid(ada), convertedRows[0]["Uuid"]);
        Assert.Equal(new Value.Bool(true), convertedRows[0]["IsEnabled"]);
        Assert.True(convertedRows[0]["Created"] is Value.DateTime);

        Assert.Equal(1L, finalEntities[0].Id);
        Assert.Equal(ada, finalEntities[0].Uuid);
        Assert.Equal("ada", finalEntities[0].Username);
        Assert.True(finalEntities[0].IsEnabled);
        Assert.Equal(2001, finalEntities[0].Created.Year);

        Assert.Equal(ada, finalManualEntities[0].Uuid);
        Assert.Equal("grace", finalManualEntities[1].Username);
        Assert.False(finalManualEntities[2].IsEnabled);
    }
}
