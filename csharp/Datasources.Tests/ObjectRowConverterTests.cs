using Datasources;

namespace Datasources.Tests;

public class ObjectConverterTests
{
    sealed class Flag
    {
        public int id { get; set; }
        public bool is_enabled { get; set; }
    }

    sealed class FlagConverter : TypeConverter<Flag, Row>
    {
        public override Flag Convert(Row from) => new()
        {
            id = (int)(long)from["id"].ToObject()!,
            is_enabled = (long)from["is_enabled"].ToObject()! != 0
        };

        public override Row ReverseConvert(Flag to) => new()
        {
            ["id"] = to.id,
            ["is_enabled"] = to.is_enabled
        };
    }

    [Fact]
    public async Task ObjectConverterReadsABoolCell()
    {
        var connection = new MockConnection { Provider = "memory" };
        var command = new MockCommand(connection);
        connection.Command = command;
        command.Reader = new MockReader<Row>([new Row { ["id"] = 1L, ["is_enabled"] = 1L }]);
        var flagsRepo = new ReadonlyRepository<Flag>(connection, new ReadonlyRepositoryMappingOptions<Flag>
        {
            Entity = "flags",
            ReadProperties = ["id", "is_enabled"],
            ObjectConverter = new FlagConverter()
        });
        var flag = await flagsRepo.ReadSingle();
        Assert.Equal(1, flag!.id);
        Assert.True(flag.is_enabled);
    }
}
