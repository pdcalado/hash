import { Entity } from "../autoGeneratedTypes";

import {
  aggregateEntity,
  createEntity,
  entity,
  entityFields,
  updateEntity,
} from "./entity";
import { blockFields } from "./block";
import { createPage, namespacePages, page, updatePage } from "./pages";
import { createUser } from "./shared/createUser";
import { createOrg } from "./shared/createOrg";
import { namespaces } from "./namespace/namespaces";

import { DbOrg, DbUser } from "../../types/dbTypes";

const KNOWN_ENTITIES = ["Page", "Text", "User"];

export const resolvers = {
  Query: {
    aggregateEntity,
    entity,
    namespacePages,
    namespaces,
    page,
  },

  Mutation: {
    createPage,
    updatePage,
    createEntity,
    updateEntity,
    createUser,
    createOrg,
  },

  // Block: {
  //   namespace: entityNamespaceName,
  // },

  BlockProperties: {
    entity: blockFields.entity,
  },

  UnknownEntity: {
    properties: entityFields.properties,
  },

  Entity: {
    __resolveType(entity: Entity) {
      if (KNOWN_ENTITIES.includes(entity.type)) {
        return entity.type;
      }
      return "UnknownEntity";
    },
  },

  Namespace: {
    __resolveType(entity: DbUser | DbOrg) {
      return entity.type;
    },
  },
};
