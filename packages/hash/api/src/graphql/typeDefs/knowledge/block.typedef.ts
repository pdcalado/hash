import { gql } from "apollo-server-express";

export const persistedBlockTypedef = gql`
  type PersistedBlock implements PersistedEntity {
    """
    The block's linked data entity.
    """
    dataEntity: PersistedEntity!
    """
    The component id of the block.
    """
    componentId: String!

    # ENTITY INTERFACE FIELDS BEGIN #
    """
    The metadata of the entity.
    """
    metadata: PersistedEntityMetadata!
    """
    The linked entities of the entity.
    """
    linkedEntities: [PersistedEntity!]!
    """
    The JSON object containing the entity's properties.
    """
    properties: JSONObject!
    # ENTITY INTERFACE FIELDS END #
  }

  input LatestPersistedEntityRef {
    entityId: ID!
  }

  extend type Query {
    """
    Get a specified list of blocks by their entity id
    """
    persistedBlocks(blocks: [LatestPersistedEntityRef!]!): [PersistedBlock!]!
  }
`;
