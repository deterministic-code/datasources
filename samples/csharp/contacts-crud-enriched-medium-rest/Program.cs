using System.Net;
using System.Text;
using ContactsCrudEnrichedMediumRest;
using Datasources;
using Datasources.Mysql;
using Datasources.Postgres;
using Datasources.Sqlite;

var url = Env("DATASOURCE_URL") ?? throw new InvalidOperationException("DATASOURCE_URL is required");
var schema = Env("DATABASE_SCHEMA");
var provider = ProviderFromUrl.From(url) ?? throw new InvalidOperationException("unknown provider");
var connection = provider switch
{
    "sqlite" => new SqliteDriver().Connection(new Dictionary<string, string?> { ["url"] = url }),
    "mysql" => new MysqlDriver().Connection(new Dictionary<string, string?> { ["url"] = url }),
    "postgres" => new PostgresDriver().Connection(new Dictionary<string, string?> { ["url"] = url, ["schema"] = schema }),
    _ => throw new InvalidOperationException($"unknown provider {provider}")
};
await connection.Open();
await Migrate.Apply(connection.CreateCommand());
var api = new Api(new ContactsRepository(connection), new LegacyContactRepository(connection));
var port = int.TryParse(Env("PORT"), out var parsed) ? parsed : 8080;
using var listener = new HttpListener();
listener.Prefixes.Add($"http://127.0.0.1:{port}/");
listener.Start();
while (listener.IsListening)
{
    var context = await listener.GetContextAsync();
    using var reader = new StreamReader(context.Request.InputStream, context.Request.ContentEncoding);
    var result = await api.Handle(context.Request.HttpMethod, context.Request.Url?.PathAndQuery ?? "/", await reader.ReadToEndAsync());
    var bytes = Encoding.UTF8.GetBytes(result.Body);
    context.Response.StatusCode = result.Status;
    context.Response.ContentType = "application/json";
    await context.Response.OutputStream.WriteAsync(bytes);
    context.Response.Close();
}

static string? Env(string key)
{
    var value = Environment.GetEnvironmentVariable(key);
    if (!string.IsNullOrWhiteSpace(value))
        return value.Trim();
    if (!File.Exists(".env"))
        return null;
    foreach (var line in File.ReadAllLines(".env"))
    {
        var i = line.IndexOf('=');
        if (i > 0 && line[..i].Trim() == key)
        {
            var raw = line[(i + 1)..].Trim();
            return string.IsNullOrWhiteSpace(raw) ? null : raw;
        }
    }
    return null;
}
