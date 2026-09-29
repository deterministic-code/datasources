using Datasources;

namespace Datasources.Json;

internal sealed class JsonTransaction : ITransaction
{
    public JsonTransaction(JsonConnection connection, IsolationLevel isolation = IsolationLevel.ReadCommitted)
    {
        Connection = connection;
        Isolation = isolation;
    }
    public IConnection Connection { get; }
    public IsolationLevel Isolation { get; }
    private JsonConnection Typed => (JsonConnection)Connection;

    public async Task Commit()
    {
        Typed.FinishCommit();
        if (!string.IsNullOrEmpty(Typed.Path))
            await Typed.FileStore.Write(Typed.Path, Typed.Data.ToJsonString());
    }

    public Task Abort()
    {
        Typed.FinishAbort();
        return Task.CompletedTask;
    }
}
