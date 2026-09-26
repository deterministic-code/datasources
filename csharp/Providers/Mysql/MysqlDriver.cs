using System.Globalization;
using Datasources;

namespace Datasources.Mysql;

public sealed class MysqlDriver : IDriver
{
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new MysqlConnection(options);
    public ICommand Command(IConnection connection) => connection.CreateCommand();
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.Serializable) =>
        new MysqlTransaction((MysqlConnection)connection, isolation);
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters => MysqlRowTypeConverters.All;

    public Value GeneratedId(Value id) => id switch
    {
        Value.Text(var text) when long.TryParse(text, NumberStyles.Integer, CultureInfo.InvariantCulture, out var n) => new Value.Int(n),
        _ => id
    };
}
