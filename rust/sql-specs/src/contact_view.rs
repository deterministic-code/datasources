#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddressView {
    pub id: i64,
    pub line1: String,
    pub city: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhoneView {
    pub id: i64,
    pub number: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContactView {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub contact_source_name: String,
    pub contact_type_name: String,
    pub addresses: Vec<AddressView>,
    pub phones: Vec<PhoneView>,
}
