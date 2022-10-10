import { ApolloError } from "apollo-server-errors";
import { CommentModel, UserModel } from "../../../../model";
import { ResolverFn } from "../../../apiTypes.gen";
import { LoggedInGraphQLContext } from "../../../context";
import { UnresolvedPersistedCommentGQL } from "../model-mapping";

export const persistedCommentAuthor: ResolverFn<
  Promise<UserModel>,
  UnresolvedPersistedCommentGQL,
  LoggedInGraphQLContext,
  {}
> = async ({ entityId }, _, { dataSources }) => {
  const { graphApi } = dataSources;
  const comment = await CommentModel.getCommentById(graphApi, { entityId });

  if (!comment) {
    throw new ApolloError(
      `Comment with entityId ${entityId} not found`,
      "NOT_FOUND",
    );
  }

  return await comment.getAuthor(graphApi);
};
