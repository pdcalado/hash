import {
  GraphResolveDepths,
  OntologyTypeEditionId,
} from "@hashintel/hash-graph-client";
import {
  DataTypeWithMetadata,
  EntityTypeWithMetadata,
  Entity,
  PropertyTypeWithMetadata,
} from "./element";
import { Vertices } from "./vertex";
import { Edges } from "./edge";
import { EntityVertexId } from "./identifier";

export type SubgraphRootTypes = {
  dataType: {
    vertexId: OntologyTypeEditionId;
    element: DataTypeWithMetadata;
  };
  propertyType: {
    vertexId: OntologyTypeEditionId;
    element: PropertyTypeWithMetadata;
  };
  entityType: {
    vertexId: OntologyTypeEditionId;
    element: EntityTypeWithMetadata;
  };
  entity: {
    vertexId: EntityVertexId;
    element: Entity;
  };
};

export type SubgraphRootType = SubgraphRootTypes[keyof SubgraphRootTypes];

export type Subgraph<RootType extends SubgraphRootType = SubgraphRootType> = {
  roots: RootType["vertexId"][];
  vertices: Vertices;
  edges: Edges;
  depths: GraphResolveDepths;
};
