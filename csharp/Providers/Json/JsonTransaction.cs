using Datasources;

namespace Datasources.Json;

public sealed class JsonTransaction : MiddlewareHost, ITransaction
{
    public JsonTransaction(JsonConnection connection, IsolationLevel isolation = IsolationLevel.Serializable)
    {
        Connection = connection;
        Isolation = isolation;
    }
    public IConnection Connection { get; }
    public IsolationLevel Isolation { get; }
    private JsonConnection Typed => (JsonConnection)Connection;

    public Task Commit() => Run("commit", async () =>
    {
        Typed.FinishCommit();
        if (!string.IsNullOrEmpty(Typed.Path))
            await Typed.FileStore.Write(Typed.Path, Typed.Data.ToJsonString());
    });

    public Task Abort() => Run("abort", () =>
    {
        Typed.FinishAbort();
        return Task.CompletedTask;
    });
}
