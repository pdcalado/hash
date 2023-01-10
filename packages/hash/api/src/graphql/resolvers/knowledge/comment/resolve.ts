import {
  getCommentById,
  resolveComment,
} from "../../../../graph/knowledge/system-types/comment";
import { MutationResolveCommentArgs, ResolverFn } from "../../../api-types.gen";
import { LoggedInGraphQLContext } from "../../../context";
import { dataSourceToImpureGraphContext } from "../../util";
import { mapCommentToGQL, UnresolvedCommentGQL } from "../graphql-mapping";

export const resolveCommentResolver: ResolverFn<
  Promise<UnresolvedCommentGQL>,
  {},
  LoggedInGraphQLContext,
  MutationResolveCommentArgs
> = async (_, { entityId }, { dataSources, user }) => {
  const ctx = dataSourceToImpureGraphContext(dataSources);

  const comment = await getCommentById(ctx, {
    entityId,
  });

  const updatedComment = await resolveComment(ctx, {
    comment,
    actorId: user.accountId,
  });

  return mapCommentToGQL(updatedComment);
};
