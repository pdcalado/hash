import { genEntityId } from "src/db/adapter";
import { DbOrg } from "../../../types/dbTypes";
import {
  MutationCreateOrgArgs,
  Resolver,
  Visibility,
} from "../../autoGeneratedTypes";
import { GraphQLContext } from "../../context";

export const createOrg: Resolver<
  Promise<DbOrg>,
  {},
  GraphQLContext,
  MutationCreateOrgArgs
> = async (_, { createdById, shortname }, { dataSources }) => {
  const id = genEntityId();

  const entity = await dataSources.db.createEntity({
    namespaceId: id,
    id,
    createdById,
    type: "Org",
    properties: { shortname },
  });

  const org: DbOrg = {
    ...entity,
    type: "Org",
    visibility: Visibility.Public, // TODO
  };

  return org;
};
