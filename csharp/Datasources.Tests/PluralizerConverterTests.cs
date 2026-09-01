using Datasources;

namespace Datasources.Tests;

public class PluralizerConverterTests
{
    [Fact]
    public void PluralizesAndSingularizes()
    {
        var pairs = new (string Singular, string Plural)[]
        {
            ("person", "people"),
            ("child", "children"),
            ("man", "men"),
            ("woman", "women"),
            ("mouse", "mice"),
            ("goose", "geese"),
            ("tooth", "teeth"),
            ("foot", "feet"),
            ("ox", "oxen"),
            ("sheep", "sheep"),
            ("fish", "fish"),
            ("series", "series"),
            ("species", "species"),
            ("deer", "deer"),
            ("news", "news"),
            ("box", "boxes"),
            ("dish", "dishes"),
            ("church", "churches"),
            ("quiz", "quizes"),
            ("kiss", "kisses"),
            ("bus", "buses"),
            ("city", "cities"),
            ("cat", "cats"),
            ("toy", "toys"),
            ("2y", "2ys"),
            ("Person", "People"),
            ("", ""),
        };
        var converter = new PluralizerConverter();
        foreach (var (singular, plural) in pairs)
        {
            Assert.Equal(plural, converter.Convert(singular));
            Assert.Equal(plural, converter.Convert(plural));
            Assert.Equal(singular, converter.ReverseConvert(plural));
            Assert.Equal(singular, converter.ReverseConvert(singular));
        }
    }
}
