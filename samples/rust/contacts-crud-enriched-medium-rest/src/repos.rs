use crate::repositories::contact_group_repository::ContactGroupRepository;
use crate::repositories::contact_repository::ContactRepository;
use crate::repositories::legacy_contact_repository::legacy_contact_repository;
use crate::LegacyContact;
use datasources::{CommandMiddleware, Connection, CRUDRepository, Result};
use std::sync::Arc;

pub fn open(
    connection: Arc<dyn Connection>,
    middleware: Option<Arc<dyn CommandMiddleware>>,
) -> Result<(ContactRepository, CRUDRepository<LegacyContact>, ContactGroupRepository)> {
    let contacts = ContactRepository::new(Arc::clone(&connection))?;
    let groups_repo = ContactGroupRepository::new(Arc::clone(&connection))?;
    let legacy = legacy_contact_repository(connection)?;
    if let Some(middleware) = middleware {
        contacts.add_middleware(Arc::clone(&middleware));
        groups_repo.add_middleware(Arc::clone(&middleware));
        legacy.add_middleware(middleware);
    }
    Ok((contacts, legacy, groups_repo))
}
