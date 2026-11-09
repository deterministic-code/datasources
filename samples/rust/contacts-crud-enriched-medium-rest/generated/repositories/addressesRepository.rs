use datasources::{Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::Address;
use super::addressTypesRepository::AddressTypesRepository;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct AddressesRepository {
    pub(crate) inner: Arc<CRUDRepository<Address>>,
    pub address_type: AddressTypesRepository,
}

impl AddressesRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Address".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Address".into(), "address_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
            ("label_note".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("floor".into(), SchemaColumn { type_name: "unsignedinteger".into(), nullable: true, has_default: false, is_id: false }),
            ("score".into(), SchemaColumn { type_name: "decimal".into(), nullable: true, has_default: false, is_id: false }),
            ("external_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: true, has_default: false, is_id: false }),
            ("is_verified".into(), SchemaColumn { type_name: "boolean".into(), nullable: true, has_default: false, is_id: false }),
            ("verified_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false, is_id: false }),
            ("stamp".into(), SchemaColumn { type_name: "binary".into(), nullable: true, has_default: false, is_id: false }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
            ("line1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("line2".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("region".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("country".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("address_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into(), "address_type_id".into()];
        options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options
    })
    .add_for("sqlite", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Address".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Address".into(), "addresses".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
            ("label_note".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("floor".into(), SchemaColumn { type_name: "unsignedinteger".into(), nullable: true, has_default: false, is_id: false }),
            ("score".into(), SchemaColumn { type_name: "decimal".into(), nullable: true, has_default: false, is_id: false }),
            ("external_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: true, has_default: false, is_id: false }),
            ("is_verified".into(), SchemaColumn { type_name: "boolean".into(), nullable: true, has_default: false, is_id: false }),
            ("verified_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false, is_id: false }),
            ("stamp".into(), SchemaColumn { type_name: "binary".into(), nullable: true, has_default: false, is_id: false }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
            ("line1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("line2".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("region".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("country".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("address_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into(), "address_type_id".into()];
        options.create_properties = Some(vec!["uuid".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options
    })
    .add_for("mysql", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Address".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Address".into(), "addresses".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
            ("label_note".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("floor".into(), SchemaColumn { type_name: "unsignedinteger".into(), nullable: true, has_default: false, is_id: false }),
            ("score".into(), SchemaColumn { type_name: "decimal".into(), nullable: true, has_default: false, is_id: false }),
            ("external_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: true, has_default: false, is_id: false }),
            ("is_verified".into(), SchemaColumn { type_name: "boolean".into(), nullable: true, has_default: false, is_id: false }),
            ("verified_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false, is_id: false }),
            ("stamp".into(), SchemaColumn { type_name: "binary".into(), nullable: true, has_default: false, is_id: false }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
            ("line1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("line2".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("region".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("country".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("address_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into(), "address_type_id".into()];
        options.create_properties = Some(vec!["latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options
    })
    .add_for("postgres", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Address".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Address".into(), "addresses".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
            ("label_note".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("floor".into(), SchemaColumn { type_name: "unsignedinteger".into(), nullable: true, has_default: false, is_id: false }),
            ("score".into(), SchemaColumn { type_name: "decimal".into(), nullable: true, has_default: false, is_id: false }),
            ("external_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: true, has_default: false, is_id: false }),
            ("is_verified".into(), SchemaColumn { type_name: "boolean".into(), nullable: true, has_default: false, is_id: false }),
            ("verified_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false, is_id: false }),
            ("stamp".into(), SchemaColumn { type_name: "binary".into(), nullable: true, has_default: false, is_id: false }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
            ("line1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("line2".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("region".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("country".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("address_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into(), "address_type_id".into()];
        options.create_properties = Some(vec!["latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

        let address_type = AddressTypesRepository::new(Arc::clone(&connection))?;
        let mut repo = CRUDRepository::<Address>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("address_type_name", "address_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), &address_type, None);
        repo.add_enriched(Enrichment::new("description", "address_type_id", "id", Some("description".into()), EnrichmentPropertyType::Object), &address_type, None);
        Ok(Self { inner: Arc::new(repo), address_type })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<Address>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "Address".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Address".into(), "address_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
            ("label_note".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("floor".into(), SchemaColumn { type_name: "unsignedinteger".into(), nullable: true, has_default: false, is_id: false }),
            ("score".into(), SchemaColumn { type_name: "decimal".into(), nullable: true, has_default: false, is_id: false }),
            ("external_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: true, has_default: false, is_id: false }),
            ("is_verified".into(), SchemaColumn { type_name: "boolean".into(), nullable: true, has_default: false, is_id: false }),
            ("verified_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false, is_id: false }),
            ("stamp".into(), SchemaColumn { type_name: "binary".into(), nullable: true, has_default: false, is_id: false }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
            ("line1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("line2".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("region".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("country".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("address_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into(), "address_type_id".into()];
        let address_type = AddressTypesRepository::new(Arc::clone(&connection))?;
        let mut repo = ReadonlyRepository::<Address>::new(connection, options)?;
        repo.add_enriched(Enrichment::new("address_type_name", "address_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), &address_type, None);
        repo.add_enriched(Enrichment::new("description", "address_type_id", "id", Some("description".into()), EnrichmentPropertyType::Object), &address_type, None);
        Ok(repo)
    }
}

impl Deref for AddressesRepository {
    type Target = CRUDRepository<Address>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<Address> for &AddressesRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<Address>>,
        Option<Arc<dyn ICRUDRepository<Address>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<Address>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<Address>>),
        )
    }
}

impl IRepository for AddressesRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }
}

#[async_trait]
impl IReadonlyRepository<Address> for AddressesRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<Address>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Address>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Address>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Address>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Address>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}

#[async_trait]
impl ICRUDRepository<Address> for AddressesRepository {
    async fn create(&self, item: Address) -> Result<Address> {
        self.inner.create(item).await
    }

    async fn update(&self, item: Address, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: Address, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Address>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn addresses_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Address".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Address".into(), "address_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
            ("label_note".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("floor".into(), SchemaColumn { type_name: "unsignedinteger".into(), nullable: true, has_default: false, is_id: false }),
            ("score".into(), SchemaColumn { type_name: "decimal".into(), nullable: true, has_default: false, is_id: false }),
            ("external_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: true, has_default: false, is_id: false }),
            ("is_verified".into(), SchemaColumn { type_name: "boolean".into(), nullable: true, has_default: false, is_id: false }),
            ("verified_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false, is_id: false }),
            ("stamp".into(), SchemaColumn { type_name: "binary".into(), nullable: true, has_default: false, is_id: false }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
            ("line1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("line2".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("region".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("country".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("address_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into(), "address_type_id".into()];
        options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options
    })
    .add_for("sqlite", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Address".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Address".into(), "addresses".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
            ("label_note".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("floor".into(), SchemaColumn { type_name: "unsignedinteger".into(), nullable: true, has_default: false, is_id: false }),
            ("score".into(), SchemaColumn { type_name: "decimal".into(), nullable: true, has_default: false, is_id: false }),
            ("external_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: true, has_default: false, is_id: false }),
            ("is_verified".into(), SchemaColumn { type_name: "boolean".into(), nullable: true, has_default: false, is_id: false }),
            ("verified_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false, is_id: false }),
            ("stamp".into(), SchemaColumn { type_name: "binary".into(), nullable: true, has_default: false, is_id: false }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
            ("line1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("line2".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("region".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("country".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("address_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into(), "address_type_id".into()];
        options.create_properties = Some(vec!["uuid".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options
    })
    .add_for("mysql", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Address".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Address".into(), "addresses".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
            ("label_note".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("floor".into(), SchemaColumn { type_name: "unsignedinteger".into(), nullable: true, has_default: false, is_id: false }),
            ("score".into(), SchemaColumn { type_name: "decimal".into(), nullable: true, has_default: false, is_id: false }),
            ("external_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: true, has_default: false, is_id: false }),
            ("is_verified".into(), SchemaColumn { type_name: "boolean".into(), nullable: true, has_default: false, is_id: false }),
            ("verified_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false, is_id: false }),
            ("stamp".into(), SchemaColumn { type_name: "binary".into(), nullable: true, has_default: false, is_id: false }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
            ("line1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("line2".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("region".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("country".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("address_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into(), "address_type_id".into()];
        options.create_properties = Some(vec!["latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options
    })
    .add_for("postgres", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Address".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Address".into(), "addresses".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
            ("label_note".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("floor".into(), SchemaColumn { type_name: "unsignedinteger".into(), nullable: true, has_default: false, is_id: false }),
            ("score".into(), SchemaColumn { type_name: "decimal".into(), nullable: true, has_default: false, is_id: false }),
            ("external_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: true, has_default: false, is_id: false }),
            ("is_verified".into(), SchemaColumn { type_name: "boolean".into(), nullable: true, has_default: false, is_id: false }),
            ("verified_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false, is_id: false }),
            ("stamp".into(), SchemaColumn { type_name: "binary".into(), nullable: true, has_default: false, is_id: false }),
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
            ("line1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("line2".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("region".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("country".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("address_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into(), "address_type_id".into()];
        options.create_properties = Some(vec!["latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options.update_properties = Some(vec!["uuid".into(), "version".into(), "latitude".into(), "label_note".into(), "floor".into(), "score".into(), "external_id".into(), "is_verified".into(), "verified_at".into(), "stamp".into(), "contact_id".into(), "line1".into(), "line2".into(), "city".into(), "region".into(), "postal_code".into(), "country".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

        let address_type = Arc::new(super::addressTypesRepository::address_types_row_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("address_type_name", "address_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&address_type), None);
        repo.add_enriched(Enrichment::new("description", "address_type_id", "id", Some("description".into()), EnrichmentPropertyType::Object), Arc::clone(&address_type), None);
        Ok(repo)
}
