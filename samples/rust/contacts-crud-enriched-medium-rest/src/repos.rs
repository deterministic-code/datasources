use crate::repositories::contact_groups_repository::ContactGroupsRepository;
use crate::repositories::contacts_repository::ContactsRepository;
use crate::repositories::legacy_contacts_repository::LegacyContactsRepository;
use datasources::{CommandMiddleware, Connection, Result};
use std::sync::Arc;

pub fn open(
    connection: Arc<dyn Connection>,
    middleware: Option<Arc<dyn CommandMiddleware>>,
) -> Result<(ContactsRepository, LegacyContactsRepository, ContactGroupsRepository)> {
    let contacts = ContactsRepository::new(Arc::clone(&connection))?;
    let groups_repo = ContactGroupsRepository::new(Arc::clone(&connection))?;
    let legacy = LegacyContactsRepository::new(connection)?;
    if let Some(middleware) = middleware {
        contacts.add_middleware(Arc::clone(&middleware));
        groups_repo.add_middleware(Arc::clone(&middleware));
        legacy.add_middleware(middleware);
    }
    Ok((contacts, legacy, groups_repo))
}
