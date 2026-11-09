use datasources::{Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, Association, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::Address;
use super::addressAttributesRepository::AddressAttributesRepository;
use super::labelsRepository::LabelsRepository;
use super::addressLabelsRepository::AddressLabelsRepository;
use super::addressTypesRepository::AddressTypesRepository;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct AddressesRepository {
    pub(crate) inner: Arc<CRUDRepository<Address>>,
    pub address_attribute: AddressAttributesRepository,
    pub label: LabelsRepository,
    pub address_label: AddressLabelsRepository,
    pub address_type: AddressTypesRepository,
}

impl AddressesRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
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
        ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
        ("display_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("line_1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("line_2".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("line_3".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("neighborhood".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("district".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("state_province".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("country_code".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("formatted_address".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
        ("longitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
        ("is_primary".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: true, is_id: false }),
        ("address_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("description".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
    ]));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "display_name".into(), "line_1".into(), "line_2".into(), "line_3".into(), "neighborhood".into(), "district".into(), "city".into(), "state_province".into(), "postal_code".into(), "country_code".into(), "formatted_address".into(), "latitude".into(), "longitude".into(), "is_primary".into(), "address_type_id".into()];
    options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "display_name".into(), "line_1".into(), "line_2".into(), "line_3".into(), "neighborhood".into(), "district".into(), "city".into(), "state_province".into(), "postal_code".into(), "country_code".into(), "formatted_address".into(), "latitude".into(), "longitude".into(), "is_primary".into()]);
    options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "display_name".into(), "line_1".into(), "line_2".into(), "line_3".into(), "neighborhood".into(), "district".into(), "city".into(), "state_province".into(), "postal_code".into(), "country_code".into(), "formatted_address".into(), "latitude".into(), "longitude".into(), "is_primary".into()]);

        let address_attribute = AddressAttributesRepository::new(Arc::clone(&connection))?;
        let label = LabelsRepository::new(Arc::clone(&connection))?;
        let address_label = AddressLabelsRepository::new(Arc::clone(&connection))?;
        let address_type = AddressTypesRepository::new(Arc::clone(&connection))?;
        let mut repo = CRUDRepository::<Address>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("attributes", "id", "address_id", Some("value".into()), EnrichmentPropertyType::Dictionary), &address_attribute, None);
        repo.add_linked(Enrichment::new("labels", "id", "id", Some("name".into()), EnrichmentPropertyType::List).linked(Association::new("address_id", "label_id")), &label, Arc::clone(&address_label.inner) as Arc<dyn ICRUDRepository<_>>, None);
        repo.add_enriched(Enrichment::new("address_type_name", "address_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), &address_type, None);
        repo.add_enriched(Enrichment::new("description", "address_type_id", "id", Some("description".into()), EnrichmentPropertyType::Object), &address_type, None);
        Ok(Self { inner: Arc::new(repo), address_attribute, label, address_label, address_type })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<Address>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
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
            ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
            ("display_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
            ("line_1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("line_2".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
            ("line_3".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
            ("neighborhood".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
            ("district".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
            ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("state_province".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
            ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
            ("country_code".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("formatted_address".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
            ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
            ("longitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
            ("is_primary".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: true, is_id: false }),
            ("address_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "display_name".into(), "line_1".into(), "line_2".into(), "line_3".into(), "neighborhood".into(), "district".into(), "city".into(), "state_province".into(), "postal_code".into(), "country_code".into(), "formatted_address".into(), "latitude".into(), "longitude".into(), "is_primary".into(), "address_type_id".into()];
        let address_attribute = AddressAttributesRepository::new(Arc::clone(&connection))?;
        let label = LabelsRepository::new(Arc::clone(&connection))?;
        let address_type = AddressTypesRepository::new(Arc::clone(&connection))?;
        let mut repo = ReadonlyRepository::<Address>::new(connection, options)?;
        repo.add_enriched(Enrichment::new("attributes", "id", "address_id", Some("value".into()), EnrichmentPropertyType::Dictionary), &address_attribute, None);
        repo.add_enriched(Enrichment::new("labels", "id", "id", Some("name".into()), EnrichmentPropertyType::List), &label, None);
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

    fn schema(&self) -> &Schema {
        self.inner.schema()
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
        ("contact_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
        ("display_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("line_1".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("line_2".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("line_3".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("neighborhood".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("district".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("city".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("state_province".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("postal_code".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("country_code".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("formatted_address".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
        ("latitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
        ("longitude".into(), SchemaColumn { type_name: "integer".into(), nullable: true, has_default: false, is_id: false }),
        ("is_primary".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: true, is_id: false }),
        ("address_type_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("description".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: true, is_id: false }),
    ]));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "display_name".into(), "line_1".into(), "line_2".into(), "line_3".into(), "neighborhood".into(), "district".into(), "city".into(), "state_province".into(), "postal_code".into(), "country_code".into(), "formatted_address".into(), "latitude".into(), "longitude".into(), "is_primary".into(), "address_type_id".into()];
    options.create_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "display_name".into(), "line_1".into(), "line_2".into(), "line_3".into(), "neighborhood".into(), "district".into(), "city".into(), "state_province".into(), "postal_code".into(), "country_code".into(), "formatted_address".into(), "latitude".into(), "longitude".into(), "is_primary".into()]);
    options.update_properties = Some(vec!["uuid".into(), "created".into(), "updated".into(), "version".into(), "contact_id".into(), "display_name".into(), "line_1".into(), "line_2".into(), "line_3".into(), "neighborhood".into(), "district".into(), "city".into(), "state_province".into(), "postal_code".into(), "country_code".into(), "formatted_address".into(), "latitude".into(), "longitude".into(), "is_primary".into()]);

        let address_attribute = Arc::new(super::addressAttributesRepository::address_attributes_row_repository(Arc::clone(&connection))?);
        let label = Arc::new(super::labelsRepository::labels_row_repository(Arc::clone(&connection))?);
        let address_label = Arc::new(super::addressLabelsRepository::address_labels_row_repository(Arc::clone(&connection))?);
        let address_type = Arc::new(super::addressTypesRepository::address_types_row_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("attributes", "id", "address_id", Some("value".into()), EnrichmentPropertyType::Dictionary), Arc::clone(&address_attribute), None);
        repo.add_linked(Enrichment::new("labels", "id", "id", Some("name".into()), EnrichmentPropertyType::List).linked(Association::new("address_id", "label_id")), Arc::clone(&label), Arc::clone(&address_label) as Arc<dyn ICRUDRepository<_>>, None);
        repo.add_enriched(Enrichment::new("address_type_name", "address_type_id", "id", Some("name".into()), EnrichmentPropertyType::Object), Arc::clone(&address_type), None);
        repo.add_enriched(Enrichment::new("description", "address_type_id", "id", Some("description".into()), EnrichmentPropertyType::Object), Arc::clone(&address_type), None);
        Ok(repo)
}
