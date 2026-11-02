import { RepositoryFactory } from "@deterministic-code/datasources";
import { GroupRepository } from "./groupRepository";
import { RoleRepository } from "./roleRepository";
import { UserRepository } from "./userRepository";
import { UserGroupRepository } from "./userGroupRepository";
import { UserRoleRepository } from "./userRoleRepository";
export { group } from "../types/generated/datasource/group";
export { role } from "../types/generated/datasource/role";
export { user } from "../types/generated/datasource/user";
export { userGroup } from "../types/generated/datasource/userGroup";
export { userRole } from "../types/generated/datasource/userRole";

export const register = (factory: RepositoryFactory): void => {
  factory.register("Group", "groups", (connection) => new GroupRepository(connection), (connection) => GroupRepository.createReadonly(connection));
  factory.register("Role", "roles", (connection) => new RoleRepository(connection), (connection) => RoleRepository.createReadonly(connection));
  factory.register("User", "users", (connection) => new UserRepository(connection), (connection) => UserRepository.createReadonly(connection));
  factory.register("UserGroup", "userGroups", (connection) => new UserGroupRepository(connection), (connection) => UserGroupRepository.createReadonly(connection));
  factory.register("UserRole", "userRoles", (connection) => new UserRoleRepository(connection), (connection) => UserRoleRepository.createReadonly(connection));
};
RepositoryFactory.bind(register);

declare module "@deterministic-code/datasources" {
  interface Datasource {
    readonly groups: GroupRepository;
    readonly roles: RoleRepository;
    readonly users: UserRepository;
    readonly userGroups: UserGroupRepository;
    readonly userRoles: UserRoleRepository;
  }
}
