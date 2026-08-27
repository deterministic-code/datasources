using Datasources;

namespace Datasources.Text;

public sealed class TextConnection : StubConnection
{
    public TextConnection() : base("text") { }
}

public sealed class TextDatasource : IDatasource, IDatasourceFactory
{
    public static readonly TextDatasource Factory = new();
    public string Provider => "text";
    public IDatasource Create(object? options = null) => new TextDatasource();
    IConnection NewConnection() => new TextConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(NewConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new StubCommand(Provider, connection ?? NewConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null, RepositoryOptions? options = null) =>
        new StubRepository<T>(Provider, connection ?? NewConnection());
}
