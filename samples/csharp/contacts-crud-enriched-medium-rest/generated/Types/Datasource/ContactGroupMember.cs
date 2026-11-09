// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type ContactGroupMember. */
public class ContactGroupMember : Base
{
    public long ContactId { get; set; }
    public long ContactGroupId { get; set; }
}
