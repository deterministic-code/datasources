using System.Text.Json;
using System.Text.Json.Serialization;

namespace ContactsCrudEnrichedMediumRest;

static class JsonObject
{
    static readonly JsonSerializerOptions Options = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
        PropertyNameCaseInsensitive = true,
        DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull,
    };

    public static string ToString<T>(T value) => JsonSerializer.Serialize(value, Options);

    public static T FromString<T>(string? body) where T : new()
    {
        if (string.IsNullOrWhiteSpace(body)) return new T();
        return JsonSerializer.Deserialize<T>(body, Options) ?? throw new JsonException("invalid json");
    }
}
