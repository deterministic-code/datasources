use datasources::{Datasource, ICRUDRepository, IReadonlyRepository, RepositoryFactory};
use super::super::Address;
use super::super::AddressType;
use super::super::Category;
use super::super::Contact;
use super::super::ContactChangeLog;
use super::super::ContactGroup;
use super::super::ContactGroupMember;
use super::super::ContactPreference;
use super::super::ContactSetting;
use super::super::ContactSource;
use super::super::GameSetting;
use super::super::GameState;
use super::super::Game;
use super::super::GamesCategory;
use super::super::Group;
use super::super::LegacyContact;
use super::super::Phone;
use super::super::PhoneType;
use super::super::Review;
use super::super::Role;
use super::super::User;
use super::super::UserGroup;
use super::super::UserRole;
use super::super::UserSetting;
use std::sync::Arc;

pub fn register(factory: &RepositoryFactory) {
    factory.register_named(
        "Address",
        "addresses",
        |connection| Arc::new(super::addressesRepository::AddressesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Address>>,
        |connection| Arc::new(super::addressesRepository::AddressesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Address>>,
    );
    factory.register_named(
        "AddressType",
        "address_types",
        |connection| Arc::new(super::addressTypesRepository::AddressTypesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<AddressType>>,
        |connection| Arc::new(super::addressTypesRepository::AddressTypesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<AddressType>>,
    );
    factory.register_named(
        "Category",
        "categories",
        |connection| Arc::new(super::categoriesRepository::CategoriesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Category>>,
        |connection| Arc::new(super::categoriesRepository::CategoriesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Category>>,
    );
    factory.register_named(
        "Contact",
        "contacts",
        |connection| Arc::new(super::contactsRepository::ContactsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Contact>>,
        |connection| Arc::new(super::contactsRepository::ContactsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Contact>>,
    );
    factory.register_named(
        "ContactChangeLog",
        "contact_change_logs",
        |connection| Arc::new(super::contactChangeLogsRepository::ContactChangeLogsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactChangeLog>>,
        |connection| Arc::new(super::contactChangeLogsRepository::ContactChangeLogsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactChangeLog>>,
    );
    factory.register_named(
        "ContactGroup",
        "contact_groups",
        |connection| Arc::new(super::contactGroupsRepository::ContactGroupsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactGroup>>,
        |connection| Arc::new(super::contactGroupsRepository::ContactGroupsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactGroup>>,
    );
    factory.register_named(
        "ContactGroupMember",
        "contact_group_members",
        |connection| Arc::new(super::contactGroupMembersRepository::ContactGroupMembersRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactGroupMember>>,
        |connection| Arc::new(super::contactGroupMembersRepository::ContactGroupMembersRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactGroupMember>>,
    );
    factory.register_named(
        "ContactPreference",
        "contact_preferences",
        |connection| Arc::new(super::contactPreferencesRepository::ContactPreferencesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactPreference>>,
        |connection| Arc::new(super::contactPreferencesRepository::ContactPreferencesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactPreference>>,
    );
    factory.register_named(
        "ContactSetting",
        "contact_settings",
        |connection| Arc::new(super::contactSettingsRepository::ContactSettingsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactSetting>>,
        |connection| Arc::new(super::contactSettingsRepository::ContactSettingsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactSetting>>,
    );
    factory.register_named(
        "ContactSource",
        "contact_sources",
        |connection| Arc::new(super::contactSourcesRepository::ContactSourcesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactSource>>,
        |connection| Arc::new(super::contactSourcesRepository::ContactSourcesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactSource>>,
    );
    factory.register_named(
        "GameSetting",
        "game_settings",
        |connection| Arc::new(super::gameSettingsRepository::GameSettingsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<GameSetting>>,
        |connection| Arc::new(super::gameSettingsRepository::GameSettingsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<GameSetting>>,
    );
    factory.register_named(
        "GameState",
        "game_states",
        |connection| Arc::new(super::gameStatesRepository::GameStatesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<GameState>>,
        |connection| Arc::new(super::gameStatesRepository::GameStatesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<GameState>>,
    );
    factory.register_named(
        "Game",
        "games",
        |connection| Arc::new(super::gamesRepository::GamesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Game>>,
        |connection| Arc::new(super::gamesRepository::GamesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Game>>,
    );
    factory.register_named(
        "GamesCategory",
        "games_categories",
        |connection| Arc::new(super::gamesCategoriesRepository::GamesCategoriesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<GamesCategory>>,
        |connection| Arc::new(super::gamesCategoriesRepository::GamesCategoriesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<GamesCategory>>,
    );
    factory.register_named(
        "Group",
        "groups",
        |connection| Arc::new(super::groupsRepository::GroupsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Group>>,
        |connection| Arc::new(super::groupsRepository::GroupsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Group>>,
    );
    factory.register_named(
        "LegacyContact",
        "legacy_contacts",
        |connection| Arc::new(super::legacyContactsRepository::LegacyContactsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<LegacyContact>>,
        |connection| Arc::new(super::legacyContactsRepository::LegacyContactsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<LegacyContact>>,
    );
    factory.register_named(
        "Phone",
        "phones",
        |connection| Arc::new(super::phonesRepository::PhonesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Phone>>,
        |connection| Arc::new(super::phonesRepository::PhonesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Phone>>,
    );
    factory.register_named(
        "PhoneType",
        "phone_types",
        |connection| Arc::new(super::phoneTypesRepository::PhoneTypesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<PhoneType>>,
        |connection| Arc::new(super::phoneTypesRepository::PhoneTypesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<PhoneType>>,
    );
    factory.register_named(
        "Review",
        "reviews",
        |connection| Arc::new(super::reviewsRepository::ReviewsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Review>>,
        |connection| Arc::new(super::reviewsRepository::ReviewsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Review>>,
    );
    factory.register_named(
        "Role",
        "roles",
        |connection| Arc::new(super::rolesRepository::RolesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Role>>,
        |connection| Arc::new(super::rolesRepository::RolesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Role>>,
    );
    factory.register_named(
        "User",
        "users",
        |connection| Arc::new(super::usersRepository::UsersRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<User>>,
        |connection| Arc::new(super::usersRepository::UsersRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<User>>,
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
    factory.register_named(
        "UserSetting",
        "user_settings",
        |connection| Arc::new(super::userSettingsRepository::UserSettingsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<UserSetting>>,
        |connection| Arc::new(super::userSettingsRepository::UserSettingsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<UserSetting>>,
    );
}

datasources::bind_repositories!(register);

pub trait Repositories {
    fn addresses(&self) -> Arc<super::addressesRepository::AddressesRepository>;
    fn address_types(&self) -> Arc<super::addressTypesRepository::AddressTypesRepository>;
    fn categories(&self) -> Arc<super::categoriesRepository::CategoriesRepository>;
    fn contacts(&self) -> Arc<super::contactsRepository::ContactsRepository>;
    fn contact_change_logs(&self) -> Arc<super::contactChangeLogsRepository::ContactChangeLogsRepository>;
    fn contact_groups(&self) -> Arc<super::contactGroupsRepository::ContactGroupsRepository>;
    fn contact_group_members(&self) -> Arc<super::contactGroupMembersRepository::ContactGroupMembersRepository>;
    fn contact_preferences(&self) -> Arc<super::contactPreferencesRepository::ContactPreferencesRepository>;
    fn contact_settings(&self) -> Arc<super::contactSettingsRepository::ContactSettingsRepository>;
    fn contact_sources(&self) -> Arc<super::contactSourcesRepository::ContactSourcesRepository>;
    fn game_settings(&self) -> Arc<super::gameSettingsRepository::GameSettingsRepository>;
    fn game_states(&self) -> Arc<super::gameStatesRepository::GameStatesRepository>;
    fn games(&self) -> Arc<super::gamesRepository::GamesRepository>;
    fn games_categories(&self) -> Arc<super::gamesCategoriesRepository::GamesCategoriesRepository>;
    fn groups(&self) -> Arc<super::groupsRepository::GroupsRepository>;
    fn legacy_contacts(&self) -> Arc<super::legacyContactsRepository::LegacyContactsRepository>;
    fn phones(&self) -> Arc<super::phonesRepository::PhonesRepository>;
    fn phone_types(&self) -> Arc<super::phoneTypesRepository::PhoneTypesRepository>;
    fn reviews(&self) -> Arc<super::reviewsRepository::ReviewsRepository>;
    fn roles(&self) -> Arc<super::rolesRepository::RolesRepository>;
    fn users(&self) -> Arc<super::usersRepository::UsersRepository>;
    fn user_groups(&self) -> Arc<super::userGroupsRepository::UserGroupsRepository>;
    fn user_roles(&self) -> Arc<super::userRolesRepository::UserRolesRepository>;
    fn user_settings(&self) -> Arc<super::userSettingsRepository::UserSettingsRepository>;
}

impl Repositories for Arc<dyn Datasource> {
    fn addresses(&self) -> Arc<super::addressesRepository::AddressesRepository> {
        Arc::new(super::addressesRepository::AddressesRepository::new(self.bind_connection(None)).expect("addresses"))
    }
    fn address_types(&self) -> Arc<super::addressTypesRepository::AddressTypesRepository> {
        Arc::new(super::addressTypesRepository::AddressTypesRepository::new(self.bind_connection(None)).expect("address_types"))
    }
    fn categories(&self) -> Arc<super::categoriesRepository::CategoriesRepository> {
        Arc::new(super::categoriesRepository::CategoriesRepository::new(self.bind_connection(None)).expect("categories"))
    }
    fn contacts(&self) -> Arc<super::contactsRepository::ContactsRepository> {
        Arc::new(super::contactsRepository::ContactsRepository::new(self.bind_connection(None)).expect("contacts"))
    }
    fn contact_change_logs(&self) -> Arc<super::contactChangeLogsRepository::ContactChangeLogsRepository> {
        Arc::new(super::contactChangeLogsRepository::ContactChangeLogsRepository::new(self.bind_connection(None)).expect("contact_change_logs"))
    }
    fn contact_groups(&self) -> Arc<super::contactGroupsRepository::ContactGroupsRepository> {
        Arc::new(super::contactGroupsRepository::ContactGroupsRepository::new(self.bind_connection(None)).expect("contact_groups"))
    }
    fn contact_group_members(&self) -> Arc<super::contactGroupMembersRepository::ContactGroupMembersRepository> {
        Arc::new(super::contactGroupMembersRepository::ContactGroupMembersRepository::new(self.bind_connection(None)).expect("contact_group_members"))
    }
    fn contact_preferences(&self) -> Arc<super::contactPreferencesRepository::ContactPreferencesRepository> {
        Arc::new(super::contactPreferencesRepository::ContactPreferencesRepository::new(self.bind_connection(None)).expect("contact_preferences"))
    }
    fn contact_settings(&self) -> Arc<super::contactSettingsRepository::ContactSettingsRepository> {
        Arc::new(super::contactSettingsRepository::ContactSettingsRepository::new(self.bind_connection(None)).expect("contact_settings"))
    }
    fn contact_sources(&self) -> Arc<super::contactSourcesRepository::ContactSourcesRepository> {
        Arc::new(super::contactSourcesRepository::ContactSourcesRepository::new(self.bind_connection(None)).expect("contact_sources"))
    }
    fn game_settings(&self) -> Arc<super::gameSettingsRepository::GameSettingsRepository> {
        Arc::new(super::gameSettingsRepository::GameSettingsRepository::new(self.bind_connection(None)).expect("game_settings"))
    }
    fn game_states(&self) -> Arc<super::gameStatesRepository::GameStatesRepository> {
        Arc::new(super::gameStatesRepository::GameStatesRepository::new(self.bind_connection(None)).expect("game_states"))
    }
    fn games(&self) -> Arc<super::gamesRepository::GamesRepository> {
        Arc::new(super::gamesRepository::GamesRepository::new(self.bind_connection(None)).expect("games"))
    }
    fn games_categories(&self) -> Arc<super::gamesCategoriesRepository::GamesCategoriesRepository> {
        Arc::new(super::gamesCategoriesRepository::GamesCategoriesRepository::new(self.bind_connection(None)).expect("games_categories"))
    }
    fn groups(&self) -> Arc<super::groupsRepository::GroupsRepository> {
        Arc::new(super::groupsRepository::GroupsRepository::new(self.bind_connection(None)).expect("groups"))
    }
    fn legacy_contacts(&self) -> Arc<super::legacyContactsRepository::LegacyContactsRepository> {
        Arc::new(super::legacyContactsRepository::LegacyContactsRepository::new(self.bind_connection(None)).expect("legacy_contacts"))
    }
    fn phones(&self) -> Arc<super::phonesRepository::PhonesRepository> {
        Arc::new(super::phonesRepository::PhonesRepository::new(self.bind_connection(None)).expect("phones"))
    }
    fn phone_types(&self) -> Arc<super::phoneTypesRepository::PhoneTypesRepository> {
        Arc::new(super::phoneTypesRepository::PhoneTypesRepository::new(self.bind_connection(None)).expect("phone_types"))
    }
    fn reviews(&self) -> Arc<super::reviewsRepository::ReviewsRepository> {
        Arc::new(super::reviewsRepository::ReviewsRepository::new(self.bind_connection(None)).expect("reviews"))
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
    fn user_settings(&self) -> Arc<super::userSettingsRepository::UserSettingsRepository> {
        Arc::new(super::userSettingsRepository::UserSettingsRepository::new(self.bind_connection(None)).expect("user_settings"))
    }
}
