import { ApolloError } from "apollo-server-express";

import { genEntityId } from "../../../util";
import { DbPage } from "../../../types/dbTypes";
import {
  MutationInsertBlockIntoPageArgs,
  Resolver,
  Visibility,
} from "../../autoGeneratedTypes";
import { GraphQLContext } from "../../context";

export const insertBlockIntoPage: Resolver<
  Promise<DbPage>,
  {},
  GraphQLContext,
  MutationInsertBlockIntoPageArgs
> = async (
  _,
  {
    componentId,
    entityId,
    entityProperties,
    entityType,
    accountId,
    pageId,
    position,
  },
  { dataSources }
) => {
  // TODO: everything here should be inside a transaction

  const page = await dataSources.db.getEntity({ accountId, entityId: pageId });
  if (!page) {
    throw new ApolloError(
      `Could not find page with pageId ${pageId}`,
      "NOT_FOUND"
    );
  }

  let entity;
  if (entityId) {
    // Update
    entity = await dataSources.db.getEntity({ accountId, entityId });
    if (!entity) {
      throw new ApolloError(`entity ${entityId} not found`, "NOT_FOUND");
    }
  } else if (entityProperties && entityType) {
    // Create new entity
    entity = await dataSources.db.createEntity({
      accountId,
      createdById: genEntityId(), // TODO
      type: entityType,
      properties: entityProperties,
    });
  } else {
    throw new Error(
      `One of entityId OR entityProperties and entityType must be provided`
    );
  }

  const blockProperties = {
    componentId,
    entityType: entity.type,
    entityId: entity.entityId,
    accountId: entity.accountId,
  };

  const newBlock = await dataSources.db.createEntity({
    accountId,
    type: "Block",
    createdById: genEntityId(), // TODO
    properties: blockProperties,
  });

  if (position > page.properties.contents.length) {
    position = page.properties.contents.length;
  }

  page.properties.contents = [
    ...page.properties.contents.slice(0, position),
    {
      type: "Block",
      entityId: newBlock.entityId,
      accountId: newBlock.accountId,
    },
    ...page.properties.contents.slice(position),
  ];
  const updatedEntities = await dataSources.db.updateEntity(page);

  // TODO: for now, all entities are non-versioned, so the list array only have a single
  // element. Return when versioned entities are implemented at the API layer.
  return {
    ...updatedEntities[0],
    id: updatedEntities[0].entityId,
    accountId: updatedEntities[0].accountId,
    visibility: Visibility.Public, // TODO: get from entity metadata
  } as DbPage;
};
