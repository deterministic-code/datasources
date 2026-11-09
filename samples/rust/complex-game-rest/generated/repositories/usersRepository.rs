use datasources::{Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, Association, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::User;
use super::userSettingsRepository::UserSettingsRepository;
use super::rolesRepository::RolesRepository;
use super::userRolesRepository::UserRolesRepository;
use super::groupsRepository::GroupsRepository;
use super::userGroupsRepository::UserGroupsRepository;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct UsersRepository {
    pub(crate) inner: Arc<CRUDRepository<User>>,
    pub user_settings: UserSettingsRepository,
    pub role: RolesRepository,
    pub user_role: UserRolesRepository,
    pub group: GroupsRepository,
    pub user_group: UserGroupsRepository,
}

impl UsersRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "User".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("User".into(), "user_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("username".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("password".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("is_enabled".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: true, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "username".into(), "password".into(), "is_enabled".into()];
        options.create_properties = Some(vec!["username".into(), "password".into(), "is_enabled".into()]);
        options.update_properties = Some(vec!["uuid".into(), "username".into(), "password".into(), "is_enabled".into()]);
        options
    })
    .add_for("sqlite", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "User".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("User".into(), "user_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("username".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("password".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("is_enabled".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: true, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "username".into(), "password".into(), "is_enabled".into()];
        options.create_properties = Some(vec!["uuid".into(), "created".into(), "version".into(), "username".into(), "password".into(), "is_enabled".into()]);
        options.update_properties = Some(vec!["uuid".into(), "created".into(), "version".into(), "username".into(), "password".into(), "is_enabled".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

        let user_settings = UserSettingsRepository::new(Arc::clone(&connection))?;
        let role = RolesRepository::new(Arc::clone(&connection))?;
        let user_role = UserRolesRepository::new(Arc::clone(&connection))?;
        let group = GroupsRepository::new(Arc::clone(&connection))?;
        let user_group = UserGroupsRepository::new(Arc::clone(&connection))?;
        let mut repo = CRUDRepository::<User>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("settings", "id", "user_id", Some("value".into()), EnrichmentPropertyType::Dictionary), &user_settings, None);
        repo.add_linked(Enrichment::new("roles", "id", "id", Some("name".into()), EnrichmentPropertyType::List).linked(Association::new("user_id", "role_id")), &role, Arc::clone(&user_role.inner) as Arc<dyn ICRUDRepository<_>>, None);
        repo.add_linked(Enrichment::new("groups", "id", "id", Some("name".into()), EnrichmentPropertyType::List).linked(Association::new("user_id", "group_id")), &group, Arc::clone(&user_group.inner) as Arc<dyn ICRUDRepository<_>>, None);
        Ok(Self { inner: Arc::new(repo), user_settings, role, user_role, group, user_group })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<User>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "User".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("User".into(), "user_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("username".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("password".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("is_enabled".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: true, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "username".into(), "password".into(), "is_enabled".into()];
        let user_settings = UserSettingsRepository::new(Arc::clone(&connection))?;
        let role = RolesRepository::new(Arc::clone(&connection))?;
        let group = GroupsRepository::new(Arc::clone(&connection))?;
        let mut repo = ReadonlyRepository::<User>::new(connection, options)?;
        repo.add_enriched(Enrichment::new("settings", "id", "user_id", Some("value".into()), EnrichmentPropertyType::Dictionary), &user_settings, None);
        repo.add_enriched(Enrichment::new("roles", "id", "id", Some("name".into()), EnrichmentPropertyType::List), &role, None);
        repo.add_enriched(Enrichment::new("groups", "id", "id", Some("name".into()), EnrichmentPropertyType::List), &group, None);
        Ok(repo)
    }
}

impl Deref for UsersRepository {
    type Target = CRUDRepository<User>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<User> for &UsersRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<User>>,
        Option<Arc<dyn ICRUDRepository<User>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<User>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<User>>),
        )
    }
}

impl IRepository for UsersRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }

    fn schema(&self) -> &Schema {
        self.inner.schema()
    }
}

#[async_trait]
impl IReadonlyRepository<User> for UsersRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<User>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<User>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<User>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<User>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<User>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}

#[async_trait]
impl ICRUDRepository<User> for UsersRepository {
    async fn create(&self, item: User) -> Result<User> {
        self.inner.create(item).await
    }

    async fn update(&self, item: User, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: User, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<User>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn users_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "User".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("User".into(), "user_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("username".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("password".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("is_enabled".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: true, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "username".into(), "password".into(), "is_enabled".into()];
        options.create_properties = Some(vec!["username".into(), "password".into(), "is_enabled".into()]);
        options.update_properties = Some(vec!["uuid".into(), "username".into(), "password".into(), "is_enabled".into()]);
        options
    })
    .add_for("sqlite", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "User".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("User".into(), "user_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("username".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("password".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("is_enabled".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: true, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "username".into(), "password".into(), "is_enabled".into()];
        options.create_properties = Some(vec!["uuid".into(), "created".into(), "version".into(), "username".into(), "password".into(), "is_enabled".into()]);
        options.update_properties = Some(vec!["uuid".into(), "created".into(), "version".into(), "username".into(), "password".into(), "is_enabled".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

        let user_settings = Arc::new(super::userSettingsRepository::user_settings_row_repository(Arc::clone(&connection))?);
        let role = Arc::new(super::rolesRepository::roles_row_repository(Arc::clone(&connection))?);
        let user_role = Arc::new(super::userRolesRepository::user_roles_row_repository(Arc::clone(&connection))?);
        let group = Arc::new(super::groupsRepository::groups_row_repository(Arc::clone(&connection))?);
        let user_group = Arc::new(super::userGroupsRepository::user_groups_row_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("settings", "id", "user_id", Some("value".into()), EnrichmentPropertyType::Dictionary), Arc::clone(&user_settings), None);
        repo.add_linked(Enrichment::new("roles", "id", "id", Some("name".into()), EnrichmentPropertyType::List).linked(Association::new("user_id", "role_id")), Arc::clone(&role), Arc::clone(&user_role) as Arc<dyn ICRUDRepository<_>>, None);
        repo.add_linked(Enrichment::new("groups", "id", "id", Some("name".into()), EnrichmentPropertyType::List).linked(Association::new("user_id", "group_id")), Arc::clone(&group), Arc::clone(&user_group) as Arc<dyn ICRUDRepository<_>>, None);
        Ok(repo)
}
