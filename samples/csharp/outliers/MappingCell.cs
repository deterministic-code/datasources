namespace Outliers;

public sealed class MappingCell
{
    public long Id { get; set; }
    public string TextToString { get; set; } = "";
    public long TextToInteger { get; set; }
    public decimal TextToDecimal { get; set; }
    public DateTime TextToDatetime { get; set; }
    public byte[] TextToBinary { get; set; } = [];
    public Guid TextToGuid { get; set; }
    public long IntToInteger { get; set; }
    public double IntToFloat { get; set; }
    public decimal IntToDecimal { get; set; }
    public bool IntToBoolean { get; set; }
    public long FloatToInteger { get; set; }
    public double FloatToFloat { get; set; }
    public decimal FloatToDecimal { get; set; }
    public long DecimalToInteger { get; set; }
    public double DecimalToFloat { get; set; }
    public decimal DecimalToDecimal { get; set; }
    public bool BoolToBoolean { get; set; }
    public DateTime DatetimeToDatetime { get; set; }
    public byte[] BinaryToBinary { get; set; } = [];
    public Guid BinaryToGuid { get; set; }
    public string GuidToString { get; set; } = "";
    public Guid GuidToGuid { get; set; }
}
