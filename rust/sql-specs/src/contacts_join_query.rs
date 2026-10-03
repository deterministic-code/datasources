use datasources::{IWhere, Join, SelectStatement, SelectStatementOptions, Where};

pub fn contacts_join_query(where_clause: Option<&Where>) -> SelectStatement {
    SelectStatement::new(SelectStatementOptions {
        joins: &[
            Join::inner(
                "ContactSource",
                "Contact.ContactSourceId",
                "ContactSource.Id",
            ),
            Join::inner(
                "ContactType",
                "Contact.ContactTypeId",
                "ContactType.Id",
            ),
            Join::left("Address", "Contact.Id", "Address.ContactId"),
            Join.left("AddressType", "Address.AddressTypeId", "AddressType.Id"),
            Join.left("Phone", "Contact.Id", "Phone.ContactId"),
            Join.left("PhoneType", "Phone.PhoneTypeId", "PhoneType.Id"),
        ],
        where_clause: where_clause.map(|clause| clause as &dyn IWhere),
        ..SelectStatementOptions::new(
            "Contact",
            &[
                "Contact.Id AS Id",
                "Contact.FirstName AS FirstName",
                "Contact.LastName AS LastName",
                "Contact.Email AS Email",
                "ContactSource.Name AS ContactSourceName",
                "ContactType.Name AS ContactTypeName",
                "Address.Id AS AddressId",
                "Address.Line1 AS Line1",
                "Address.City AS City",
                "AddressType.Name AS AddressTypeName",
                "Phone.Id AS PhoneId",
                "Phone.Number AS Number",
                "PhoneType.Name AS PhoneTypeName",
            ],
        )
    })
}
