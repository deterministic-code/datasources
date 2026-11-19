import { NodeGraph } from "../types/view/nodeGraph";
import { DictionaryMapper, GraphReadonlyEnrichmentStrategy, ReadonlyRepository, Schema, type IConnection } from "@deterministic-code/datasources";
import { NodeEdgesRepository } from "./nodeEdgesRepository";
export class NodeGraphsRepository extends ReadonlyRepository<NodeGraph> {
  readonly nodeEdge: NodeEdgesRepository;
  constructor(connection: IConnection) {
    super(connection, { type: NodeGraph, entity: "NodeGraph", entityMapper: new DictionaryMapper([["NodeGraph", "nodes"]]), propertyMappings: new DictionaryMapper([]), readProperties: ["id", "value"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, value: { type: "string", nullable: false, hasDefault: false } }) });
    this.nodeEdge = new NodeEdgesRepository(connection);
    this.addEnriched({ destinationProperty: "edges", sourceProperty: "id", relatedKey: "id", association: { parent: "fromNodeId", related: "toNodeId" }, type: "list" }, new ReadonlyRepository<NodeGraph>(connection, { type: NodeGraph, entity: "NodeGraph", entityMapper: new DictionaryMapper([["NodeGraph", "nodes"]]), propertyMappings: new DictionaryMapper([]), readProperties: ["id", "value"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, value: { type: "string", nullable: false, hasDefault: false } }) }), new GraphReadonlyEnrichmentStrategy(this.nodeEdge));
  }
  static createReadonly = (connection: IConnection) => new NodeGraphsRepository(connection);
}
