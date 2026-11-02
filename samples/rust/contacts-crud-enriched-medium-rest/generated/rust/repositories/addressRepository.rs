use datasources::{Association, Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, ICRUDRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, Value, Result};
use super::super::Address;
use std::collections::HashMap;
use std::sync::Arc;
use std::ops::Deref;

pub struct AddressRepository {
    pub repo: Arc<CRUDRepository<Address>>,
    pub address_type: Arc<CRUDRepository>,
}

impl AddressRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let options = CRUDRepositoryBuilder::new()
            .add({
                let mut options = CRUDRepositoryOptions::default();
                options.entity = "Address".into();
                options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                    ("Address".into(), "address_bases".into()),
                ])));
                options.schema = Schema::new(HashMap::from([
                        ("id".into(), "integer".into()),
                        ("uuid".into(), "uuid".into()),
                        ("created".into(), "datetime".into()),
                        ("updated".into(), "datetime".into()),
                        ("version".into(), "binary".into()),
                        ("latitude".into(), "integer".into()),
                        ("floor".into(), "unsignedinteger".into()),
                        ("score".into(), "decimal".into()),
                        ("external_id".into(), "uuid".into()),
                        ("is_verified".into(), "boolean".into()),
                        ("verified_at".into(), "datetime".into()),
                        ("stamp".into(), "binary".into()),
                    ]));
                options.ids = vec!["id".into()];
                options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()];
                options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
                options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
                options
            })
            .add_for("sqlite", {
                let mut options = CRUDRepositoryOptions::default();
                options.entity = "Address".into();
                options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                    ("Address".into(), "addresses".into()),
                ])));
                options.schema = Schema::new(HashMap::from([
                        ("id".into(), "integer".into()),
                        ("uuid".into(), "uuid".into()),
                        ("created".into(), "datetime".into()),
                        ("updated".into(), "datetime".into()),
                        ("version".into(), "binary".into()),
                        ("latitude".into(), "integer".into()),
                        ("floor".into(), "unsignedinteger".into()),
                        ("score".into(), "decimal".into()),
                        ("external_id".into(), "uuid".into()),
                        ("is_verified".into(), "boolean".into()),
                        ("verified_at".into(), "datetime".into()),
                        ("stamp".into(), "binary".into()),
                    ]));
                options.ids = vec!["id".into()];
                options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()];
                options.create_properties = Some(vec!["uuid".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
                options.update_properties = Some(vec!["uuid".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
                options
            })
            .add_for("mysql", {
                let mut options = CRUDRepositoryOptions::default();
                options.entity = "Address".into();
                options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                    ("Address".into(), "addresses".into()),
                ])));
                options.schema = Schema::new(HashMap::from([
                        ("id".into(), "integer".into()),
                        ("uuid".into(), "uuid".into()),
                        ("created".into(), "datetime".into()),
                        ("updated".into(), "datetime".into()),
                        ("version".into(), "binary".into()),
                        ("latitude".into(), "integer".into()),
                        ("floor".into(), "unsignedinteger".into()),
                        ("score".into(), "decimal".into()),
                        ("external_id".into(), "uuid".into()),
                        ("is_verified".into(), "boolean".into()),
                        ("verified_at".into(), "datetime".into()),
                        ("stamp".into(), "binary".into()),
                    ]));
                options.ids = vec!["id".into()];
                options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()];
                options.create_properties = Some(vec!["latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
                options.update_properties = Some(vec!["uuid".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
                options
            })
            .add_for("postgres", {
                let mut options = CRUDRepositoryOptions::default();
                options.entity = "Address".into();
                options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                    ("Address".into(), "addresses".into()),
                ])));
                options.schema = Schema::new(HashMap::from([
                        ("id".into(), "integer".into()),
                        ("uuid".into(), "uuid".into()),
                        ("created".into(), "datetime".into()),
                        ("updated".into(), "datetime".into()),
                        ("version".into(), "binary".into()),
                        ("latitude".into(), "integer".into()),
                        ("floor".into(), "unsignedinteger".into()),
                        ("score".into(), "decimal".into()),
                        ("external_id".into(), "uuid".into()),
                        ("is_verified".into(), "boolean".into()),
                        ("verified_at".into(), "datetime".into()),
                        ("stamp".into(), "binary".into()),
                    ]));
                options.ids = vec!["id".into()];
                options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()];
                options.create_properties = Some(vec!["latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
                options.update_properties = Some(vec!["uuid".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
                options
            })
            .for_connection(connection.as_ref())?;
        let address_type = Arc::new(super::addressTypeRepository::address_type_row_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::<Address>::new(Arc::clone(&connection), options).map(|repo| repo.with_create_zeros([
        ("line1".into(), Value::from("")),
        ("city".into(), Value::from("")),
    ].into_iter().collect()))?;
        repo.add_enriched(Enrichment::new("address_type_name", "address_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&address_type), None);
        Ok(Self { repo: Arc::new(repo), address_type })
    }
}

impl Deref for AddressRepository {
    type Target = CRUDRepository<Address>;
    fn deref(&self) -> &Self::Target {
        self.repo.as_ref()
    }
}

pub fn address_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<Address>> {
    Arc::try_unwrap(AddressRepository::new(connection)?.repo).map_err(|_| datasources::DatasourceError::message("repository"))
}

pub fn address_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let options = CRUDRepositoryBuilder::new()
        .add({
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "Address".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("Address".into(), "address_bases".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                    ("latitude".into(), "integer".into()),
                    ("floor".into(), "unsignedinteger".into()),
                    ("score".into(), "decimal".into()),
                    ("external_id".into(), "uuid".into()),
                    ("is_verified".into(), "boolean".into()),
                    ("verified_at".into(), "datetime".into()),
                    ("stamp".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()];
            options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
            options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
            options
        })
        .add_for("sqlite", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "Address".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("Address".into(), "addresses".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                    ("latitude".into(), "integer".into()),
                    ("floor".into(), "unsignedinteger".into()),
                    ("score".into(), "decimal".into()),
                    ("external_id".into(), "uuid".into()),
                    ("is_verified".into(), "boolean".into()),
                    ("verified_at".into(), "datetime".into()),
                    ("stamp".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()];
            options.create_properties = Some(vec!["uuid".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
            options
        })
        .add_for("mysql", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "Address".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("Address".into(), "addresses".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                    ("latitude".into(), "integer".into()),
                    ("floor".into(), "unsignedinteger".into()),
                    ("score".into(), "decimal".into()),
                    ("external_id".into(), "uuid".into()),
                    ("is_verified".into(), "boolean".into()),
                    ("verified_at".into(), "datetime".into()),
                    ("stamp".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()];
            options.create_properties = Some(vec!["latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
            options
        })
        .add_for("postgres", {
            let mut options = CRUDRepositoryOptions::default();
            options.entity = "Address".into();
            options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
                ("Address".into(), "addresses".into()),
            ])));
            options.schema = Schema::new(HashMap::from([
                    ("id".into(), "integer".into()),
                    ("uuid".into(), "uuid".into()),
                    ("created".into(), "datetime".into()),
                    ("updated".into(), "datetime".into()),
                    ("version".into(), "binary".into()),
                    ("latitude".into(), "integer".into()),
                    ("floor".into(), "unsignedinteger".into()),
                    ("score".into(), "decimal".into()),
                    ("external_id".into(), "uuid".into()),
                    ("is_verified".into(), "boolean".into()),
                    ("verified_at".into(), "datetime".into()),
                    ("stamp".into(), "binary".into()),
                ]));
            options.ids = vec!["id".into()];
            options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()];
            options.create_properties = Some(vec!["latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
            options.update_properties = Some(vec!["uuid".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
            options
        })
        .for_connection(connection.as_ref())?;
        let address_type = Arc::new(super::addressTypeRepository::address_type_row_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("address_type_name", "address_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&address_type), None);
        Ok(repo)
}

pub fn address_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<Address>> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "Address".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("Address".into(), "address_bases".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
            ("id".into(), "integer".into()),
            ("uuid".into(), "uuid".into()),
            ("created".into(), "datetime".into()),
            ("updated".into(), "datetime".into()),
            ("version".into(), "binary".into()),
            ("latitude".into(), "integer".into()),
            ("floor".into(), "unsignedinteger".into()),
            ("score".into(), "decimal".into()),
            ("external_id".into(), "uuid".into()),
            ("is_verified".into(), "boolean".into()),
            ("verified_at".into(), "datetime".into()),
            ("stamp".into(), "binary".into()),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()];
        let address_type = Arc::new(super::addressTypeRepository::address_type_row_repository(Arc::clone(&connection))?);
        let mut repo = ReadonlyRepository::<Address>::new(connection, options)?;
        repo.add_enriched(Enrichment::new("address_type_name", "address_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&address_type), None);
        Ok(repo)
}
