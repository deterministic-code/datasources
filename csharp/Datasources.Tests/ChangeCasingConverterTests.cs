using Datasources;

namespace Datasources.Tests;

public class ChangeCasingConverterTests
{
    static readonly (Casing Casing, string Text)[] Forms =
    [
        (Casing.Camel, "twoWords"),
        (Casing.Capital, "Two Words"),
        (Casing.Constant, "TWO_WORDS"),
        (Casing.Dot, "two.words"),
        (Casing.Kebab, "two-words"),
        (Casing.No, "two words"),
        (Casing.Pascal, "TwoWords"),
        (Casing.PascalSnake, "Two_Words"),
        (Casing.Path, "two/words"),
        (Casing.Sentence, "Two words"),
        (Casing.Snake, "two_words"),
        (Casing.Train, "Two-Words"),
    ];

    [Fact]
    public void ChangeCasing()
    {
        foreach (var (_, from) in Forms)
        {
            foreach (var (casing, expected) in Forms)
                Assert.Equal(expected, new ChangeCasingConverter(casing).Convert(from));
            Assert.Equal(from, new ChangeCasingConverter(Casing.Snake).ReverseConvert(from));
        }
        Assert.Equal("xmlParser", new ChangeCasingConverter(Casing.Camel).Convert("XMLParser"));
        Assert.Equal("html5", new ChangeCasingConverter(Casing.Camel).Convert("html5"));
        Assert.Equal("v2Name", new ChangeCasingConverter(Casing.Camel).Convert("v2Name"));
        Assert.Equal("", new ChangeCasingConverter(Casing.Snake).Convert(""));
        Assert.Equal("", new ChangeCasingConverter(Casing.Snake).Convert("___"));
        Assert.Equal("two_words", new ChangeCasingConverter(Casing.Snake).Convert(" two  words\t"));
        Assert.Equal("", new ChangeCasingConverter(Casing.Camel).Convert(""));
        Assert.Throws<ArgumentOutOfRangeException>(() => new ChangeCasingConverter((Casing)99).Convert("a"));
    }
}
