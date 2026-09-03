use crate::contact_view::ContactView;

pub fn expect_contacts_view(contacts: &[ContactView], count: usize) {
    let of = |id: i64| contacts.iter().find(|contact| contact.id == id);
    assert_eq!(contacts.len(), count);
    assert_eq!(of(10).map(|c| c.addresses.len()), Some(2));
    assert_eq!(of(10).map(|c| c.phones.len()), Some(1));
    assert_eq!(of(4).map(|c| c.addresses.len()), Some(0));
    assert_eq!(of(4).map(|c| c.phones.len()), Some(1));
    assert_eq!(of(6).map(|c| c.addresses.len()), Some(2));
    assert_eq!(of(6).map(|c| c.phones.len()), Some(0));
    assert_eq!(of(12).map(|c| c.addresses.len()), Some(0));
    assert_eq!(of(12).map(|c| c.phones.len()), Some(0));
}
