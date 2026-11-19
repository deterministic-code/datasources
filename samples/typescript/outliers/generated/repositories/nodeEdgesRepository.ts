import { NodeEdge } from "../types/datasource/nodeEdge";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, Schema, type IConnection } from "@deterministic-code/datasources";
export class NodeEdgesRepository extends CRUDRepository<NodeEdge> {
  constructor(connection: IConnection) {
    super(connection, { type: NodeEdge, entity: "NodeEdge", entityMapper: new DictionaryMapper([["NodeEdge", "node_edges"]]), propertyMappings: new DictionaryMapper([["fromNodeId", "from_node_id"], ["toNodeId", "to_node_id"]]), ids: [], createProperties: ["fromNodeId", "toNodeId"], updateProperties: ["fromNodeId", "toNodeId"], readProperties: ["fromNodeId", "toNodeId"], schema: new Schema({ fromNodeId: { type: "integer", nullable: false, hasDefault: false }, toNodeId: { type: "integer", nullable: false, hasDefault: false } }) });
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<NodeEdge>(connection, { type: NodeEdge, entity: "NodeEdge", entityMapper: new DictionaryMapper([["NodeEdge", "node_edges"]]), propertyMappings: new DictionaryMapper([["fromNodeId", "from_node_id"], ["toNodeId", "to_node_id"]]), readProperties: ["fromNodeId", "toNodeId"], schema: new Schema({ fromNodeId: { type: "integer", nullable: false, hasDefault: false }, toNodeId: { type: "integer", nullable: false, hasDefault: false } }) });
}
