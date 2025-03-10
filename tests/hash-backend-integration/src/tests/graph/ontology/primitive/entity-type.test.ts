import { EntityType, TypeSystemInitializer } from "@blockprotocol/type-system";
import {
  ensureSystemGraphIsInitialized,
  ImpureGraphContext,
} from "@hashintel/hash-api/src/graph";
import { User } from "@hashintel/hash-api/src/graph/knowledge/system-types/user";
import { createDataType } from "@hashintel/hash-api/src/graph/ontology/primitive/data-type";
import {
  createEntityType,
  getEntityTypeById,
  updateEntityType,
} from "@hashintel/hash-api/src/graph/ontology/primitive/entity-type";
import { createPropertyType } from "@hashintel/hash-api/src/graph/ontology/primitive/property-type";
import { Logger } from "@hashintel/hash-backend-utils/logger";
import { OwnedById } from "@hashintel/hash-shared/types";
import {
  DataTypeWithMetadata,
  EntityTypeWithMetadata,
  linkEntityTypeUri,
  PropertyTypeWithMetadata,
} from "@hashintel/hash-subgraph";

import { createTestImpureGraphContext, createTestUser } from "../../../util";

jest.setTimeout(60000);

const logger = new Logger({
  mode: "dev",
  level: "debug",
  serviceName: "integration-tests",
});

const graphContext: ImpureGraphContext = createTestImpureGraphContext();

let testUser: User;
let testUser2: User;
let entityTypeSchema: Omit<EntityType, "$id">;
let workerEntityType: EntityTypeWithMetadata;
let textDataType: DataTypeWithMetadata;
let namePropertyType: PropertyTypeWithMetadata;
let favoriteBookPropertyType: PropertyTypeWithMetadata;
let knowsLinkEntityType: EntityTypeWithMetadata;
let previousAddressLinkEntityType: EntityTypeWithMetadata;
let addressEntityType: EntityTypeWithMetadata;

beforeAll(async () => {
  await TypeSystemInitializer.initialize();
  await ensureSystemGraphIsInitialized({ logger, context: graphContext });

  testUser = await createTestUser(graphContext, "entity-type-test-1", logger);
  testUser2 = await createTestUser(graphContext, "entity-type-test-2", logger);

  textDataType = await createDataType(graphContext, {
    ownedById: testUser.accountId as OwnedById,
    schema: {
      kind: "dataType",
      title: "Text",
      type: "string",
    },
    actorId: testUser.accountId,
  });

  await Promise.all([
    createEntityType(graphContext, {
      ownedById: testUser.accountId as OwnedById,
      schema: {
        kind: "entityType",
        title: "Worker",
        type: "object",
        properties: {},
      },
      actorId: testUser.accountId,
    }).then((val) => {
      workerEntityType = val;
    }),
    createEntityType(graphContext, {
      ownedById: testUser.accountId as OwnedById,
      schema: {
        kind: "entityType",
        title: "Address",
        type: "object",
        properties: {},
      },
      actorId: testUser.accountId,
    }).then((val) => {
      addressEntityType = val;
    }),
    createPropertyType(graphContext, {
      ownedById: testUser.accountId as OwnedById,
      schema: {
        kind: "propertyType",
        title: "Favorite Book",
        oneOf: [{ $ref: textDataType.schema.$id }],
      },
      actorId: testUser.accountId,
    }).then((val) => {
      favoriteBookPropertyType = val;
    }),
    createPropertyType(graphContext, {
      ownedById: testUser.accountId as OwnedById,
      schema: {
        kind: "propertyType",
        title: "Name",
        oneOf: [{ $ref: textDataType.schema.$id }],
      },
      actorId: testUser.accountId,
    }).then((val) => {
      namePropertyType = val;
    }),
    createEntityType(graphContext, {
      ownedById: testUser.accountId as OwnedById,
      schema: {
        kind: "entityType",
        title: "Knows",
        description: "Knows of someone",
        type: "object",
        properties: {},
        allOf: [{ $ref: linkEntityTypeUri }],
      },
      actorId: testUser.accountId,
    }).then((val) => {
      knowsLinkEntityType = val;
    }),
    createEntityType(graphContext, {
      ownedById: testUser.accountId as OwnedById,
      schema: {
        kind: "entityType",
        title: "Previous Address",
        description: "A previous address of something.",
        type: "object",
        properties: {},
        allOf: [{ $ref: linkEntityTypeUri }],
      },
      actorId: testUser.accountId,
    }).then((val) => {
      previousAddressLinkEntityType = val;
    }),
  ]);

  entityTypeSchema = {
    kind: "entityType",
    title: "Some",
    type: "object",
    properties: {
      [favoriteBookPropertyType.metadata.editionId.baseId]: {
        $ref: favoriteBookPropertyType.schema.$id,
      },
      [namePropertyType.metadata.editionId.baseId]: {
        $ref: namePropertyType.schema.$id,
      },
    },
    links: {
      [knowsLinkEntityType.schema.$id]: {
        type: "array",
        items: {
          oneOf: [{ $ref: workerEntityType.schema.$id }],
        },
        ordered: false,
      },
      [previousAddressLinkEntityType.schema.$id]: {
        type: "array",
        items: {
          oneOf: [{ $ref: addressEntityType.schema.$id }],
        },
        ordered: true,
      },
    },
  };
});

describe("Entity type CRU", () => {
  let createdEntityType: EntityTypeWithMetadata;

  it("can create an entity type", async () => {
    createdEntityType = await createEntityType(graphContext, {
      ownedById: testUser.accountId as OwnedById,
      schema: entityTypeSchema,
      actorId: testUser.accountId,
    });
  });

  it("can read an entity type", async () => {
    const fetchedEntityType = await getEntityTypeById(graphContext, {
      entityTypeId: createdEntityType.schema.$id,
    });

    expect(fetchedEntityType.schema).toEqual(createdEntityType.schema);
  });

  const updatedTitle = "New text!";

  it("can update an entity type", async () => {
    expect(createdEntityType.metadata.provenance.updatedById).toBe(
      testUser.accountId,
    );

    const updatedEntityType = await updateEntityType(graphContext, {
      entityTypeId: createdEntityType.schema.$id,
      schema: { ...entityTypeSchema, title: updatedTitle },
      actorId: testUser2.accountId,
    }).catch((err) => Promise.reject(err.data));

    expect(updatedEntityType.metadata.provenance.updatedById).toBe(
      testUser2.accountId,
    );
  });
});
