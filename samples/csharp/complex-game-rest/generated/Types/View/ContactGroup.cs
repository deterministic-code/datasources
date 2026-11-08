// schema-version: 1.0
using System.Collections.Generic;

namespace Backend.Types.View;

/** View ContactGroup. */
public class ContactGroup : Backend.Types.Datasource.ContactGroupsBase
{
    public List<Contact> Members { get; set; } = [];
}
