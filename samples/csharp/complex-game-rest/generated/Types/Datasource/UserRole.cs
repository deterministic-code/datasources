// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type UserRole. */
public class UserRole : Base
{
    public long UserId { get; set; }
    public long RoleId { get; set; }
}
