import { RepositoryFactory } from "@deterministic-code/datasources";
import { GroupsRepository } from "./groupsRepository";
import { RolesRepository } from "./rolesRepository";
import { UserBasesRepository } from "./userBasesRepository";
import { UserGroupsRepository } from "./userGroupsRepository";
import { UserRolesRepository } from "./userRolesRepository";
export { group } from "../types/datasource/group";
export { role } from "../types/datasource/role";
export { userBase } from "../types/datasource/userBase";
export { userGroup } from "../types/datasource/userGroup";
export { userRole } from "../types/datasource/userRole";

export const register = (factory: RepositoryFactory): void => {
  factory.register("Group", "groups", (connection) => new GroupsRepository(connection), (connection) => GroupsRepository.createReadonly(connection));
  factory.register("Role", "roles", (connection) => new RolesRepository(connection), (connection) => RolesRepository.createReadonly(connection));
  factory.register("UserBase", "userBases", (connection) => new UserBasesRepository(connection), (connection) => UserBasesRepository.createReadonly(connection));
  factory.register("UserGroup", "userGroups", (connection) => new UserGroupsRepository(connection), (connection) => UserGroupsRepository.createReadonly(connection));
  factory.register("UserRole", "userRoles", (connection) => new UserRolesRepository(connection), (connection) => UserRolesRepository.createReadonly(connection));
};
RepositoryFactory.bind(register);

declare module "@deterministic-code/datasources" {
  interface Datasource {
    readonly groups: GroupsRepository;
    readonly roles: RolesRepository;
    readonly userBases: UserBasesRepository;
    readonly userGroups: UserGroupsRepository;
    readonly userRoles: UserRolesRepository;
  }
}
