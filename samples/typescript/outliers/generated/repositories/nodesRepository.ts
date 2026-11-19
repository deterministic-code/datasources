import { Node } from "../types/datasource/node";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, Schema, type IConnection } from "@deterministic-code/datasources";
export class NodesRepository extends CRUDRepository<Node> {
  constructor(connection: IConnection) {
    super(connection, { type: Node, entity: "Node", entityMapper: new DictionaryMapper([["Node", "nodes"]]), propertyMappings: new DictionaryMapper([]), ids: ["id"], createProperties: ["value"], updateProperties: ["value"], readProperties: ["id", "value"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, value: { type: "string", nullable: false, hasDefault: false } }) });
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<Node>(connection, { type: Node, entity: "Node", entityMapper: new DictionaryMapper([["Node", "nodes"]]), propertyMappings: new DictionaryMapper([]), readProperties: ["id", "value"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, value: { type: "string", nullable: false, hasDefault: false } }) });
}
