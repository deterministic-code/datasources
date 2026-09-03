use crate::contact_view::ContactView;

pub fn as_contacts_view(contacts: &[ContactView]) -> Vec<ContactView> {
    let mut items = contacts.to_vec();
    items.sort_by_key(|contact| contact.id);
    items
        .into_iter()
        .map(|mut contact| {
            contact.addresses.sort_by_key(|address| address.id);
            contact.phones.sort_by_key(|phone| phone.id);
            contact
        })
        .collect()
}
