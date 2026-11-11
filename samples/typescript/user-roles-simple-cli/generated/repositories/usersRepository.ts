import { User } from "../types/view/user";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, Schema, type IConnection } from "@deterministic-code/datasources";
import { RolesRepository } from "./rolesRepository";
import { UserRolesRepository } from "./userRolesRepository";
import { GroupsRepository } from "./groupsRepository";
import { UserGroupsRepository } from "./userGroupsRepository";
export class UsersRepository extends CRUDRepository<User> {
  readonly role: RolesRepository;
  readonly userRole: UserRolesRepository;
  readonly group: GroupsRepository;
  readonly userGroup: UserGroupsRepository;
  constructor(connection: IConnection) {
    super(connection, { type: User, entity: "User", entityMapper: new DictionaryMapper([["User", "users"]]), propertyMappings: new DictionaryMapper([["isEnabled", "is_enabled"]]), ids: ["id"], createProperties: ["username", "password", "isEnabled"], updateProperties: ["uuid", "username", "password", "isEnabled"], readProperties: ["id", "uuid", "created", "updated", "username", "password", "isEnabled"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "binary", nullable: false, hasDefault: true }, username: { type: "string", nullable: false, hasDefault: false }, password: { type: "string", nullable: false, hasDefault: false }, isEnabled: { type: "boolean", nullable: false, hasDefault: true } }) });
    this.role = new RolesRepository(connection);
    this.userRole = new UserRolesRepository(connection);
    this.group = new GroupsRepository(connection);
    this.userGroup = new UserGroupsRepository(connection);
    this.addLinked({ destinationProperty: "roles", sourceProperty: "id", relatedKey: "id", relatedProperty: "name", association: { parent: "userId", related: "roleId" }, type: "list" }, this.role, this.userRole);
    this.addLinked({ destinationProperty: "groups", sourceProperty: "id", relatedKey: "id", relatedProperty: "name", association: { parent: "userId", related: "groupId" }, type: "list" }, this.group, this.userGroup);
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<User>(connection, { type: User, entity: "User", entityMapper: new DictionaryMapper([["User", "users"]]), propertyMappings: new DictionaryMapper([["isEnabled", "is_enabled"]]), readProperties: ["id", "uuid", "created", "updated", "username", "password", "isEnabled"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "binary", nullable: false, hasDefault: true }, username: { type: "string", nullable: false, hasDefault: false }, password: { type: "string", nullable: false, hasDefault: false }, isEnabled: { type: "boolean", nullable: false, hasDefault: true } }) });
    const role = new RolesRepository(connection);
    const userRole = new UserRolesRepository(connection);
    const group = new GroupsRepository(connection);
    const userGroup = new UserGroupsRepository(connection);
    repo.addEnriched({ destinationProperty: "roles", sourceProperty: "id", relatedKey: "id", relatedProperty: "name", type: "list" }, role);
    repo.addEnriched({ destinationProperty: "groups", sourceProperty: "id", relatedKey: "id", relatedProperty: "name", type: "list" }, group);
    return repo;
  }
}
