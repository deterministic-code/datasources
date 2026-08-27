using Datasources;

namespace Datasources.Json;

public sealed class JsonTransaction : MiddlewareHost, ITransaction
{
    public JsonTransaction(JsonConnection connection) => Connection = connection;
    public IConnection Connection { get; }
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
