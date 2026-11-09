// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type ContactBase. */
public class ContactBase : Base
{
    public string Name { get; set; } = "";
    public string GivenName { get; set; } = "";
    public string AdditionalName { get; set; } = "";
    public string Pronunciation { get; set; } = "";
    public string FamilyName { get; set; } = "";
    public string NickName { get; set; } = "";
    public string JobTitle { get; set; } = "";
    public long GenderTypeId { get; set; }
    public string Organization { get; set; } = "";
    public bool IsStarred { get; set; }
    public bool IsFavorite { get; set; }
    public bool IsDeleted { get; set; }
}
