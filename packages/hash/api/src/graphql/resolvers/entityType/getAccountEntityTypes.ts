import {
  QueryDeprecatedGetAccountEntityTypesArgs,
  ResolverFn,
} from "../../apiTypes.gen";
import { GraphQLContext } from "../../context";
import EntityType, {
  UnresolvedGQLEntityType,
} from "../../../model/entityType.model";

export const deprecatedGetAccountEntityTypes: ResolverFn<
  Promise<UnresolvedGQLEntityType[]>,
  {},
  GraphQLContext,
  QueryDeprecatedGetAccountEntityTypesArgs
> = async (
  _,
  { accountId, includeAllTypes, includeOtherTypesInUse },
  { dataSources },
) =>
  EntityType.getAccountEntityTypes(dataSources.db, {
    accountId,
    includeAllTypes,
    includeOtherTypesInUse,
  });
