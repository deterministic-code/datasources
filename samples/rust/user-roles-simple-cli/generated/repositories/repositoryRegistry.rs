use datasources::{Datasource, DatasourceError, ICRUDRepository, IReadonlyRepository, RepositoryFactory};
use super::super::Group;
use super::super::Role;
use super::super::User;
use super::super::UserGroup;
use super::super::UserRole;
use std::sync::Arc;

pub fn register(factory: &RepositoryFactory) {
    factory.register_named(
        "Group",
        "groups",
        |connection| Ok(Arc::new(super::groupsRepository::GroupsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Group>>),
        |connection| Arc::new(super::groupsRepository::GroupsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Group>>,
    );
    factory.register_named(
        "Role",
        "roles",
        |connection| Ok(Arc::new(super::rolesRepository::RolesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Role>>),
        |connection| Arc::new(super::rolesRepository::RolesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Role>>,
    );
    factory.register_named(
        "User",
        "users",
        |connection| Ok(Arc::new(super::usersRepository::UsersRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<User>>),
        |connection| Arc::new(super::usersRepository::UsersRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<User>>,
    );
    factory.register_named(
        "UserGroup",
        "user_groups",
        |connection| Ok(Arc::new(super::userGroupsRepository::UserGroupsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<UserGroup>>),
        |connection| Arc::new(super::userGroupsRepository::UserGroupsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<UserGroup>>,
    );
    factory.register_named(
        "UserRole",
        "user_roles",
        |connection| Ok(Arc::new(super::userRolesRepository::UserRolesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<UserRole>>),
        |connection| Arc::new(super::userRolesRepository::UserRolesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<UserRole>>,
    );
}

datasources::bind_repositories!(register);

pub trait Repositories {
    fn groups(&self) -> Arc<super::groupsRepository::GroupsRepository>;
    fn roles(&self) -> Arc<super::rolesRepository::RolesRepository>;
    fn users(&self) -> Arc<super::usersRepository::UsersRepository>;
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
    fn users(&self) -> Arc<super::usersRepository::UsersRepository> {
        Arc::new(super::usersRepository::UsersRepository::new(self.bind_connection(None)).expect("users"))
    }
    fn user_groups(&self) -> Arc<super::userGroupsRepository::UserGroupsRepository> {
        Arc::new(super::userGroupsRepository::UserGroupsRepository::new(self.bind_connection(None)).expect("user_groups"))
    }
    fn user_roles(&self) -> Arc<super::userRolesRepository::UserRolesRepository> {
        Arc::new(super::userRolesRepository::UserRolesRepository::new(self.bind_connection(None)).expect("user_roles"))
    }
}
