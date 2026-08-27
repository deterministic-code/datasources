using Datasources;

namespace Datasources.Text;

public sealed class TextDatasource : IDatasource, IDatasourceFactory
{
    public static readonly TextDatasource Factory = new();
    public string Provider => "text";
    public IDatasource Create(object? options = null) => new TextDatasource();
    IConnection NewConnection() => new TextConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(NewConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new TextCommand(connection ?? NewConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        new StubRepository<T>(Provider, connection ?? NewConnection());
}
