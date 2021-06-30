import {
  AggregateOperationInput,
  AggregationResponse,
  Resolver,
  UnknownEntity,
  Visibility,
} from "../../autoGeneratedTypes";
import { DbUnknownEntity } from "../../../types/dbTypes";
import { aggregateEntity } from "./aggregateEntity";
import { GraphQLContext } from "../../context";

export const isRecord = (thing: unknown): thing is Record<string, any> => {
  if (typeof thing !== "object") {
    return false;
  }
  if (thing == null) {
    return false;
  }
  if (thing instanceof Array) {
    return false;
  }
  return true;
};

// Where a property needs to resolve to another object or objects of a type,
// that property should be expressed as this object under a __linkedData key
// e.g.
// properties: {
//   email: "c@hash.ai",
//   employer: { <-- will be resolved to the data requested in __linkedData
//     __linkedData: {
//       entityType: "Company",
//       entityId: "c1"
//     }
//   }
// },
type LinkedDataDefinition = {
  aggregate?: AggregateOperationInput;
  entityType?: string;
  entityId?: string;
};

// Recursively resolve any __linkedData fields in arbitrary entities
const resolveLinkedData = async (
  ctx: GraphQLContext,
  namespaceId: string,
  object: Record<string, any>
) => {
  if (!isRecord(object.properties)) {
    return;
  }
  for (const [key, value] of Object.entries(object.properties)) {
    // We're only interested in properties which link to other data
    if (!isRecord(value) || !value.__linkedData) {
      continue;
    }

    const { aggregate, entityId, entityType } =
      value.__linkedData as LinkedDataDefinition;

    // We need a type and one of an aggregation operation or id
    if (!entityType || (!aggregate && !entityId)) {
      continue;
    }

    if (entityId) {
      // Fetch a single entity and resolve any linked data in it
      const entity = await ctx.dataSources.db.getEntity({
        namespaceId,
        id: entityId,
      });
      if (!entity) {
        throw new Error(
          `entity ${entityId} in namespace ${namespaceId} not found`
        );
      }
      const e: DbUnknownEntity = {
        ...entity,
        __typename: entityType,
        visibility: Visibility.Public, // TODO
      };
      object.properties[key] = e;
      await resolveLinkedData(ctx, entity.namespaceId, object.properties[key]);
    } else if (aggregate) {
      // Fetch an array of entities
      const { results } = (await (aggregateEntity as any)(null, {
        type: entityType,
        operation: aggregate,
      })) as AggregationResponse;

      object.properties[key] = results;
      // Resolve linked data for each entity in the array
      await Promise.all(
        object.properties[key].map((entity: DbUnknownEntity) => {
          (entity as any).__typename = entityType;
          return resolveLinkedData(ctx, entity.namespaceId, entity);
        })
      );
    }
  }
};

export const properties: Resolver<
  UnknownEntity["properties"],
  DbUnknownEntity,
  GraphQLContext
> = async (entity, _, ctx) => {
  await resolveLinkedData(ctx, entity.namespaceId, entity);
  return entity.properties;
};
