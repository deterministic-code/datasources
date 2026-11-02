use datasources::{ Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, IReadonlyRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Result};
use super::super::Address;
use super::super::AddressType;
use std::collections::HashMap;
use std::sync::Arc;
use std::ops::Deref;

pub struct AddressRepository {
    pub repo: Arc<CRUDRepository<Address>>,
    pub address_type: Arc<CRUDRepository<AddressType>>,
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
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false }),
            ("label_note".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("floor".into(), SchemaColumn { type_name: "unsignedinteger".into(), nullable: true, has_default: false }),
            ("score".into(), SchemaColumn { type_name: "decimal".into(), nullable: true, has_default: false }),
            ("external_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: true, has_default: false }),
            ("is_verified".into(), SchemaColumn { type_name: "boolean".into(), nullable: true, has_default: false }),
            ("verified_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false }),
            ("stamp".into(), SchemaColumn { type_name: "binary".into(), nullable: true, has_default: false }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("address_type_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("line1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("line2".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("region".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("country".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
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
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false }),
            ("label_note".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("floor".into(), SchemaColumn { type_name: "unsignedinteger".into(), nullable: true, has_default: false }),
            ("score".into(), SchemaColumn { type_name: "decimal".into(), nullable: true, has_default: false }),
            ("external_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: true, has_default: false }),
            ("is_verified".into(), SchemaColumn { type_name: "boolean".into(), nullable: true, has_default: false }),
            ("verified_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false }),
            ("stamp".into(), SchemaColumn { type_name: "binary".into(), nullable: true, has_default: false }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("address_type_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("line1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("line2".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("region".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("country".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
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
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false }),
            ("label_note".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("floor".into(), SchemaColumn { type_name: "unsignedinteger".into(), nullable: true, has_default: false }),
            ("score".into(), SchemaColumn { type_name: "decimal".into(), nullable: true, has_default: false }),
            ("external_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: true, has_default: false }),
            ("is_verified".into(), SchemaColumn { type_name: "boolean".into(), nullable: true, has_default: false }),
            ("verified_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false }),
            ("stamp".into(), SchemaColumn { type_name: "binary".into(), nullable: true, has_default: false }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("address_type_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("line1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("line2".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("region".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("country".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
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
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false }),
            ("label_note".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("floor".into(), SchemaColumn { type_name: "unsignedinteger".into(), nullable: true, has_default: false }),
            ("score".into(), SchemaColumn { type_name: "decimal".into(), nullable: true, has_default: false }),
            ("external_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: true, has_default: false }),
            ("is_verified".into(), SchemaColumn { type_name: "boolean".into(), nullable: true, has_default: false }),
            ("verified_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false }),
            ("stamp".into(), SchemaColumn { type_name: "binary".into(), nullable: true, has_default: false }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("address_type_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("line1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("line2".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("region".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("country".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()];
        options.create_properties = Some(vec!["latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

        let address_type = Arc::new(super::address_type_repository::address_type_repository(Arc::clone(&connection))?);
        let repo = CRUDRepository::<Address>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("address_type_name", "address_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&address_type) as Arc<dyn IReadonlyRepository<AddressType>>, None);
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
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false }),
            ("label_note".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("floor".into(), SchemaColumn { type_name: "unsignedinteger".into(), nullable: true, has_default: false }),
            ("score".into(), SchemaColumn { type_name: "decimal".into(), nullable: true, has_default: false }),
            ("external_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: true, has_default: false }),
            ("is_verified".into(), SchemaColumn { type_name: "boolean".into(), nullable: true, has_default: false }),
            ("verified_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false }),
            ("stamp".into(), SchemaColumn { type_name: "binary".into(), nullable: true, has_default: false }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("address_type_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("line1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("line2".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("region".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("country".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
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
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false }),
            ("label_note".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("floor".into(), SchemaColumn { type_name: "unsignedinteger".into(), nullable: true, has_default: false }),
            ("score".into(), SchemaColumn { type_name: "decimal".into(), nullable: true, has_default: false }),
            ("external_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: true, has_default: false }),
            ("is_verified".into(), SchemaColumn { type_name: "boolean".into(), nullable: true, has_default: false }),
            ("verified_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false }),
            ("stamp".into(), SchemaColumn { type_name: "binary".into(), nullable: true, has_default: false }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("address_type_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("line1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("line2".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("region".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("country".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
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
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false }),
            ("label_note".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("floor".into(), SchemaColumn { type_name: "unsignedinteger".into(), nullable: true, has_default: false }),
            ("score".into(), SchemaColumn { type_name: "decimal".into(), nullable: true, has_default: false }),
            ("external_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: true, has_default: false }),
            ("is_verified".into(), SchemaColumn { type_name: "boolean".into(), nullable: true, has_default: false }),
            ("verified_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false }),
            ("stamp".into(), SchemaColumn { type_name: "binary".into(), nullable: true, has_default: false }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("address_type_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("line1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("line2".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("region".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("country".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
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
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
            ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false }),
            ("label_note".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("floor".into(), SchemaColumn { type_name: "unsignedinteger".into(), nullable: true, has_default: false }),
            ("score".into(), SchemaColumn { type_name: "decimal".into(), nullable: true, has_default: false }),
            ("external_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: true, has_default: false }),
            ("is_verified".into(), SchemaColumn { type_name: "boolean".into(), nullable: true, has_default: false }),
            ("verified_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false }),
            ("stamp".into(), SchemaColumn { type_name: "binary".into(), nullable: true, has_default: false }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("address_type_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
            ("line1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("line2".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
            ("region".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
            ("country".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()];
        options.create_properties = Some(vec!["latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

        let address_type = Arc::new(super::address_type_repository::address_type_row_repository(Arc::clone(&connection))?);
        let repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("address_type_name", "address_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&address_type) as Arc<dyn IReadonlyRepository>, None);
        Ok(repo)
}

pub fn address_readonly_repository(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<Address>> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "Address".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("Address".into(), "address_bases".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true }),
        ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
        ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true }),
        ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true }),
        ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false }),
        ("label_note".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
        ("floor".into(), SchemaColumn { type_name: "unsignedinteger".into(), nullable: true, has_default: false }),
        ("score".into(), SchemaColumn { type_name: "decimal".into(), nullable: true, has_default: false }),
        ("external_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: true, has_default: false }),
        ("is_verified".into(), SchemaColumn { type_name: "boolean".into(), nullable: true, has_default: false }),
        ("verified_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false }),
        ("stamp".into(), SchemaColumn { type_name: "binary".into(), nullable: true, has_default: false }),
        ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ("address_type_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false }),
        ("line1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
        ("line2".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
        ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false }),
        ("region".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
        ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
        ("country".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false }),
    ]));
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "address_type_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()];
        let address_type = Arc::new(super::address_type_repository::address_type_repository(Arc::clone(&connection))?);
        let repo = ReadonlyRepository::<Address>::new(connection, options)?;
        repo.add_enriched(Enrichment::new("address_type_name", "address_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&address_type) as Arc<dyn IReadonlyRepository<AddressType>>, None);
        Ok(repo)
}
