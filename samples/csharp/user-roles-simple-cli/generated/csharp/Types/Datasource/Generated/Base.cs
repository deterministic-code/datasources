// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type Base. */
public class Base
{
    public long Id { get; set; }
    public string Uuid { get; set; } = Guid.NewGuid().ToString();
    public System.DateTime Created { get; set; }
    public byte[] Updated { get; set; } = new byte[] { 0 };
}
