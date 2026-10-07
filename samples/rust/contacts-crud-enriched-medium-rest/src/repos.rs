use crate::repositories::{
    address_repository::address_repository, contact_source_repository::contact_source_repository,
    legacy_contact_repository::legacy_contact_repository, phone_repository::phone_repository,
};
use datasources::{
    CommandMiddleware, Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper,
    Enrichment, EnrichmentPropertyType, Result,
};
use std::sync::Arc;

pub struct Repos {
    pub contacts: CRUDRepository,
    pub contact_source: CRUDRepository,
    pub legacy: CRUDRepository,
}

pub fn open(
    connection: Arc<dyn Connection>,
    middleware: Option<Arc<dyn CommandMiddleware>>,
) -> Result<Repos> {
    let mut address = address_repository(Arc::clone(&connection))?;
    let mut phone = phone_repository(Arc::clone(&connection))?;
    let mut contact_source = contact_source_repository(Arc::clone(&connection))?;
    let mut contact_source_lookup = contact_source_repository(Arc::clone(&connection))?;
    let mut contacts = contact_repository(Arc::clone(&connection))?;
    let mut legacy = legacy_contact_repository(connection)?;
    if let Some(middleware) = middleware {
        address.add_middleware(Arc::clone(&middleware));
        phone.add_middleware(Arc::clone(&middleware));
        contact_source.add_middleware(Arc::clone(&middleware));
        contact_source_lookup.add_middleware(Arc::clone(&middleware));
        contacts.add_middleware(Arc::clone(&middleware));
        legacy.add_middleware(middleware);
    }
    contacts.add_enriched(
        Enrichment::new(
            "addresses",
            "id",
            "contact_id",
            None,
            EnrichmentPropertyType::List,
        ),
        Arc::new(address),
        None,
    );
    contacts.add_enriched(
        Enrichment::new(
            "phones",
            "id",
            "contact_id",
            None,
            EnrichmentPropertyType::List,
        ),
        Arc::new(phone),
        None,
    );
    contacts.add_enriched(
        Enrichment::new(
            "contact_source_name",
            "contact_source_id",
            "id",
            Some("name".into()),
            EnrichmentPropertyType::Object,
        ),
        Arc::new(contact_source),
        None,
    );
    Ok(Repos {
        contacts,
        contact_source: contact_source_lookup,
        legacy,
    })
}

fn contact_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "Contact".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([(
        "Contact".into(),
        "contacts".into(),
    )])));
    options.ids = vec!["id".into()];
    options.read_properties = vec![
        "id".into(),
        "uuid".into(),
        "created".into(),
        "updated".into(),
        "version".into(),
        "contact_source_id".into(),
        "first_name".into(),
        "last_name".into(),
        "email".into(),
        "notes".into(),
    ];
    CRUDRepository::new(connection, options)
}
