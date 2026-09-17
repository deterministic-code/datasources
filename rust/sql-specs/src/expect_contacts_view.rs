use crate::contact_view::ContactView;

pub fn contact_with(contacts: &[ContactView], addresses: usize, phones: usize) -> &ContactView {
    contacts
        .iter()
        .find(|contact| contact.addresses.len() == addresses && contact.phones.len() == phones)
        .unwrap_or_else(|| panic!("no contact with {addresses} addresses and {phones} phones"))
}

pub fn expect_contacts_view(contacts: &[ContactView], count: usize) {
    assert_eq!(contacts.len(), count);
    let _ = contact_with(contacts, 2, 1);
    let _ = contact_with(contacts, 0, 1);
    let _ = contact_with(contacts, 2, 0);
    let _ = contact_with(contacts, 0, 0);
}
