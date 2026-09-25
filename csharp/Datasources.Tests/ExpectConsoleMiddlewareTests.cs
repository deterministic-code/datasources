using Datasources;

namespace Datasources.Tests;

public sealed class ExpectConsoleMiddlewareTests
{
    static readonly DateTimeOffset Stamp = new(2026, 8, 28, 1, 54, 12, 101, TimeSpan.Zero);

    [Fact]
    public async Task MatchesTheTranscriptAndDropsFinishDuration()
    {
        var middleware = new ExpectConsoleMiddleware(
            [
                "[2026-08-28T01:54:12.101Z]-[datasource][Start]-[SELECT 1]",
                "[2026-08-28T01:54:12.101Z]-[datasource][Request]-[{id: 1}]",
                "[2026-08-28T01:54:12.101Z]-[datasource][Finish]-[SELECT 1]",
                "[2026-08-28T01:54:12.101Z]-[datasource][AfterCommand]-[{id: 1}]"
            ],
            () => Stamp);
        var host = new RecordingCommand();
        host.AddMiddleware(middleware);
        await host.Run("SELECT 1", [new Parameter("id", 1L)], (_, _) => Task.FromResult((1, Rows(new Row { ["id"] = 1L }))));
        middleware.Complete();
        Assert.Equal("[2026-08-28T01:54:12.101Z]-[datasource][Finish]-[SELECT 1]", ExpectConsoleMiddleware.Signature("[2026-08-28T01:54:12.101Z]-[datasource][Finish]-[SELECT 1] 2ms"));
        Assert.Equal("[2026-08-28T01:54:12.101Z]-[datasource][Start]-[SELECT 1]", ExpectConsoleMiddleware.Signature("[2026-08-28T01:54:12.101Z]-[datasource][Start]-[SELECT 1]"));
    }

    [Fact]
    public async Task RejectsADifferentLine()
    {
        var middleware = new ExpectConsoleMiddleware(["[nope]"], () => Stamp);
        var host = new RecordingCommand();
        host.AddMiddleware(middleware);
        var error = await Assert.ThrowsAsync<InvalidOperationException>(() =>
            host.Run("SELECT 1", [new Parameter("id", 1L)], (_, _) => Task.FromResult((1, Rows(new Row { ["id"] = 1L })))));
        Assert.Contains("console mismatch at 0", error.Message);
        Assert.Contains("actual:", error.Message);
    }

    [Fact]
    public void RejectsAShortTranscript()
    {
        var middleware = new ExpectConsoleMiddleware(["[missing]"], () => Stamp);
        var error = Assert.Throws<InvalidOperationException>(middleware.Complete);
        Assert.Contains("actual:   <missing>", error.Message);
    }

    static IReadOnlyList<Row> Rows(params Row[] rows) => rows;

    sealed class RecordingCommand : CommandMiddlewareHost, ICommand
    {
        public IConnection Connection { get; } = new MockConnection();
        public Task<IReader<Row>> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null) =>
            throw new NotSupportedException();
        public Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null) =>
            throw new NotSupportedException();
    }
}
