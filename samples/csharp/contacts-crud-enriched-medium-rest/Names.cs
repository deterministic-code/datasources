using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public static class Names
{
    public static readonly IMapper<string, string> Columns = new DictionaryMapper<string>(new Dictionary<string, string>
    {
        ["id"] = "Id",
        ["uuid"] = "m_Uuid",
        ["created"] = "Created",
        ["updated"] = "m_Updated",
        ["version"] = "Version",
        ["name"] = "m_Name",
        ["description"] = "Description",
        ["first_name"] = "m_FirstName",
        ["last_name"] = "LastName",
        ["email"] = "m_Email",
        ["notes"] = "Notes",
        ["contact_source_id"] = "ContactSourceId",
        ["contact_type_id"] = "m_ContactTypeId",
        ["contact_id"] = "ContactId",
        ["address_type_id"] = "m_AddressTypeId",
        ["phone_type_id"] = "PhoneTypeId",
        ["line1"] = "m_Line1",
        ["line2"] = "Line2",
        ["city"] = "m_City",
        ["region"] = "Region",
        ["postal_code"] = "m_PostalCode",
        ["country"] = "Country",
        ["number"] = "m_Number",
        ["contact_group_id"] = "ContactGroupId",
        ["key"] = "m_Key",
        ["imported_at"] = "ImportedAt",
        ["occurred_at"] = "m_OccurredAt",
        ["changed_by"] = "ChangedBy",
        ["change_summary"] = "m_ChangeSummary"
    });

    public static readonly IMapper<string, string> Tables = new DictionaryMapper<string>(new Dictionary<string, string>
    {
        ["contacts"] = "tbl_Contacts",
        ["address"] = "tbl_Address",
        ["phone"] = "tbl_Phone",
        ["contact_groups"] = "tbl_ContactGroups",
        ["contact_group_member"] = "tbl_ContactGroupMember",
        ["contact_type"] = "tbl_ContactType",
        ["phone_type"] = "tbl_PhoneType",
        ["contact_change_log"] = "tbl_ContactChangeLog",
        ["contact_source"] = "ContactSource",
        ["address_type"] = "AddressType",
        ["legacy_contact"] = "legacy_contact"
    });

    public static string Column(string logical) => Columns.Map(logical);

    public static string Table(string logical) => Tables.Map(logical);
}
