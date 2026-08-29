using Datasources;

namespace Datasources.Text;

public sealed class TextDatasource : IDatasource, IDatasourceFactory
{
    public static readonly TextDatasource Instance = new();
    public string Provider => "text";
    public IDatasource Create(object? options = null) => new TextDatasource();
    IConnection? _connection;
    IConnection DefaultConnection() => _connection ??= new TextConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(DefaultConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new TextCommand(connection ?? DefaultConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        new StubRepository<T>(Provider, connection ?? DefaultConnection());
}
