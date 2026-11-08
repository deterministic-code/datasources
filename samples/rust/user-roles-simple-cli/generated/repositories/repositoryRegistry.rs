use datasources::{Datasource, ICRUDRepository, IReadonlyRepository, RepositoryFactory};
use super::super::Group;
use super::super::Role;
use super::super::UserBase;
use super::super::UserGroup;
use super::super::UserRole;
use std::sync::Arc;

pub fn register(factory: &RepositoryFactory) {
    factory.register_named(
        "Group",
        "groups",
        |connection| Arc::new(super::groupsRepository::GroupsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Group>>,
        |connection| Arc::new(super::groupsRepository::GroupsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Group>>,
    );
    factory.register_named(
        "Role",
        "roles",
        |connection| Arc::new(super::rolesRepository::RolesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Role>>,
        |connection| Arc::new(super::rolesRepository::RolesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Role>>,
    );
    factory.register_named(
        "UserBase",
        "user_bases",
        |connection| Arc::new(super::userBasesRepository::UserBasesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<UserBase>>,
        |connection| Arc::new(super::userBasesRepository::UserBasesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<UserBase>>,
    );
    factory.register_named(
        "UserGroup",
        "user_groups",
        |connection| Arc::new(super::userGroupsRepository::UserGroupsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<UserGroup>>,
        |connection| Arc::new(super::userGroupsRepository::UserGroupsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<UserGroup>>,
    );
    factory.register_named(
        "UserRole",
        "user_roles",
        |connection| Arc::new(super::userRolesRepository::UserRolesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<UserRole>>,
        |connection| Arc::new(super::userRolesRepository::UserRolesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<UserRole>>,
    );
}

datasources::bind_repositories!(register);

pub trait Repositories {
    fn groups(&self) -> Arc<super::groupsRepository::GroupsRepository>;
    fn roles(&self) -> Arc<super::rolesRepository::RolesRepository>;
    fn user_bases(&self) -> Arc<super::userBasesRepository::UserBasesRepository>;
    fn user_groups(&self) -> Arc<super::userGroupsRepository::UserGroupsRepository>;
    fn user_roles(&self) -> Arc<super::userRolesRepository::UserRolesRepository>;
}

impl Repositories for Arc<dyn Datasource> {
    fn groups(&self) -> Arc<super::groupsRepository::GroupsRepository> {
        Arc::new(super::groupsRepository::GroupsRepository::new(self.bind_connection(None)).expect("groups"))
    }
    fn roles(&self) -> Arc<super::rolesRepository::RolesRepository> {
        Arc::new(super::rolesRepository::RolesRepository::new(self.bind_connection(None)).expect("roles"))
    }
    fn user_bases(&self) -> Arc<super::userBasesRepository::UserBasesRepository> {
        Arc::new(super::userBasesRepository::UserBasesRepository::new(self.bind_connection(None)).expect("user_bases"))
    }
    fn user_groups(&self) -> Arc<super::userGroupsRepository::UserGroupsRepository> {
        Arc::new(super::userGroupsRepository::UserGroupsRepository::new(self.bind_connection(None)).expect("user_groups"))
    }
    fn user_roles(&self) -> Arc<super::userRolesRepository::UserRolesRepository> {
        Arc::new(super::userRolesRepository::UserRolesRepository::new(self.bind_connection(None)).expect("user_roles"))
    }
}
