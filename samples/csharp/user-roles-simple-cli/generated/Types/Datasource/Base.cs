// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type Base. */
public class Base
{
    public long Id { get; set; }
    public System.Guid Uuid { get; set; } = Guid.NewGuid();
    public System.DateTime Created { get; set; } = DateTime.UtcNow;
    public byte[] Updated { get; set; } = new byte[] { 0 };
}
